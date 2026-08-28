//! The record types and the book registry shared by every `companion` book.
//!
//! # Why this module exists, and what `companion` actually is
//!
//! `companion` is the last of SD-29's kind lanes to get a chassis, and the only
//! one with no corpus-wide precedent when its round opened: every one of its
//! 1,696 corpus units read `companion_content_has_no_engine_table` or
//! `no_compiled_rule_set_for_book`, and the engine's only companion content was
//! two hand-grounded species (`pilot_compute::ground_wolf_companion_stat_block`
//! and `ground_horse_companion_stat_block`) whose values are Rust constants, not
//! corpus reads.
//!
//! `v06_work_inventory::file_kind` types three structurally different `.lst`
//! shapes as `companion`, and conflating them is the first way this lane can go
//! wrong:
//!
//! * **creature** rows — `*_races_companion.lst`, `*_races_familiar.lst`. A
//!   companion or familiar creature. This is the chassis: `SIZE:`/`FACT:BaseSize`,
//!   `MOVE:`, `RACETYPE:`, `MONSTERCLASS:`, natural attacks, `BONUS:STAT`.
//! * **ability** rows — `*_abilities_companion.lst`, `*_abilities_familiar.lst`,
//!   `*_abilities_race_*companion*.lst`. A special quality, special attack or
//!   level-advancement package that reaches a player **only underneath the
//!   creature that owns it**, exactly as `monster_ability` does underneath
//!   `monster` (`docs/release/corpus-work-channels.md §9.2`).
//! * **class** rows — `*_classes_companion.lst`. The PCGen `Companion` /
//!   `Familiar` monster *classes* that a creature row's `MONSTERCLASS:` token
//!   names. Hit-dice progressions, neither creature nor ability; this chassis
//!   does not model them and no registered book carries one.
//!
//! # Only ability rows WITH an owner are registered
//!
//! Same predicate `monster_chassis` states, for the same reason: an ability row
//! no creature row claims is a record that loads and is never shown.
//! `scripts/classify_companion_rows.py` classifies a candidate book's rows before
//! a round commits to it, per `decisions.md §45.1`.
//!
//! Through round 3 this was a per-BOOK predicate — a book with any orphan was
//! held back — because an orphan-free candidate always remained. `bestiary` was
//! the last one, so from round 4 the rule is the monster lane's: **transcribe
//! the linked subset, drop the orphans, and carry them as an `OPEN_FINDINGS`
//! entry naming their remedy** (`decisions.md §50`). The dropped rows keep their
//! honest `engine-does-not-hold` status. What is still absolute is the other half: a
//! book may never SHIP a row nothing can reach, which
//! `every_shipped_ability_row_is_owned_by_a_creature_of_its_own_book` pins.
//!
//! # The five ownership shapes, every one stated by the corpus
//!
//! 1. **row-named** — a creature row's `ABILITY:Special Ability|AUTOMATIC|<name>`
//!    names the ability outright, by `KEY:` or by display name.
//! 2. **prerace** — the ability row's own `PRERACE:1,<Race>` names a creature row
//!    of this book. `monster_ability` has no analogue and every
//!    `TYPE:CompanionAdvancement` row carries it; a chassis that knew only shape
//!    1 would report all 11 of the registered books' advancement rows as orphans.
//! 3. **prefix** — a namespaced `KEY:<Owner> ~ <Leaf>` whose `<Owner>` is a
//!    creature of this book, either verbatim or as the inner name of
//!    `Companion (<Owner>)` / `Familiar (<Owner>)`. `Worg ~ Mastery` owns through
//!    `Companion (Worg)`, which the monster chassis's bare-prefix rule would miss.
//! 4. **granted-by** (`decisions.md §54.1`) — shape 1's own token read on an
//!    ability row that is itself already owned, seeded only from owned rows so an
//!    orphan can never grant reachability to an orphan.
//! 5. **display-name** (`decisions.md §56.1`) — the `<Owner>` of shape 3 is the
//!    creature's `OUTPUTNAME:` rather than its `KEY:`. `KEY:Kyton (Augur)`
//!    displays as `Augur` and its abilities are keyed `Augur ~ …`. Read from the
//!    row's own token, never inferred by unwrapping the key's parentheses:
//!    `Familiar (Fox)` and `Kyton (Augur)` look identical to a string unwrap and
//!    mean different things — one is a wrapper, the other a genus and species.
//!
//! 6. **relay** (`decisions.md §59.1`) — the owner is stated across a corpus row
//!    that is not an inventory unit at all. Bestiary 4's `Familiar (Giant Flea)`
//!    names `Racial Traits ~ Flea (Giant)`, a `CATEGORY:Internal` row of
//!    `b4_abilities_companion.lst`, and THAT row names `Flea (Giant) ~ Disease`.
//!    Shape 4 walks unit-to-unit and cannot see the hop. The first reference is
//!    also read under ANY `ABILITY:<Category>|AUTOMATIC|` category, because the
//!    creature's own token here is `Internal`, not `Special Ability`.
//!
//! Every shape after the third was found by a round that had already committed
//! to a book and READ the rows the classifier was about to throw away, and each
//! one moved the lane's ceiling UP. Corpus-wide the classifier now reports
//! **735** orphan ability rows of the kind's 1,696 units.
//!
//! The ceiling is **923** — and it is the size of the UNION of the exclusions,
//! never the sum: 735 orphans + 2 `PRECAMPAIGN`-gated rows + 7 class rows + 30
//! `.COPY=`/`.MOD` delta rows is 774, but exactly one row is both an orphan and
//! a delta, so 773 distinct rows are excluded (`decisions.md §59.2`). Not
//! 1,696, not the 888 this comment claimed when three shapes were known, and
//! not the 937 it claimed before delta rows were subtracted at all. That is a
//! ceiling, not a backlog.
//!
//! # What a book costs, now that this exists
//!
//! A data module produced by `scripts/transcribe_companion_tables.py`, one
//! [`CompanionBook`] row in [`COMPANION_BOOKS`], and the book's own `RuleSetId`.
//! Every consumer iterates the registry rather than naming books:
//! `v06_work_inventory`'s classifier, `gen_book_cache`'s generator,
//! `companion_catalog`'s wire mapping and `reach_gate`'s claims.
//!
//! # Identity is the `KEY:` token, never the display name
//!
//! `Tinkering` is defined twice in Inner Sea Intrigue alone — once for the
//! Clockwork Familiar and once for the Clockwork Spy — and the rows differ. The
//! keys (`Clockwork Familiar ~ Tinkering`, `Clockwork Spy ~ Tinkering`) are what
//! separate them.

pub use super::monster_chassis::{NaturalAttack, Speed};

/// One `BONUS:STAT|<abbrev>|<amount>` token from a creature or advancement row.
///
/// **An adjustment, never a score.** PCGen computes a companion's actual ability
/// scores at runtime from a base plus these tokens plus the companion class's own
/// level advance; this chassis transcribes the token and does not compute the
/// result, exactly as [`CompanionRecord::monster_class`] carries the hit-dice
/// token without computing hit points. Serving `6` in a column labelled
/// "Strength" would be the quieter lie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatAdjustment {
    /// `"STR"`, `"DEX"`, ... — the corpus abbreviation, verbatim. A token naming
    /// several abilities (`BONUS:STAT|DEX,WIS|4`) is split into one record each,
    /// which is what PCGen itself does with it.
    pub ability: &'static str,
    pub amount: i16,
}

/// One `BONUS:WEAPONPROF=<attack>|DAMAGE|<formula>` token from a creature row —
/// the extra damage that row states for one of its named attacks.
///
/// **The token, verbatim; never a computed number.** Same discipline
/// [`StatAdjustment`] and [`CompanionRecord::monster_class`] already state, and
/// the same one `monster_chassis::MonsterStatBlock::sla_cl_token` states for the
/// monster lane's own derived seam: this chassis transcribes what the row says,
/// and `derived_evaluator_fixture_check::parse_companion_strength_damage`
/// interprets it. The dominant corpus spelling is `max(0,(STR/2))` — PCGen's
/// encoding of PF1's "a creature with only one natural attack adds 1½ × its
/// Strength BONUS to damage" rule (CRB p.182): the base attack already applies
/// the full modifier, this token adds the other half, and `max(0,…)` is why a
/// Strength PENALTY is never multiplied.
///
/// **`attack` is the token's own selector, and it is NOT guaranteed to name one
/// of [`CompanionRecord::natural_attacks`].** Re-derived corpus-wide 2026-08-19
/// over all 927 ingested companion records: `advanced_players_guide:companion:
/// parrot` (`ce_races_familiar_apg.lst:17`) states
/// `BONUS:WEAPONPROF=Claw|DAMAGE|max(0,(STR/2))` while its only natural attack
/// is `Bite`. Carried as the row states it rather than joined-and-dropped —
/// inventing the join would hide a real corpus fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaturalAttackDamageBonus {
    /// The `WEAPONPROF=` selector verbatim: `"Bite"`, `"Claw"`, `"Slam"`, …
    pub attack: &'static str,
    /// The token's trailing formula half, verbatim: `"max(0,(STR/2))"`,
    /// `"STR"`, `"-STR"`, `"5"`, … Never normalised — `max(0,(STR/2))` and
    /// `max(0,STR/2)` are two real corpus spellings and both ship as written.
    pub formula: &'static str,
}

/// One `BONUS:SKILL|<skills>|<formula>` token whose formula half is an
/// ability-score DIFFERENCE expression (`"DEX-STR"`), never a flat
/// `TYPE=Racial` number — those are a different, static quantity this field
/// deliberately does not carry (`scripts/transcribe_companion_tables.py`'s
/// `parse_skill_ability_diff_bonuses` reads only the arithmetic shape).
///
/// **The token, verbatim; never a computed number.** Same discipline
/// [`NaturalAttackDamageBonus`] states: this chassis transcribes what the row
/// says, and `derived_evaluator_fixture_check::parse_companion_skill_ability_diff`
/// interprets it. The single corpus spelling, re-derived 2026-08-19 over
/// every registered book's creature rows (136 occurrences, zero variance):
/// `BONUS:SKILL|Climb,Swim|DEX-STR` — familiars and small companions whose
/// Dexterity typically exceeds their Strength get their Climb and Swim
/// checks computed from the DIFFERENCE between the two modifiers rather than
/// from Strength alone, which is what Climb and Swim otherwise key off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillAbilityDiffBonus {
    /// Every skill the token names, in row order: `["Climb", "Swim"]`.
    pub skills: &'static [&'static str],
    /// The token's trailing formula half, verbatim: `"DEX-STR"`. Never
    /// normalised — a future book stating the terms in the other order or
    /// naming different abilities ships exactly as its row spells it.
    pub formula: &'static str,
}

/// Which modelled facet of `companion` an ability row is, read from the row's
/// `TYPE:` segments.
///
/// Deliberately `Option`al on the record: Inner Sea Intrigue's three
/// `TYPE:ClockworkFamiliarInstalledItem` rows carry no segment this enum models,
/// and dropping them or forcing them into the nearest variant would both be
/// worse than saying so. [`CompanionAbilityRecord::type_segments`] keeps every
/// segment verbatim regardless, so nothing the corpus states is ever lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanionAbilityFacet {
    /// The level-up package a companion gains at a master-level threshold. The
    /// dominant shape: 11 of the 23 registered ability rows.
    CompanionAdvancement,
    SpecialQuality,
    SpecialAttack,
}

impl CompanionAbilityFacet {
    /// The wire/display token, spelled exactly as the corpus `TYPE:` segment.
    pub fn corpus_token(self) -> &'static str {
        match self {
            CompanionAbilityFacet::CompanionAdvancement => "CompanionAdvancement",
            CompanionAbilityFacet::SpecialQuality => "SpecialQuality",
            CompanionAbilityFacet::SpecialAttack => "SpecialAttack",
        }
    }
}

/// How the ability is delivered — the `Supernatural` / `Extraordinary` /
/// `SpellLike` segment of the same `TYPE:` token. `None` when the row does not
/// say. Spelled and read exactly as `monster_chassis::MonsterAbilityDelivery`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanionAbilityDelivery {
    Supernatural,
    Extraordinary,
    SpellLike,
}

impl CompanionAbilityDelivery {
    pub fn corpus_token(self) -> &'static str {
        match self {
            CompanionAbilityDelivery::Supernatural => "Supernatural",
            CompanionAbilityDelivery::Extraordinary => "Extraordinary",
            CompanionAbilityDelivery::SpellLike => "SpellLike",
        }
    }
}

/// One conditional `DESC:` token of an ability row that carries several.
///
/// PCGen rows may state their rules text more than once, each token gated on a
/// different `PRE…` predicate, and the reader is meant to see the text whose
/// gate its character meets. Ultimate Wilderness is the first companion book
/// where this occurs at scale — 22 of its ability rows carry between 2 and 9
/// `DESC:` tokens (`decisions.md §61.1`) — and it is the shape
/// [`parse_desc`](../../../../scripts/transcribe_companion_tables.py) previously
/// refused outright rather than resolve by position.
///
/// **Nothing here is evaluated.** The condition tokens are carried verbatim
/// from the row and rendered into prose on the wire; this chassis has no
/// character to evaluate them against, and picking one variant would be the
/// same lie as picking one by position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompanionDescriptionVariant {
    /// The variant's `DESC:` text, exactly as the row states it.
    pub text: &'static str,
    /// The `%N` argument list belonging to *this* token, not to the row.
    pub variables: &'static [&'static str],
    /// Every `PRE…` entry gating this token, verbatim and in row order.
    /// Empty is a real state: a row carrying one ungated token plus several
    /// gated ones states the ungated one unconditionally.
    pub conditions: &'static [&'static str],
}

/// One `companion` ability record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompanionAbilityRecord {
    /// The corpus `KEY:` token — the identity. Falls back to the display name
    /// only for rows carrying no `KEY:`, which is what PCGen itself does.
    pub key: &'static str,
    pub name: &'static str,
    /// The modelled facet, or `None` for a row whose `TYPE:` states none this
    /// chassis models. See [`CompanionAbilityFacet`].
    pub facet: Option<CompanionAbilityFacet>,
    pub delivery: Option<CompanionAbilityDelivery>,
    /// EVERY `TYPE:` segment of the row, verbatim and in row order — including
    /// the ones `facet` and `delivery` were read from. This is the field that
    /// makes an unmodelled shape visible rather than lost.
    pub type_segments: &'static [&'static str],
    /// The row's `DESC:` text. `None` when the row carries none.
    pub description: Option<&'static str>,
    /// The `DESC:` token's trailing variable list, which is what the `%N`
    /// placeholders in `description` refer to.
    pub description_variables: &'static [&'static str],
    /// Every `DESC:` token of a row that carries SEVERAL under differing
    /// `PRE…` gates, in row order — see [`CompanionDescriptionVariant`].
    ///
    /// Empty for the ordinary single-`DESC:` row, which is why registering this
    /// field changed no already-shipped record's rendered text. When it is
    /// non-empty, [`description`](Self::description) holds the row's single
    /// UNGATED token if it has exactly one and `None` otherwise: a row whose
    /// every token is conditional has no unconditional rules text, and saying
    /// so is the honest state.
    pub description_variants: &'static [CompanionDescriptionVariant],
    /// `BONUS:STAT` tokens the advancement package applies. Adjustments, never
    /// scores — see [`StatAdjustment`].
    pub stat_adjustments: &'static [StatAdjustment],
    pub source_page: Option<&'static str>,
    /// Every creature IN THIS BOOK whose row, `PRERACE:` gate or namespaced
    /// key claims this ability. A row owned only cross-book (see
    /// [`cross_book_owners`](Self::cross_book_owners)) carries this empty —
    /// see that field's own doc for when that is legitimate rather than an
    /// orphan.
    pub owners: &'static [&'static str],
    /// Shape 8, cross-book ownership (`AT-34-E3-001`, `decisions.md §67`):
    /// every `(owner_book, creature_key)` pair naming a creature that owns
    /// this ability but is registered under a DIFFERENT book than this
    /// ability's own. Real for a genuine split the source books themselves
    /// state — Core Rulebook states the Familiar special-ability rules
    /// (Magic chapter) while Bestiary states the familiar creature stat
    /// blocks (Bat, Cat, ...) — never a same-book laziness shortcut: the
    /// invariant test below refuses an entry whose `owner_book` equals this
    /// ability's own book. Empty for every ordinary same-book-owned or
    /// dropped row, which is every row registered before this field existed.
    /// A row with BOTH `owners` and `cross_book_owners` non-empty is legal
    /// (multiple ownership shapes may name the same ability) but does not
    /// occur among currently-registered books.
    pub cross_book_owners: &'static [(&'static str, &'static str)],
    /// The abilities-`.lst` basename this record was read from. Carried per row
    /// because [`source_line`](Self::source_line) is only meaningful together
    /// with its file: Bestiary 3 is the first book whose ability rows come from
    /// TWO files (`b3_abilities_companion.lst` and `b3_abilities_familiar.lst`),
    /// and line 40 means a different row in each. Same discipline as
    /// `MonsterBookSpec::races_lsts`.
    pub source_file: &'static str,
    /// The 1-based line, within [`source_file`](Self::source_file), that this
    /// record was read from.
    pub source_line: u32,
}

/// One companion or familiar creature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompanionRecord {
    pub key: &'static str,
    pub name: &'static str,
    /// From `SIZE:` or, where the row carries none, `FACT:BaseSize|` — both
    /// shapes occur, and 8 of the registered creature rows use only the second.
    pub size: Option<&'static str>,
    pub speeds: &'static [Speed],
    /// The `REACH:` token in feet. `None` when the row carries none.
    pub reach_feet: Option<u32>,
    pub race_type: Option<&'static str>,
    pub race_subtype: Option<&'static str>,
    /// The `MONSTERCLASS:` token verbatim (`"Companion:2"`), which is what hit
    /// dice, AC, hit points and saves are computed from and this ingest
    /// deliberately does not compute — the same discipline
    /// `MonsterStatBlock::monster_class` states.
    pub monster_class: Option<&'static str>,
    /// Every `TYPE:` segment verbatim (`Companion`, `Familiar`, `Construct`).
    /// Empty is a real corpus state: 9 of the registered creature rows carry no
    /// `TYPE:` token at all.
    pub type_segments: &'static [&'static str],
    pub natural_attacks: &'static [NaturalAttack],
    /// Every `BONUS:WEAPONPROF=<attack>|DAMAGE|<formula>` token on the row, in
    /// row order — see [`NaturalAttackDamageBonus`]. Empty for the majority of
    /// rows, which is a real corpus state and not a gap this chassis fills.
    pub natural_attack_damage_bonuses: &'static [NaturalAttackDamageBonus],
    /// `BONUS:SKILL|<skills>|<ability-diff-formula>` tokens on the creature's
    /// row — see [`SkillAbilityDiffBonus`]. Empty for the majority of rows: a
    /// flat `TYPE=Racial` skill bonus is a different, static quantity this
    /// field does not carry.
    pub skill_ability_diff_bonuses: &'static [SkillAbilityDiffBonus],
    /// `BONUS:STAT` tokens on the creature's own row. Adjustments, never scores.
    pub stat_adjustments: &'static [StatAdjustment],
    /// `BONUS:VAR|AC_Natural_Armor|<n>|TYPE=Base`, when the row carries one.
    pub natural_armor: Option<i16>,
    pub source_page: Option<&'static str>,
    /// Keys into this book's `companion_abilities`, in creature-row order.
    pub ability_keys: &'static [&'static str],
    /// Ability names this row cites that this book does not define.
    pub external_ability_refs: &'static [&'static str],
    /// The races-`.lst` basename this record was read from. Carried per row for
    /// the same reason as [`CompanionAbilityRecord::source_file`]: Bestiary 3
    /// draws creature rows from both `b3_races_companion.lst` and
    /// `b3_races_familiar.lst`.
    pub source_file: &'static str,
    /// The 1-based line, within [`source_file`](Self::source_file), that this
    /// record was read from.
    pub source_line: u32,
}

/// One ingested companion book: its corpus directory id and its two tables.
#[derive(Debug, Clone, Copy)]
pub struct CompanionBook {
    /// The corpus directory this book's records file under, which is also the
    /// `engine_book` `v06_work_inventory` joins on and the namespace every
    /// served key carries.
    pub corpus_book: &'static str,
    pub companions: &'static [CompanionRecord],
    pub companion_abilities: &'static [CompanionAbilityRecord],
}

impl CompanionBook {
    /// The creature with this corpus key, if this book defines one.
    pub fn companion_resolve(&self, key: &str) -> Option<&'static CompanionRecord> {
        self.companions.iter().find(|c| c.key == key)
    }

    /// The ability record with this corpus key, if this book defines one.
    pub fn companion_ability_resolve(&self, key: &str) -> Option<&'static CompanionAbilityRecord> {
        self.companion_abilities.iter().find(|a| a.key == key)
    }

    /// The abilities a creature holds, resolved through its own `ability_keys`.
    pub fn abilities_of(&self, companion: &CompanionRecord) -> Vec<&'static CompanionAbilityRecord> {
        companion
            .ability_keys
            .iter()
            .filter_map(|key| self.companion_ability_resolve(key))
            .collect()
    }
}

/// Every book whose `companion` rows this repo has ingested.
///
/// Adding a book here is what makes its records reach the work inventory, the
/// corpus cache, the companion catalog and the reach gate at once — none of
/// those consumers names a book of its own.
///
/// The first eight are every book with **zero** orphan ability rows that this
/// lane reached, derived rather than assumed:
/// `python3 scripts/classify_companion_rows.py inner_sea_combat monster_codex
/// inner_sea_intrigue horror_adventures bestiary_5 bestiary_6 bestiary_2
/// bestiary`.
///
/// Round 2's three (`bestiary_5`, `bestiary_6`, `bestiary_2`) were held back
/// from round 1 because each needs its own `RuleSetId`, whose scope flip moves
/// several hundred units of OTHER kinds from `not-started` to `engine-does-not-hold`.
///
/// **`bestiary` was the last orphan-free book in the corpus** (`decisions.md
/// §54`), so "every registered book ships every row it owns" stopped being true
/// at the ninth. From `bestiary_3` on, a registered book ships the rows its own
/// creature rows reach and carries the rest as an `OPEN_FINDINGS` entry — the
/// monster lane's rule (`decisions.md §50`), which that lane adopted one book
/// earlier for exactly the same reason. Whether a book is registerable is
/// therefore no longer a question about orphans at all; the tests below assert
/// the property that still holds, which is that every SHIPPED ability row has an
/// owner.
pub const COMPANION_BOOKS: &[CompanionBook] = &[
    CompanionBook {
        corpus_book: "inner_sea_combat",
        companions: super::inner_sea_combat::companions_static(),
        companion_abilities: super::inner_sea_combat::companion_abilities_static(),
    },
    CompanionBook {
        corpus_book: "monster_codex",
        companions: super::monster_codex::companions_static(),
        companion_abilities: super::monster_codex::companion_abilities_static(),
    },
    CompanionBook {
        corpus_book: "inner_sea_intrigue",
        companions: super::inner_sea_intrigue::companions_static(),
        companion_abilities: super::inner_sea_intrigue::companion_abilities_static(),
    },
    CompanionBook {
        corpus_book: "horror_adventures",
        companions: super::horror_adventures::companions_static(),
        companion_abilities: super::horror_adventures::companion_abilities_static(),
    },
    CompanionBook {
        corpus_book: "bestiary_5",
        companions: super::bestiary_5::companions_static(),
        companion_abilities: super::bestiary_5::companion_abilities_static(),
    },
    CompanionBook {
        corpus_book: "bestiary_6",
        companions: super::bestiary_6::companions_static(),
        companion_abilities: super::bestiary_6::companion_abilities_static(),
    },
    CompanionBook {
        corpus_book: "bestiary_2",
        companions: super::bestiary_2::companions_static(),
        companion_abilities: super::bestiary_2::companion_abilities_static(),
    },
    // SD-29 Epic 7 round 3. Bestiary 1, and the first registered book whose
    // name is spelled THREE different ways by three different consumers
    // (`decisions.md §54.3`):
    //
    // * `beastiary`   — the `data/corpus/` directory, and therefore the value
    //                   of this field, because every consumer of
    //                   `corpus_book` (the generator's output root,
    //                   `reach_gate::companions_reach`'s denominator) reads a
    //                   corpus directory.
    // * `bestiary`    — the PCGen source directory and `work-inventory.json`'s
    //                   book id.
    // * `beastiary1`  — this Rust module, and the ingest diagnostic's book id.
    //
    // None of the three is renamed here: `§44` records the same split silently
    // under-reporting 108 Bestiary 1 records once already, and the fix for that
    // class of bug is a translation the code performs, not a spelling everyone
    // is asked to remember. `CORPUS_DIR_ALIASES` (work inventory) and
    // `CORPUS_BOOK_IDS` (reach gate) are the two translations, and both already
    // existed before this row.
    //
    // First book needing no new `RuleSetId` — `RuleSetId::Bestiary1` was
    // already compiled for the book's monsters and equipment, so registering
    // its companions moved no other kind's status — and the first admitted by
    // the granted-by ownership shape (`decisions.md §54.1`). Under shapes 1-3
    // alone it reported five orphans and would have been held back exactly as
    // `§51.7` predicted.
    CompanionBook {
        corpus_book: "beastiary",
        companions: super::beastiary1::companions_static(),
        companion_abilities: super::beastiary1::companion_abilities_static(),
    },
    // SD-29 Epic 7 round 4. Bestiary 3 — the first book with TWO source files
    // per shape (`_companion` and `_familiar`), which is what widened
    // `CompanionBookSpec`'s file fields into lists and put a `source_file` on
    // every record above.
    //
    // All 31 creature rows and all 54 ability rows ship. The round was dispatched
    // expecting 19 orphans and expecting to have to build the drop-and-record
    // disposition for them; the disposition was built, and then the 19 turned
    // out not to be orphans. Their creature rows are namespaced by `OUTPUTNAME:`
    // rather than `KEY:` — `KEY:Kyton (Augur)` displays as `Augur`, and its six
    // abilities are keyed `Augur ~ …`. Six of this book's creature rows are
    // shaped that way and they own all 19 rows between them. Reading the token
    // is ownership shape 5 (`decisions.md §56.1`), and it is read from the row,
    // never inferred by unwrapping the key's parentheses — `Familiar (Fox)` and
    // `Kyton (Augur)` look identical to a string unwrap and mean different
    // things.
    CompanionBook {
        corpus_book: "bestiary_3",
        companions: super::bestiary_3::companions_static(),
        companion_abilities: super::bestiary_3::companion_abilities_static(),
    },
    // SD-29 Epic 7 round 5. Bestiary 4 — the book that made ownership shape 6
    // unavoidable (`decisions.md §59.1`). Its `Familiar (Giant Flea)` names
    // `Racial Traits ~ Flea (Giant)`, a `CATEGORY:Internal` row that is not an
    // inventory unit, and THAT row names the two abilities the classifier had
    // been reporting as orphans. `Familiar (Pipefox)` and `Familiar (Ratling)`
    // reach three more the same way.
    //
    // 78 of its 80 units ship — the book's whole reachable remainder, with ZERO
    // orphans. The two exclusions are `.COPY=` delta rows (`§59.2`), the first
    // any registered companion book has carried. No new `RuleSetId`: the monster
    // lane compiled `RuleSetId::B4` in `52da4bc3`.
    CompanionBook {
        corpus_book: "bestiary_4",
        companions: super::bestiary_4::companions_static(),
        companion_abilities: super::bestiary_4::companion_abilities_static(),
    },
    // SD-29 Epic 7 round 6 (`SD29-E7-F2-007`). Ultimate Wilderness — the
    // largest companion block in the corpus, and the first registered book
    // whose shortfall is bigger than its ingest: 327 of 575 rows ship, and the
    // 247 that do not are archetype and option-group rows this chassis is the
    // wrong shape for, not rows it failed to read (`decisions.md §61.2`).
    CompanionBook {
        corpus_book: "ultimate_wilderness",
        companions: super::ultimate_wilderness::companions_static(),
        companion_abilities: super::ultimate_wilderness::companion_abilities_static(),
    },
    // SD-29 Epic 7 round 8 (`SD29-E7-F2-009`). Core Rulebook — the book the
    // lane's transcriber had been REFUSING by name since round 1
    // (`decisions.md §65.1`), and the first registered book whose engine module
    // is spelled differently from its corpus directory in the OTHER direction
    // from Bestiary 1: the corpus says `core_rulebook`, the module says `crb`,
    // and the abbreviation is the older of the two.
    //
    // 156 of its 184 rows ship — 38 creature rows, 118 ability rows — and the
    // 28 that do not are THREE named remainders (`AT-34-E3-001`, `decisions.md
    // §66`): 12 zero-content `Base Companion ~ …` / `Companion ~ …` internal
    // plumbing rows (see below), 2 `cr_classes_companion.lst` rows
    // (`Companion`, `Shadow Companion`, PCGen monster classes — modelling them
    // is a new record type, not a wider predicate on this one), and 14
    // `ce_abilities_familiar_cr.lst` rows reattributed here (the master-side
    // familiar special-ability pool — no familiar CREATURE is registered
    // under this book for them to hang from; familiars are drawn from OTHER
    // books' chassis tables).
    //
    // Through `AT-34-E3-001` this was 84 of the 118 ability rows: the generic
    // `Animal Companion ~ …` / `Animal Companion Feat ~ …` / `Animal Trick ~
    // …` / `Animal Training ~ …` / `Companion Stat ~ …` records, orphaned
    // because the corpus states them exactly ONCE for the whole
    // `CLASS:Companion` chassis every one of this book's 38 creatures shares
    // (`cr_classes_companion.lst`'s single `Companion` class), rather than
    // per-creature. Shape 7, book-wide grant (`scripts/transcribe_
    // companion_tables.py`), attributes each to ALL 38 creatures — a real,
    // corpus-backed fact (PF1's own Animal Companion rules, CRB p.52-55, grant
    // this identical table to every companion regardless of species), not an
    // invented link. 72 of the 84 carry real modelled content and ship; the
    // other 12 are `Base Companion ~ …` / `Companion ~ …` internal PCGen
    // plumbing rows that state only an `ABILITY:` grant token (no `TYPE:`, no
    // `DESC:`, no `BONUS:`) and are dropped by the empty-payload screen like
    // any other book's zero-content row.
    //
    // No new `RuleSetId` — `RuleSetId::Crb` is the oldest in the enum — so
    // registering this family moved no other kind's status.
    CompanionBook {
        corpus_book: "core_rulebook",
        companions: super::crb::companions_static(),
        companion_abilities: super::crb::companion_abilities_static(),
    },
    // SD-29 Epic 7 round 9 (`SD29-E7-F2-010`) — the lane's FINAL PASS, and the
    // four rows below land together because they are one finding, not four
    // books.
    //
    // Between them Ultimate Magic, Advanced Player's Guide and Advanced Race
    // Guide carry 361 orphan ability rows, and every one of them belongs to the
    // summoner's EVOLUTION POOL or the bladebound magus's BLACK BLADE — class
    // features, not creatures. That is the same missing record type round 8
    // named for Core Rulebook's 84 `Animal Companion ~ …` orphans
    // (`decisions.md §65`), seen three more times. Naming it once here is why
    // these three books' shortfall is not three separate `OPEN_FINDINGS`
    // entries.
    //
    // None of the four needed a new `RuleSetId` — `Um`, `Apg`, `Arg` and
    // `Botd1` were all compiled by earlier bundles or earlier lanes — so
    // registering them moved no other kind's status. That is what the closure
    // run 2 receipt meant by "5 books, chassis already registered".
    CompanionBook {
        corpus_book: "ultimate_magic",
        companions: super::ultimate_magic::companions_static(),
        companion_abilities: super::ultimate_magic::companion_abilities_static(),
    },
    CompanionBook {
        corpus_book: "advanced_race_guide",
        companions: super::advanced_race_guide::companions_static(),
        companion_abilities: super::advanced_race_guide::companion_abilities_static(),
    },
    // The corpus book is `advanced_players_guide`; the engine module is `apg`.
    // `MODULE_DIR` in the transcriber carries the mapping — added by round 8 for
    // exactly this row, so no second `rules_tables/advanced_players_guide/`
    // module was created here.
    CompanionBook {
        corpus_book: "advanced_players_guide",
        companions: super::apg::companions_static(),
        companion_abilities: super::apg::companion_abilities_static(),
    },
    // The first book carrying BOTH the monster chassis and this one.
    CompanionBook {
        corpus_book: "book_of_the_damned_volume_1",
        companions: super::book_of_the_damned_volume_1::companions_static(),
        companion_abilities: super::book_of_the_damned_volume_1::companion_abilities_static(),
    },
];

/// The registered book with this corpus directory id.
pub fn companion_book(corpus_book: &str) -> Option<&'static CompanionBook> {
    COMPANION_BOOKS.iter().find(|b| b.corpus_book == corpus_book)
}

/// `AT-34-E3-001` (`companion_absent_from_core_rulebook_companion_tables`
/// mechanism), cycle 4, "grant-token-only" sub-cause named by cycle 3's own
/// atlas defect 3 (`docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/atlas-defects.md`
/// entry 3, 461 of 51,482 corpus-wide). `description: null` internal PCGen
/// dispatch rows whose ENTIRE content is `KEY`, `CATEGORY`, and one-or-more
/// `ABILITY:` grant tokens -- no `TYPE:`, `DESC:`, `BONUS:` -- fanning out to
/// real, already-shipped ability rows of the SAME book.
///
/// **This is a per-record, corpus-wide VERIFIED predicate, never a
/// shape-only reclassification.** Defect 3's own cycle already warned
/// shape alone is unsafe corpus-wide; re-checking that exact concern before
/// building this table confirmed it: applying defect 3's shape query
/// (`ABILITY` present, no `TYPE`/`DESC`/`BONUS`) corpus-wide gives 461
/// matches, and testing "every `ABILITY:` target resolves in-book to a
/// content-bearing record" against all 461 finds only 171 safe, 104 whose
/// target exists but carries no content, and 280 whose target key cannot
/// even be found in-book -- a shape-only rule would silently misclassify
/// 290 of 461 records. What IS verified here, per record, for exactly
/// these 12 `core_rulebook` keys: every `ABILITY:` token's target key names
/// a real `core_rulebook` companion row this engine ALREADY HOLDS (status
/// `grounded`, `text-complete`, or `literal-verified` in the live
/// `docs/work-inventory.json` -- not merely "a corpus file exists with some
/// content"), proven against the live corpus AND the live work-inventory by
/// `grant_token_only_rows_dispatch_to_already_held_content` below. A named,
/// closed list -- never a shape predicate -- so it can only ever match
/// these 12 exact keys, none of the other 449 the corpus-wide shape query
/// also matches.
pub const GRANT_TOKEN_ONLY_DISPATCH_ROWS: &[(&str, &str)] = &[
    (
        "Base Companion ~ Animal Companion",
        "PCGen's own internal dispatch row for the Animal Companion class feature: 11 ABILITY: \
         grant tokens, each routing to a real, already-engine-held Animal Companion ~ * ability \
         row; no DESC/TYPE/BONUS token of its own.",
    ),
    (
        "Base Companion ~ Special Mount",
        "PCGen's own internal dispatch row for the Special Mount class feature: 11 ABILITY: grant \
         tokens, each routing to a real, already-engine-held Animal Companion ~ * ability row; no \
         DESC/TYPE/BONUS token of its own.",
    ),
    (
        "Companion ~ Ability Score Increase",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Ability Score Increase row; no DESC/TYPE/BONUS \
         token of its own.",
    ),
    (
        "Companion ~ Bonus Tricks",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Bonus Tricks row; no DESC/TYPE/BONUS token of its \
         own.",
    ),
    (
        "Companion ~ Devotion",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Devotion row; no DESC/TYPE/BONUS token of its \
         own.",
    ),
    (
        "Companion ~ Evasion",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Evasion row; no DESC/TYPE/BONUS token of its own.",
    ),
    (
        "Companion ~ Improved Evasion",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Improved Evasion row; no DESC/TYPE/BONUS token of \
         its own.",
    ),
    (
        "Companion ~ Link",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Link row; no DESC/TYPE/BONUS token of its own.",
    ),
    (
        "Companion ~ Multiattack",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Multiattack row; no DESC/TYPE/BONUS token of its \
         own.",
    ),
    (
        "Companion ~ Share Spells",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Share Spells row; no DESC/TYPE/BONUS token of its \
         own.",
    ),
    (
        "Companion ~ Spell Resistance (AC)",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Spell Resistance row; no DESC/TYPE/BONUS token of \
         its own.",
    ),
    (
        "Companion ~ Spell Resistance (SM)",
        "PCGen's own internal dispatch row: a single ABILITY: grant token routing to the real, \
         already-engine-held Animal Companion ~ Spell Resistance row; no DESC/TYPE/BONUS token of \
         its own.",
    ),
];

/// Looks up [`GRANT_TOKEN_ONLY_DISPATCH_ROWS`] by key, returning the stated
/// reason when it matches. `v06_work_inventory.rs`'s `Kind::Companion` arm
/// consults this immediately before its final
/// `companion_absent_from_<book>_companion_tables` fallback, mirroring
/// `class_feature_pool_catalog::vacuous_placeholder_reason`'s established
/// named-list pattern (never a live shape scan) for the identical reason.
pub fn grant_token_only_dispatch_reason(key: &str) -> Option<&'static str> {
    GRANT_TOKEN_ONLY_DISPATCH_ROWS.iter().find(|(k, _)| *k == key).map(|(_, reason)| *reason)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// A book registered twice, or a book whose tables were wired to another
    /// book's statics, is a copy-paste defect the registry cannot otherwise see.
    #[test]
    fn every_registered_book_is_distinct_and_non_empty() {
        let mut ids: Vec<_> = COMPANION_BOOKS.iter().map(|b| b.corpus_book).collect();
        let before = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), before, "a corpus book id is registered twice");
        for book in COMPANION_BOOKS {
            assert!(
                !book.companions.is_empty(),
                "{} registers no companion creature; a book with only ability rows has \
                 nothing for them to be shown under",
                book.corpus_book
            );
        }
    }

    /// Keys are namespaced per book on the wire, but a key must still be unique
    /// *within* its book or `companion_resolve` silently returns the first of two.
    #[test]
    fn keys_are_unique_within_every_book() {
        for book in COMPANION_BOOKS {
            let mut keys: Vec<_> = book.companions.iter().map(|c| c.key).collect();
            let before = keys.len();
            keys.sort_unstable();
            keys.dedup();
            assert_eq!(keys.len(), before, "{}: duplicate companion key", book.corpus_book);

            let mut keys: Vec<_> = book.companion_abilities.iter().map(|a| a.key).collect();
            let before = keys.len();
            keys.sort_unstable();
            keys.dedup();
            assert_eq!(keys.len(), before, "{}: duplicate ability key", book.corpus_book);
        }
    }

    /// The chassis link, held closed in both directions for every book: an
    /// ability a creature names is defined here, an ability listed as external is
    /// not, and every defined ability has at least one owner. An orphan means the
    /// catalog would serve a record no creature row reaches.
    #[test]
    fn the_chassis_link_resolves_in_both_directions_for_every_book() {
        for book in COMPANION_BOOKS {
            for companion in book.companions {
                for key in companion.ability_keys {
                    assert!(
                        book.companion_ability_resolve(key).is_some(),
                        "{}: {} names ability {key:?}, which the book does not define",
                        book.corpus_book,
                        companion.name
                    );
                }
                for key in companion.external_ability_refs {
                    assert!(
                        book.companion_ability_resolve(key).is_none(),
                        "{}: {} lists {key:?} as external, but the book defines it",
                        book.corpus_book,
                        companion.name
                    );
                }
            }
            for ability in book.companion_abilities {
                assert!(
                    !ability.owners.is_empty() || !ability.cross_book_owners.is_empty(),
                    "{}: {} ({}) is owned by no creature row (same-book or cross-book) and \
                     would load without ever being shown",
                    book.corpus_book,
                    ability.name,
                    ability.key
                );
                for owner in ability.owners {
                    let companion = book.companion_resolve(owner).unwrap_or_else(|| {
                        panic!("{}: owner {owner:?} is not a creature in this book", book.corpus_book)
                    });
                    assert!(
                        companion.ability_keys.contains(&ability.key),
                        "{}: {} claims owner {owner:?}, which does not name it back",
                        book.corpus_book,
                        ability.key
                    );
                }
                // Shape 8: a cross-book owner must resolve in a DIFFERENT,
                // currently-registered book — never this ability's own book
                // (that would be same-book laziness riding the escape hatch
                // this invariant exists to prevent) and never a fabricated
                // book id or creature key.
                for (owner_book, owner_key) in ability.cross_book_owners {
                    assert_ne!(
                        *owner_book,
                        book.corpus_book,
                        "{}: {} names a cross-book owner in its OWN book — this belongs in \
                         `owners`, not `cross_book_owners`",
                        book.corpus_book,
                        ability.key
                    );
                    let owner_book_entry = companion_book(owner_book).unwrap_or_else(|| {
                        panic!(
                            "{}: {} names cross-book owner_book {owner_book:?}, which is not \
                             a registered companion book",
                            book.corpus_book,
                            ability.key
                        )
                    });
                    assert!(
                        owner_book_entry.companion_resolve(owner_key).is_some(),
                        "{}: {} names cross-book owner {owner_key:?} in {owner_book:?}, which \
                         does not register that creature",
                        book.corpus_book,
                        ability.key
                    );
                }
            }
        }
    }

    /// A transcription that dropped a name would show an empty heading on the
    /// catalog; a `damage_dice` of `""` would show a blank where a die expression
    /// belongs. Neither is representable after this test.
    #[test]
    fn no_record_carries_an_empty_string_where_a_value_is_claimed() {
        for book in COMPANION_BOOKS {
            for companion in book.companions {
                assert!(!companion.key.trim().is_empty());
                assert!(!companion.name.trim().is_empty());
                for attack in companion.natural_attacks {
                    assert!(!attack.name.trim().is_empty());
                    if let Some(dice) = attack.damage_dice {
                        assert!(!dice.trim().is_empty(), "{}: empty damage dice", book.corpus_book);
                    }
                }
                for adjustment in companion.stat_adjustments {
                    assert!(
                        !adjustment.ability.trim().is_empty(),
                        "{}: a stat adjustment names no ability",
                        book.corpus_book
                    );
                }
            }
            for ability in book.companion_abilities {
                assert!(!ability.key.trim().is_empty());
                assert!(!ability.name.trim().is_empty());
                if let Some(desc) = ability.description {
                    assert!(!desc.trim().is_empty(), "{}: empty description", book.corpus_book);
                }
            }
        }
    }

    /// A row whose `TYPE:` states no facet this chassis models must still carry
    /// its segments verbatim, or the record asserts nothing at all about what it
    /// is.
    ///
    /// **Five rows exercise it, in two different shapes** — the count moved 3 →
    /// 5 in round 5 and the shape is what makes the move legitimate rather than
    /// a pin bumped to make a test pass:
    ///
    /// * Inner Sea Intrigue's three `TYPE:ClockworkFamiliarInstalledItem` rows,
    ///   which carry no delivery either.
    /// * Bestiary 4's `Comprehend Languages ~ Constant` and
    ///   `Speak with Animals (Rodents only) ~ Constant`, whose
    ///   `TYPE:Communicate.SpellLike` states a modelled DELIVERY (`SpellLike`)
    ///   and an unmodelled facet (`Communicate`). Its sibling
    ///   `Read Magic ~ Constant` says `TYPE:SpecialQuality.SpellLike` and is
    ///   therefore fully modelled — three adjacent rows of one file splitting
    ///   two ways, which is exactly why `type_segments` keeps everything
    ///   verbatim rather than trusting the enum.
    #[test]
    fn an_ability_with_no_modelled_facet_still_states_its_type_segments() {
        let mut unmodelled = 0;
        for book in COMPANION_BOOKS {
            for ability in book.companion_abilities {
                if ability.facet.is_none() {
                    unmodelled += 1;
                    assert!(
                        !ability.type_segments.is_empty(),
                        "{}: {} has neither a modelled facet nor any TYPE: segment",
                        book.corpus_book,
                        ability.key
                    );
                }
            }
        }
        assert_eq!(
            unmodelled, 93,
            "expected Inner Sea Intrigue's three ClockworkFamiliarInstalledItem rows, \
             Bestiary 4's two `TYPE:Communicate.SpellLike` rows, Ultimate Wilderness's \
             15 `TYPE:SpecialQuaility` rows -- an UPSTREAM TYPO of the modelled \
             `SpecialQuality`, deliberately not corrected into the facet \
             (`decisions.md §61.4`) -- the 11 `ce_*_familiar_*` rows (10 now \
             Bestiary 1's, 1 now Ultimate Magic's, re-attributed by \
             `SD31-CE-COMPANION-001`), Core \
             Rulebook's single `TYPE:NaturalAttack.…` row (round 8, `§65.2`), \
             round 9's three: Advanced Race Guide's two \
             `TYPE:RaceAbility.SpecialAbility` rows and the Advanced Player's \
             Guide's one `TYPE:SkillChoice` row, the four APG evolution-choice \
             rows (`TYPE:EvolutionChoice` x1, `TYPE:TempEvolutionChoice` x3) that \
             the same re-attribution gave an owner for the first time, and \
             `AT-34-E3-001`'s 54 Core Rulebook rows -- the book-wide-granted \
             generic Animal Companion progression table (`decisions.md §66`): \
             31 `TYPE:AnimalCompanionFeat` feat-pool rows, 14 `TYPE:AnimalTrick` \
             trick rows, 6 `TYPE:CompStatChoice` by-level stat rows \
             (`Companion Stat ~ STR/DEX/CON/INT/WIS/CHA`), and 3 \
             `TYPE:CompChoice`/`TYPE:Special` rows (`+2 to Dexterity and \
             Constitution`, `Companion Advancement`, `Companion Skills`) -- \
             none of which is a feat, a special quality, or a special attack \
             the way `CompanionAbilityFacet` models those concepts; \
             a change here means a book's shape moved"
        );
        // Round 9's three, named so the count above cannot be satisfied by a
        // different three. Both shapes are genuinely unmodelled CONCEPTS rather
        // than typos or CATEGORY names:
        //
        // * `RaceAbility.SpecialAbility` — the leading segment is PCGen's
        //   RACE-side ability class. These two rows are the poison and the
        //   double-damage attack of ARG's plant companions, defined on the race
        //   side because a Ghoran's companion IS a plant creature; neither
        //   `SpecialQuality` nor `SpecialAttack` states that.
        // * `SkillChoice` — the Eidolon's `Skills` row, which is a CHOICE the
        //   player makes rather than a quality the creature has. Mapping it onto
        //   any facet variant would claim the eidolon has a special quality it
        //   does not.
        //
        // Both SHIP rather than being dropped, on round 8's distinction
        // (`§65.2`): an unmodelled FACET is not an empty RECORD. All three rows
        // carry a `TYPE:`, and the ARG pair carry `DESC:` prose a player reads.
        for (book_id, key, segments) in [
            ("advanced_race_guide", "Puffball ~ Poison", &["RaceAbility", "SpecialAbility"][..]),
            (
                "advanced_race_guide",
                "Sapling Treant ~ Double Damage",
                &["RaceAbility", "SpecialAbility"][..],
            ),
            ("advanced_players_guide", "Eidolon ~ Skills", &["SkillChoice"][..]),
        ] {
            let book = companion_book(book_id).expect("registered book");
            let ability = book
                .companion_ability_resolve(key)
                .unwrap_or_else(|| panic!("{book_id} does not define {key}"));
            assert!(ability.facet.is_none(), "{key} now states a modelled facet");
            assert_eq!(ability.type_segments, segments, "{key}");
        }
        // The 11 `ce_*_familiar_*` rows, derived rather than inferred from the
        // delta: `python3` over the generated tables, grouping the `facet: None`
        // rows by `type_segments`, reports 10 x ("Special Ability",
        // "Extraordinary") under `beastiary` and 1 x ("Weakness",
        // "Extraordinary") under `ultimate_magic`. Neither leading segment is a
        // `CompanionAbilityFacet` variant -- the enum models
        // `CompanionAdvancement`, `SpecialQuality` and `SpecialAttack` -- and
        // `Special Ability` is PCGen's CATEGORY name appearing in a `TYPE:`
        // token, not a misspelling of one of them. Carried verbatim in
        // `type_segments` for the same reason `SpecialQuaility` is.
        //
        // They used to sit under a `core_essentials` registration.
        // `decisions.md §9` is why they no longer do, and the split is stated by
        // the corpus rather than chosen: `ce_abilities_familiar_race_cr.lst`
        // declares `SOURCELONG:Bestiary` in its own header and
        // `ce_abilities_familiar_race_um.lst` declares `SOURCELONG:Ultimate
        // Magic`. The two counts are pinned separately, so a re-attribution
        // that moved rows between the two books could not be absorbed by the
        // total.
        for (book_id, expected) in [("beastiary", 10), ("ultimate_magic", 1)] {
            let book = companion_book(book_id).expect("registered book");
            let n = book
                .companion_abilities
                .iter()
                .filter(|a| a.facet.is_none() && a.source_file.starts_with("ce_"))
                .count();
            assert_eq!(n, expected, "{book_id}'s re-attributed unmodelled-facet rows");
        }
        // Ultimate Wilderness's typoed segment, pinned by count and by
        // spelling. A successor that decides to model the typo must delete this
        // assertion deliberately rather than discover it as a mystery failure.
        let uw = companion_book("ultimate_wilderness").expect("registered book");
        let typoed: Vec<&str> = uw
            .companion_abilities
            .iter()
            .filter(|a| a.type_segments.first() == Some(&"SpecialQuaility"))
            .map(|a| a.key)
            .collect();
        assert_eq!(typoed.len(), 15, "the corpus's `SpecialQuaility` rows: {typoed:?}");
        for key in &typoed {
            let ability = uw.companion_ability_resolve(key).expect("its own book defines it");
            assert!(
                ability.facet.is_none(),
                "{key}: a misspelled TYPE: segment must not be read as the modelled facet"
            );
        }
        // Named, so the count above can never be satisfied by a different five.
        for (book_id, key) in [
            ("bestiary_4", "Comprehend Languages ~ Constant"),
            ("bestiary_4", "Speak with Animals (Rodents only) ~ Constant"),
        ] {
            let book = companion_book(book_id).expect("registered book");
            let ability = book
                .companion_ability_resolve(key)
                .unwrap_or_else(|| panic!("{book_id} does not define {key}"));
            assert!(ability.facet.is_none(), "{key} now states a modelled facet");
            assert_eq!(ability.type_segments, &["Communicate", "SpellLike"]);
        }
        // Core Rulebook's one row, round 8 (`decisions.md §65.2`) -- the SIXTH
        // unmodelled-facet shape and the first that is neither a category name,
        // a typo, nor a spell-like delivery. `cr_abilities_companion.lst:191`
        // reads `TYPE:NaturalAttack.NaturalAttackSecondary.Secondary`: the row
        // is a natural ATTACK, a shape `CompanionAbilityFacet` does not model
        // at all (it models `CompanionAdvancement`, `SpecialQuality` and
        // `SpecialAttack`).
        //
        // It SHIPS rather than being dropped, and the distinction from `§63.3`'s
        // `Pseudodragon ~ Tail` is the whole point of stating this here: that row
        // was dropped because it carried no `TYPE:`, no `DESC:` and no `BONUS:`,
        // so every modelled field was empty. This one carries a `TYPE:`, four
        // `BONUS:WEAPONPROF=Tail Slap` tokens and `SOURCEPAGE:p.301`. An
        // unmodelled FACET is not an empty RECORD.
        //
        // Asserted structurally and not only by the count above, because round
        // 7 learned at the cost of a gate run that a count assertion placed
        // ahead of a structural one hides the structural one for exactly as
        // long as the count is stale.
        let crb = companion_book("core_rulebook").expect("registered book");
        let crb_unmodelled: Vec<&str> = crb
            .companion_abilities
            .iter()
            .filter(|a| a.facet.is_none())
            .map(|a| a.key)
            .collect();
        // 55 total since `AT-34-E3-001`: round 8's single `Crocodile ~ Tail
        // Slap` plus the 54 book-wide-granted Animal Companion progression
        // rows asserted (by count and type_segments breakdown) above.
        assert_eq!(
            crb_unmodelled.len(),
            55,
            "Core Rulebook's unmodelled-facet rows: {crb_unmodelled:?}"
        );
        assert!(
            crb_unmodelled.contains(&"Crocodile ~ Tail Slap"),
            "round 8's row must still be among them: {crb_unmodelled:?}"
        );
        let tail_slap = crb
            .companion_ability_resolve("Crocodile ~ Tail Slap")
            .expect("its own book defines it");
        assert_eq!(
            tail_slap.type_segments,
            &["NaturalAttack", "NaturalAttackSecondary", "Secondary"]
        );
        assert!(
            tail_slap.source_page.is_some(),
            "a shipped row states where a reader can check it"
        );
    }

    /// Conditional `DESC:` variants (`decisions.md §61.1`), asserted as a
    /// PROPERTY of every registered book plus the two rows that motivated it.
    ///
    /// The property is the one that makes the field trustworthy: a variant list
    /// is either empty or it is the row's WHOLE set of `DESC:` tokens, and
    /// `description` is exactly the single ungated one when there is exactly
    /// one. Getting that backwards would ship a row's text twice or lose half
    /// of it, and no count would notice.
    #[test]
    fn a_row_stating_its_text_once_per_condition_carries_every_token_and_promotes_only_the_ungated_one()
    {
        let mut rows_with_variants = 0;
        for book in COMPANION_BOOKS {
            for ability in book.companion_abilities {
                if ability.description_variants.is_empty() {
                    continue;
                }
                rows_with_variants += 1;
                assert!(
                    ability.description_variants.len() > 1,
                    "{}: {} carries a single 'variant', which is just a description",
                    book.corpus_book,
                    ability.key
                );
                let ungated: Vec<&CompanionDescriptionVariant> = ability
                    .description_variants
                    .iter()
                    .filter(|v| v.conditions.is_empty())
                    .collect();
                match ungated.len() {
                    1 => assert_eq!(
                        ability.description,
                        Some(ungated[0].text),
                        "{}: {} has exactly one ungated token, which must BE the description",
                        book.corpus_book,
                        ability.key
                    ),
                    _ => assert!(
                        ability.description.is_none(),
                        "{}: {} has {} ungated tokens, so no one of them is the row's \
                         unconditional text",
                        book.corpus_book,
                        ability.key,
                        ungated.len()
                    ),
                }
                for variant in ability.description_variants {
                    assert!(
                        !variant.text.trim().is_empty(),
                        "{}: {} carries an empty variant",
                        book.corpus_book,
                        ability.key
                    );
                }
            }
        }
        assert_eq!(
            rows_with_variants, 13,
            "8 from Ultimate Wilderness plus round 9's 3 from Ultimate Magic plus \
             `AT-34-E3-001`'s 2 from Core Rulebook (`Animal Trick ~ Attack`, `Animal \
             Companion Feat ~ Toughness` -- both book-wide-granted, `decisions.md §66`). \
             UW's `.lst` has 22 multi-DESC rows and ships 8, because the other 14 are \
             archetype rows this chassis drops as orphans. The two numbers answering \
             different questions is the point -- a test pinned to 22 would be asserting a \
             fact about a file, not about the table"
        );

        // `AT-34-E3-001`: Core Rulebook's two multi-DESC rows, named individually for
        // the same reason Ultimate Magic's are below. Both shapes carry exactly one
        // UNGATED token plus one gated token, so `description` is `Some` for both --
        // unlike Ultimate Magic's `Giant Slug Companion ~ Acid`, which has none.
        let crb_variants = companion_book("core_rulebook").expect("registered book");
        let attack = crb_variants
            .companion_ability_resolve("Animal Trick ~ Attack")
            .expect("Core Rulebook defines it");
        assert_eq!(attack.description_variants.len(), 2, "Animal Trick ~ Attack");
        assert!(
            attack.description.is_some(),
            "Animal Trick ~ Attack has one ungated DESC: token, which must be promoted"
        );
        assert!(
            attack.description_variants.iter().any(|v| v.conditions.is_empty()),
            "Animal Trick ~ Attack: the base trick description is stated unconditionally"
        );
        let toughness = crb_variants
            .companion_ability_resolve("Animal Companion Feat ~ Toughness")
            .expect("Core Rulebook defines it");
        assert_eq!(toughness.description_variants.len(), 2, "Animal Companion Feat ~ Toughness");
        assert!(
            toughness.description.is_some(),
            "Animal Companion Feat ~ Toughness has one ungated DESC: token, which must be \
             promoted"
        );

        // Round 9: Ultimate Magic is the SECOND book to carry the shape, and
        // the first whose multi-`DESC:` rows are not all shaped the same way.
        // Named individually, because the count above would otherwise be
        // satisfiable by any three rows, and because the split is the finding:
        // two of the three DO have an unconditional token and one does not, so
        // `description` is `Some` for two and `None` for the third — which is
        // exactly the property the loop above pins and the reason it is a
        // property rather than a constant.
        let um = companion_book("ultimate_magic").expect("registered book");
        let um_variants: Vec<(&str, usize, bool)> = um
            .companion_abilities
            .iter()
            .filter(|a| !a.description_variants.is_empty())
            .map(|a| (a.key, a.description_variants.len(), a.description.is_some()))
            .collect();
        assert_eq!(
            um_variants,
            vec![
                ("Giant Leech Companion ~ Blood Drain", 3, true),
                ("Giant Scorpion Companion ~ Poison", 3, true),
                ("Giant Slug Companion ~ Acid", 2, false),
            ],
            "Ultimate Magic's multi-DESC rows"
        );

        let uw = companion_book("ultimate_wilderness").expect("registered book");

        // Every token conditional: the two halves of a poison's DC, gated on
        // whether the companion has advanced. There is no unconditional text.
        let poison = uw
            .companion_ability_resolve("Spitting Cobra ~ Poison")
            .expect("the book defines it");
        assert_eq!(poison.description, None);
        assert_eq!(poison.description_variants.len(), 2);
        assert_eq!(
            poison.description_variants[0].conditions,
            &["PREVARLT:CompanionAdvancement,1"]
        );
        assert_eq!(
            poison.description_variants[1].conditions,
            &["PREVARGTEQ:CompanionAdvancement,1"]
        );
        assert!(
            poison.description_variants[0].text.contains("blurred vision"),
            "the un-advanced companion's spit blurs vision"
        );
        assert!(
            poison.description_variants[1].text.contains("effect blindness"),
            "the advanced companion's spit blinds -- the whole reason both tokens must ship"
        );
        assert_eq!(
            poison.description_variants[0].variables,
            &["10+HD/2+CON"],
            "each token keeps ITS OWN %N argument list, not the row's"
        );

        // All 8 shipped rows are this shape -- every token conditional -- so
        // `description` is `None` for every one of them and a screen reading
        // only `description` would show a Spitting Cobra's poison as having no
        // rules text at all.
        let shipped: Vec<&str> = uw
            .companion_abilities
            .iter()
            .filter(|a| !a.description_variants.is_empty())
            .map(|a| a.key)
            .collect();
        assert_eq!(shipped.len(), 8, "{shipped:?}");
        for key in &shipped {
            let record = uw.companion_ability_resolve(key).expect("its own book defines it");
            assert!(record.description.is_none(), "{key}");
            assert_eq!(record.description_variants.len(), 2, "{key}");
        }
    }

    /// `facet` and `delivery` are reads OF `type_segments`, never a separate
    /// claim: whatever they say must appear verbatim in the segment list. A
    /// transcriber that inferred a facet the row does not state fails here.
    #[test]
    fn every_read_facet_and_delivery_appears_verbatim_in_the_type_segments() {
        for book in COMPANION_BOOKS {
            for ability in book.companion_abilities {
                if let Some(facet) = ability.facet {
                    assert!(
                        ability.type_segments.contains(&facet.corpus_token()),
                        "{}: {} reads facet {} which its TYPE: does not state",
                        book.corpus_book,
                        ability.key,
                        facet.corpus_token()
                    );
                }
                if let Some(delivery) = ability.delivery {
                    assert!(
                        ability.type_segments.contains(&delivery.corpus_token()),
                        "{}: {} reads delivery {} which its TYPE: does not state",
                        book.corpus_book,
                        ability.key,
                        delivery.corpus_token()
                    );
                }
            }
        }
    }

    /// The `Companion (<Species>)` / `Familiar (<Species>)` prefix-ownership
    /// shape, pinned on the row that needs it: Inner Sea Combat's
    /// `Worg ~ Mastery` is owned by `Companion (Worg)`, which the monster
    /// Shape 5, display-name namespacing (`decisions.md §56.1`). Bestiary 3's
    /// `Kyton (Augur)` displays as `Augur` and its abilities are keyed
    /// `Augur ~ …`; the link is the row's own `OUTPUTNAME:` token.
    ///
    /// Pinned by name rather than by count because the failure this guards is a
    /// silent one: if the transcriber stops reading `OUTPUTNAME:`, these rows do
    /// not break, they simply stop being owned and are dropped as orphans, and
    /// the book quietly ships 66 records instead of 85.
    #[test]
    fn a_namespaced_key_owns_through_the_creature_s_display_name() {
        let book = companion_book("bestiary_3").expect("Bestiary 3 is registered");
        for (ability_key, creature_key) in [
            ("Augur ~ Spell-Like Abilities", "Kyton (Augur)"),
            ("Doru ~ Poison", "Div (Doru)"),
            ("Faerie Dragon ~ Breath Weapon", "Dragon (Faerie)"),
            ("Harbinger Archon ~ Blades", "Archon (Harbinger)"),
            ("Raktavarna ~ Change Shape", "Rakshasa (Raktavarna)"),
            ("Spirit Oni ~ Poison", "Oni (Spirit)"),
        ] {
            let ability = book
                .companion_ability_resolve(ability_key)
                .unwrap_or_else(|| panic!("{ability_key} is in this book"));
            assert!(
                ability.owners.contains(&creature_key),
                "{ability_key} should be owned by {creature_key} through its OUTPUTNAME, \
                 but its owners are {:?}",
                ability.owners
            );
            let creature = book
                .companion_resolve(creature_key)
                .unwrap_or_else(|| panic!("{creature_key} is in this book"));
            assert!(
                creature.ability_keys.contains(&ability_key),
                "{creature_key} should reach {ability_key}"
            );
        }
    }

    /// Every SHIPPED ability row has an owner. This is the property that
    /// survived round 4's discovery that "every registered book is orphan-free"
    /// did not: a book may now leave rows behind, but it may never SHIP one that
    /// nothing can reach.
    #[test]
    fn every_shipped_ability_row_is_owned_by_a_creature_of_its_own_book() {
        for book in COMPANION_BOOKS {
            let creature_keys: Vec<&str> = book.companions.iter().map(|c| c.key).collect();
            for ability in book.companion_abilities {
                assert!(
                    !ability.owners.is_empty() || !ability.cross_book_owners.is_empty(),
                    "{}: {} ships with no owner (same-book or cross-book) — it would load \
                     and never be shown",
                    book.corpus_book,
                    ability.key
                );
                for owner in ability.owners {
                    assert!(
                        creature_keys.contains(owner),
                        "{}: {} is owned by {owner}, which is not a creature row of this book",
                        book.corpus_book,
                        ability.key
                    );
                }
            }
        }
    }

    /// A record's `source_line` is only meaningful with its `source_file`, so
    /// every record must name one. Bestiary 3 draws on four files; a record that
    /// named none would make the generator verify a line against whichever file
    /// happened to be first.
    #[test]
    fn every_record_names_the_file_it_was_read_from() {
        for book in COMPANION_BOOKS {
            for companion in book.companions {
                assert!(
                    companion.source_file.ends_with(".lst"),
                    "{}: {} source_file {:?} is not a .lst basename",
                    book.corpus_book,
                    companion.key,
                    companion.source_file
                );
            }
            for ability in book.companion_abilities {
                assert!(
                    ability.source_file.ends_with(".lst"),
                    "{}: {} source_file {:?} is not a .lst basename",
                    book.corpus_book,
                    ability.key,
                    ability.source_file
                );
            }
        }
    }

    /// Bestiary 3 is the first registered book drawing on more than one file per
    /// shape, and all 85 of its corpus units ship.
    #[test]
    fn bestiary_3_ships_all_eighty_five_units_from_four_files() {
        let book = companion_book("bestiary_3").expect("Bestiary 3 is registered");
        assert_eq!(book.companions.len(), 31);
        assert_eq!(book.companion_abilities.len(), 54);

        let mut creature_files: Vec<&str> = book.companions.iter().map(|c| c.source_file).collect();
        creature_files.sort_unstable();
        creature_files.dedup();
        assert_eq!(
            creature_files,
            vec!["b3_races_companion.lst", "b3_races_familiar.lst"]
        );

        let mut ability_files: Vec<&str> =
            book.companion_abilities.iter().map(|a| a.source_file).collect();
        ability_files.sort_unstable();
        ability_files.dedup();
        assert_eq!(
            ability_files,
            vec!["b3_abilities_companion.lst", "b3_abilities_familiar.lst"]
        );
    }

    /// Core Essentials is the first registered book whose CREATURE rows carry
    /// the `.COPY=` delta shape, and none of the 22 ships (`decisions.md
    /// §63.1`).
    ///
    /// `every_registered_creature_states_its_monster_class_token_verbatim`
    /// above is what caught them — `ce_races_familiar_cr.lst:33` reads
    /// `Bat.COPY=Bat (Celestial)` and carries no `MONSTERCLASS:` at all, so
    /// registering the book before the screen was widened turned that test red
    /// on all 22 at once. This test pins the fix from the other side: it names
    /// the excluded keys, so a successor who re-widens the screen has to delete
    /// an assertion deliberately rather than discover the regression as 22
    /// blank creature cards.
    ///
    /// The eleven Core Rulebook familiars each appear twice in that file, once
    /// `(Celestial)` and once `(Fiendish)`, and the BASE row of each pair is
    /// declared in Core Rulebook rather than here — so dropping the deltas
    /// drops no creature this book actually defines.
    #[test]
    fn the_reattributed_familiar_file_ships_no_copy_delta_creature_row() {
        // `ce_races_familiar_cr.lst` / `ce_abilities_familiar_race_cr.lst` both
        // declare `SOURCELONG:Bestiary`, so `decisions.md §9` re-attribution
        // files their rows under Bestiary 1 and there is no longer a
        // `core_essentials` registration to ask (`SD31-CE-COMPANION-001`). The
        // property this test was written for is unchanged and is asserted on
        // the same rows, now reached through their real book.
        let book = companion_book("beastiary").expect("Bestiary 1 is registered");
        let ce_creatures =
            book.companions.iter().filter(|c| c.source_file.starts_with("ce_")).count();
        let ce_abilities = book
            .companion_abilities
            .iter()
            .filter(|a| a.source_file.starts_with("ce_"))
            .count();
        assert_eq!(ce_creatures, 31, "31 declared creature rows ship");
        assert_eq!(ce_abilities, 36, "36 owned ability rows ship");
        assert_eq!(ce_creatures + ce_abilities, 67);

        // `Pseudodragon ~ Tail` is OWNED — `classify_companion_rows.py` is
        // right — and states nothing this chassis models, so `decisions.md
        // §63.3` drops it. Reachability is a fact about ownership;
        // shippability is a fact about the record type.
        assert!(
            book.companion_ability_resolve("Pseudodragon ~ Tail").is_none(),
            "`Pseudodragon ~ Tail` carries no TYPE:, no DESC: and no BONUS: — only an \
             ASPECT: no chassis in this program models. Shipping it puts a card on screen \
             that reads as a name over a page number"
        );

        for species in [
            "Bat", "Cat", "Hawk", "Lizard", "Monkey", "Owl", "Rat", "Raven", "Toad", "Viper",
            "Weasel",
        ] {
            for template in ["Celestial", "Fiendish"] {
                let delta = format!("{species} ({template})");
                assert!(
                    book.companion_resolve(&delta).is_none(),
                    "{delta} is a `.COPY=` row stating a delta on {species}, not a creature; \
                     transcribed verbatim it ships a card with no SIZE, no MOVE and no \
                     MONSTERCLASS"
                );
            }
        }
    }

    /// chassis's bare-prefix rule would have reported as an orphan.
    #[test]
    fn a_namespaced_key_owns_through_the_companion_species_wrapper() {
        let book = companion_book("inner_sea_combat").expect("Inner Sea Combat is registered");
        let mastery = book
            .companion_ability_resolve("Worg ~ Mastery")
            .expect("Worg ~ Mastery is in this book");
        assert_eq!(mastery.owners, &["Companion (Worg)"]);
        let worg = book
            .companion_resolve("Companion (Worg)")
            .expect("Companion (Worg) is in this book");
        assert!(worg.ability_keys.contains(&"Worg ~ Mastery"));
    }

    /// A companion's `MONSTERCLASS:` token is served verbatim and never computed
    /// into hit points, AC or saves — the exact discipline `monster_chassis`
    /// states for the same token. A record that computed them would be inventing
    /// values PCGen derives at runtime.
    #[test]
    fn every_registered_creature_states_its_monster_class_token_verbatim() {
        for book in COMPANION_BOOKS {
            for companion in book.companions {
                let token = companion
                    .monster_class
                    .unwrap_or_else(|| panic!("{}: {} carries no MONSTERCLASS:", book.corpus_book, companion.key));
                assert!(
                    token.contains(':'),
                    "{}: {} MONSTERCLASS token {token:?} is not the corpus's <Class>:<HD> shape",
                    book.corpus_book,
                    companion.key
                );
            }
        }
    }

    /// `AT-34-E2-002`'s eighth table: `companion` (built in SD-29, not rebuilt
    /// by Epic 2) must fail closed exactly like the seven Epic 2 tables in
    /// `simple_kind_tables.rs` -- a fabricated key refuses, it never falls
    /// back to the first companion in the book or any other defaulted entry.
    #[test]
    fn companion_resolve_refuses_a_fabricated_key_it_never_defaults() {
        let book = companion_book("inner_sea_combat").expect("Inner Sea Combat is registered");
        // GREEN half: a present key still resolves to its real record.
        let worg = book
            .companion_resolve("Companion (Worg)")
            .expect("Companion (Worg) is a real record in this book");
        assert_eq!(worg.key, "Companion (Worg)");
        // RED half: a key no corpus record carries must refuse, not silently
        // resolve to `book.companions[0]` or any other stand-in.
        let refusal = book.companion_resolve("___a_key_no_corpus_record_carries___");
        assert!(
            refusal.is_none(),
            "a fabricated key must never resolve to a companion record, real or defaulted"
        );
    }

    /// `AT-34-E3-001`'s `companion_absent_from_core_rulebook_companion_tables`
    /// mechanism (`decisions.md §14`): re-derives, from the live
    /// `docs/work-inventory.json` and the live ingested corpus this module
    /// already reads, WHY each of this cycle's 2-unit remainder is not
    /// owned -- a committed, re-runnable proof rather than a one-off
    /// investigation that decays.
    ///
    /// This cycle closed the 12 zero-content rows (`grant_token_only_rows_
    /// dispatch_to_already_held_content` below proves the closure); only
    /// the 2 `cr_classes_companion.lst` monster-class rows remain, named
    /// here rather than folded into "the rest".
    #[test]
    fn companion_absent_2_sub_causes_are_named_and_sum_exactly() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let inventory_text = std::fs::read_to_string(repo_root.join("docs/work-inventory.json"))
            .expect("docs/work-inventory.json is readable");
        let inventory: Value = serde_json::from_str(&inventory_text)
            .expect("docs/work-inventory.json is valid JSON");
        let units = inventory["units"].as_array().expect("units is an array");
        let mechanism_keys: Vec<String> = units
            .iter()
            .filter(|u| {
                u["book"].as_str() == Some("core_rulebook")
                    && u["status"].as_str() == Some("engine-does-not-hold")
                    && u["evidence"].as_str()
                        == Some("companion_absent_from_core_rulebook_companion_tables")
            })
            .map(|u| u["corpus_key"].as_str().unwrap_or_default().to_string())
            .collect();
        assert_eq!(
            mechanism_keys.len(),
            2,
            "mechanism population drifted from 2 (this cycle closed the 12 zero-content \
             grant-token-only rows via a verified dispatch-target predicate; only the 2 \
             monster-class rows remain)"
        );

        const CLASS_ROWS: [&str; 2] = ["Companion", "Shadow Companion"];

        let companion_dir = repo_root.join("data/corpus/core_rulebook/companion");
        let mut companion_docs: Vec<Value> = Vec::new();
        for entry in std::fs::read_dir(&companion_dir)
            .unwrap_or_else(|e| panic!("{}: {e}", companion_dir.display()))
        {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable json file");
            let doc: Value = serde_json::from_str(&text).expect("valid json");
            companion_docs.push(doc);
        }

        let mut reasons: BTreeMap<&'static str, u32> = BTreeMap::new();
        for key in &mechanism_keys {
            if CLASS_ROWS.contains(&key.as_str()) {
                *reasons.entry("monster_class_definition_not_a_creature_or_ability").or_default() += 1;
            } else {
                panic!("{key}: not accounted for by any named sub-cause -- 2 must equal 2");
            }
        }
        let _ = &companion_docs; // class rows live under cr_classes_companion.lst, not this dir

        for (reason, count) in &reasons {
            eprintln!("AT-34-E3-001 companion_absent sub-cause: {count} | {reason}");
        }
        let total: u32 = reasons.values().sum();
        assert_eq!(total, 2);
        assert_eq!(
            reasons.get("monster_class_definition_not_a_creature_or_ability").copied().unwrap_or(0),
            2
        );
    }

    /// This cycle's own build: proves `GRANT_TOKEN_ONLY_DISPATCH_ROWS`'
    /// own claim, per record, against the live corpus AND the live
    /// `docs/work-inventory.json` -- never merely asserted in a doc
    /// comment. For each of the 12 named keys: (1) the corpus shape is
    /// genuinely zero-content (no `TYPE`/`DESC`/`BONUS` token, `ABILITY:`
    /// present), and (2) EVERY `ABILITY:` token's target key is a real
    /// `core_rulebook` companion row whose live work-inventory status is
    /// already `grounded`, `text-complete`, or `literal-verified` --
    /// i.e. this row's only job is to fan out to content the engine
    /// ALREADY holds, not to a dead pointer or an unheld row. RED if the
    /// corpus ever adds real content to one of these 12 keys, or if any
    /// target's engine status ever regresses out of the held set (exactly
    /// when `decisions.md §2`'s "cleared by revisiting the stated
    /// condition" fires).
    #[test]
    fn grant_token_only_rows_dispatch_to_already_held_content() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let inventory_text = std::fs::read_to_string(repo_root.join("docs/work-inventory.json"))
            .expect("docs/work-inventory.json is readable");
        let inventory: Value = serde_json::from_str(&inventory_text)
            .expect("docs/work-inventory.json is valid JSON");
        let units = inventory["units"].as_array().expect("units is an array");
        let mut status_by_key: BTreeMap<&str, &str> = BTreeMap::new();
        for u in units {
            if u["book"].as_str() == Some("core_rulebook") && u["kind"].as_str() == Some("companion") {
                if let (Some(k), Some(s)) = (u["corpus_key"].as_str(), u["status"].as_str()) {
                    status_by_key.insert(k, s);
                }
            }
        }
        const HELD_STATUSES: [&str; 3] = ["grounded", "text-complete", "literal-verified"];

        let companion_dir = repo_root.join("data/corpus/core_rulebook/companion");
        let mut companion_docs: Vec<Value> = Vec::new();
        for entry in std::fs::read_dir(&companion_dir)
            .unwrap_or_else(|e| panic!("{}: {e}", companion_dir.display()))
        {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable json file");
            let doc: Value = serde_json::from_str(&text).expect("valid json");
            companion_docs.push(doc);
        }
        let find_by_key = |key: &str| -> &Value {
            companion_docs
                .iter()
                .find(|d| d["data"]["key"].as_str() == Some(key))
                .unwrap_or_else(|| panic!("{key}: no corpus record found under {}", companion_dir.display()))
        };

        assert_eq!(GRANT_TOKEN_ONLY_DISPATCH_ROWS.len(), 12);
        for (key, _reason) in GRANT_TOKEN_ONLY_DISPATCH_ROWS {
            let doc = find_by_key(key);
            let raw = doc["data"]["raw_tokens"].as_array().expect("raw_tokens is an array");
            let has_modelled_token = raw
                .iter()
                .any(|t| matches!(t["key"].as_str(), Some("TYPE") | Some("DESC") | Some("BONUS")));
            assert!(
                !has_modelled_token,
                "{key}: expected zero-content (ABILITY grant only), but a modelled token is \
                 present -- this row may now carry real content and no longer belong here"
            );
            let ability_targets: Vec<&str> = raw
                .iter()
                .filter(|t| t["key"].as_str() == Some("ABILITY"))
                .map(|t| {
                    let value = t["value"].as_str().expect("ABILITY token has a string value");
                    // `Companion Class Feature|AUTOMATIC|<target key>|<optional PRE conditions>`
                    value.split('|').nth(2).unwrap_or_else(|| {
                        panic!("{key}: ABILITY token has no target key segment: {value}")
                    })
                })
                .collect();
            assert!(!ability_targets.is_empty(), "{key}: expected at least one ABILITY: token");
            for target in ability_targets {
                let status = status_by_key.get(target).unwrap_or_else(|| {
                    panic!(
                        "{key}: ABILITY: target {target:?} is not a core_rulebook companion unit \
                         in docs/work-inventory.json at all"
                    )
                });
                assert!(
                    HELD_STATUSES.contains(status),
                    "{key}: ABILITY: target {target:?} has status {status:?}, not one of \
                     {HELD_STATUSES:?} -- this dispatch row would be routing to unheld content"
                );
            }
        }
    }

    /// This cycle's own build: the 14 familiar-pool rows two prior cycles
    /// named but declined to close (`AT-34-E3-001_companion_absent_cycle_
    /// receipt.md`, `_2.md`) are now SHIPPED under `core_rulebook`, owned
    /// via Shape 8 cross-book ownership rather than a fabricated same-book
    /// link -- each resolves to one of the 11 familiar creatures PF1's own
    /// Familiars table (CRB p.52-55) shares this ability pool across, all
    /// already registered under `beastiary`.
    #[test]
    fn familiar_ability_pool_closed_via_shape_8_cross_book_ownership() {
        const FAMILIAR_POOL: [&str; 14] = [
            "Familiar Alertness Choice ~ Alertness Active",
            "Familiar Alertness Choice ~ Alertness Inactive",
            "Familiar ~ Alertness",
            "Familiar ~ Deliver Touch Spells",
            "Familiar ~ Empathic Link",
            "Familiar ~ Improved Evasion",
            "Familiar ~ Intelligence Score",
            "Familiar ~ Natural Armor Bonus",
            "Familiar ~ Scry on Familiar",
            "Familiar ~ Share Spells",
            "Familiar ~ Speak One Language",
            "Familiar ~ Speak with Animals of Its Kind",
            "Familiar ~ Speak with Master",
            "Familiar ~ Spell Resistance",
        ];
        const FAMILIAR_CREATURES: [&str; 11] = [
            "Bat", "Cat", "Hawk", "Lizard", "Monkey", "Owl", "Rat", "Raven", "Toad", "Viper",
            "Weasel",
        ];

        let crb = companion_book("core_rulebook").expect("core_rulebook is registered");
        let beastiary = companion_book("beastiary").expect("beastiary is registered");

        for key in FAMILIAR_POOL {
            let ability = crb
                .companion_ability_resolve(key)
                .unwrap_or_else(|| panic!("{key} should now be shipped under core_rulebook"));
            assert!(
                ability.owners.is_empty(),
                "{key}: no same-book owner exists (core_rulebook registers no familiar \
                 creature) -- ownership must be entirely cross-book"
            );
            let owned_creatures: Vec<&str> =
                ability.cross_book_owners.iter().map(|(_, k)| *k).collect();
            for creature in FAMILIAR_CREATURES {
                assert!(
                    owned_creatures.contains(&creature),
                    "{key}: expected cross-book owner {creature:?}, found {owned_creatures:?}"
                );
                assert!(
                    beastiary.companion_resolve(creature).is_some(),
                    "{creature}: must actually be a registered beastiary creature"
                );
            }
            assert_eq!(
                ability.cross_book_owners.len(),
                11,
                "{key}: expected exactly the 11 familiar creatures, found {:?}",
                ability.cross_book_owners
            );
            for (owner_book, _) in ability.cross_book_owners {
                assert_eq!(*owner_book, "beastiary");
            }
        }
    }
}
