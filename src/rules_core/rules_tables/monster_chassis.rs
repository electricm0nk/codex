//! The record types and the book registry shared by every `monster` /
//! `monster_ability` book.
//!
//! # Why this module exists
//!
//! The Bonus Bestiary pilot (SD-29 Epic 5) defined these types inside
//! `rules_tables::bonus_bestiary`, which was correct while exactly one book
//! carried them. `../../../docs/release/corpus-work-channels.md §9.2` rules
//! `monster` the chassis kind and `monster_ability` the features kind attached
//! to it, and that is a property of the *corpus*, not of Bonus Bestiary: every
//! monster-bearing book carries the same two `.lst` shapes. Leaving the types
//! under one book's module would have made the second book import
//! `bonus_bestiary::MonsterStatBlock` to describe Monster Codex rows.
//!
//! `bonus_bestiary` re-exports every type below, so paths written against the
//! pilot still resolve; nothing about its records changed.
//!
//! # What a book costs, now that this exists
//!
//! A data module produced by `scripts/transcribe_monster_tables.py`, one
//! [`MonsterBook`] row in [`MONSTER_BOOKS`], and the book's own `RuleSetId`.
//! Every consumer below iterates the registry rather than naming books:
//! `v06_work_inventory`'s classifier, `gen_book_cache`'s generator,
//! `monster_catalog`'s wire mapping and `reach_gate`'s claims.
//!
//! # Identity is the `KEY:` token, never the display name
//!
//! Every book in the registry carries rows whose `KEY:` differs from the first
//! column — Bonus Bestiary has 6 (`Caryatid Column ~ Immunity to Magic`),
//! Monster Codex has all 3 of its abilities (`Seru ~ Poison`), and both Book of
//! the Damned volumes have all of theirs (`Vermlek ~ Flesh Armor`, whose first
//! column is just `Flesh Armor`). Joining on the display name would merge
//! `Poison` — or `Breath Weapon`, which two registered books now both define —
//! with every other book's rule of that name.
//!
//! # Only ability rows WITH an owner are registered
//!
//! `monster_ability` records reach a player only underneath the monster that
//! owns them, so an ability row no monster row claims is a record that loads and
//! is never shown. `scripts/classify_monster_ability_rows.py` classifies a
//! candidate book's rows before a round commits to it.
//!
//! The first four books here (Bonus Bestiary, Monster Codex, both Book of the
//! Damned volumes) have **zero** orphans, so registering them registered every
//! row. Inner Sea World Guide, added in round 3, is the first that does not:
//! 5 of its 30 ability rows are namespaced to a *template* (`Clockwork ~ …`,
//! `Nascent Demon Lord ~ …`) that no monster row of this book defines. The rule
//! from round 3 on is `kanban.md`'s — **transcribe the linked subset, and carry
//! the orphans as an `OPEN_FINDINGS` entry naming their remedy** — rather than
//! emitting rows that cannot be reached or skipping the book entirely. Those
//! rows stay `not-ingested` in the work inventory, which is their honest status.
//!
//! That predicate is a ceiling on the lane, not a preference — 1,327 of the
//! 4,233 remaining units are orphan ability rows, and 703 of those sit in ten
//! books that carry no monster row at all.

// `StatAdjustment` is `companion_chassis`'s type, reused rather than duplicated
// here (SD31-E6-F1-002): both chassis kinds parse the identical
// `BONUS:STAT|<abbrev-list>|<amount>` PCGen token into the identical shape, and
// `companion_chassis` already reuses THIS module's `NaturalAttack`/`Speed` the
// other direction (`pub use super::monster_chassis::{NaturalAttack, Speed};`) —
// one type per PCGen token shape, not one per consuming module.
pub use super::companion_chassis::StatAdjustment;

/// One movement mode from the row's `MOVE:` token, e.g. `Walk,30,Burrow,10`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Speed {
    pub mode: &'static str,
    pub feet: u32,
}

/// A natural attack named by the monster's row.
///
/// `damage_dice` is `None` when the corpus names the attack but carries no die
/// expression for it. It is never a placeholder string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaturalAttack {
    pub name: &'static str,
    pub damage_dice: Option<&'static str>,
}

/// One spell a monster row grants as a spell-like ability, read from a
/// `SPELLS:<label>|TIMES=<n>|[TIMEUNIT=<unit>|]CASTERLEVEL=<value>|<spell>[,<dc
/// formula>]|<spell>[,<dc formula>]…` token. One record per *spell*, not per
/// token: a single `SPELLS:` token routinely grants several spells that share
/// the token's label, frequency and caster level.
///
/// **Every field is a verbatim substring of the cited row** —
/// `transcribe_monster_tables.py`'s standing contract. Nothing here is
/// computed; the rule application lives in
/// `derived_evaluator_fixture_check::spell_like_ability_save_dc`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterSpellLikeAbility {
    /// The token's first segment (`Innate`, `Neothelid`, …) — PCGen's own
    /// name for the spell-book this grant files under.
    pub label: &'static str,
    /// The `TIMES=` segment's value verbatim (`3`, `ATWILL`), `None` when the
    /// token carries none.
    pub times: Option<&'static str>,
    /// The `TIMEUNIT=` segment's value verbatim (`Week`, `Year`), `None` when
    /// the token carries none — PCGen omits it for the near-universal
    /// per-day default rather than spelling it out.
    pub time_unit: Option<&'static str>,
    /// The `CASTERLEVEL=` segment's value verbatim — a flat literal (`12`) or
    /// a formula (`(max(TL,1))`, `SLA_CL`, `NabasuCasterLevel`). Never
    /// resolved here; `spell_like_ability_caster_level` is the rule
    /// application for the `BONUS:VAR|SLA_CL|` half of the same universal
    /// monster rule.
    pub caster_level_token: Option<&'static str>,
    /// The spell's name as the row spells it, with any trailing `,<dc
    /// formula>` stripped off into [`Self::save_dc_token`]. PCGen's own
    /// parenthesised scope qualifiers are kept (`Invisibility (self only)`) —
    /// they are part of the name the row states.
    pub spell: &'static str,
    /// The trailing `,<dc formula>` half of the spell segment, verbatim and
    /// without the leading comma (`15+CHA`, `11+INT`). `None` when the row
    /// states no save DC for this spell, which is the honest reading of a
    /// spell that allows no save.
    pub save_dc_token: Option<&'static str>,
}

/// Which of `monster_ability`'s facets a record is, read from its corpus
/// `TYPE:` token(s) — the FIRST segment, across every book's row, that names a
/// facet the chassis models (`transcribe_monster_tables.py::parse_type`).
///
/// The five variants below `SpecialQuality` were added by the T9
/// `bestiary`/`bestiary_2`/`bestiary_3`/`inner_sea_bestiary`/`inner_sea_gods`
/// widening cycle (`decisions.md §16`'s own caution against a naive widening
/// applied here): each is a **distinct, repeated, corpus-native** facet label
/// PCGen itself uses in `TYPE:` — never a semantic remapping onto
/// `SpecialAttack`/`SpecialQuality`. `Weakness` (a monster's own
/// vulnerability line), `Defensive` (a passive defensive trait), `Aura` (an
/// area effect centred on the monster), `Sense` (a perception trait) and
/// `Communicate` (a communication-only trait, e.g. `Communicate.Supernatural`
/// — telepathy/truespeech) each occur multiple times across the five books'
/// 876 PI-cleared units, verbatim, the same way `SpecialAttack`/
/// `SpecialQuality` already do. A **bare** delivery-only `TYPE:` (no facet
/// segment at all, e.g. a lone `TYPE:SpellLike`), the `CATEGORY:Internal`
/// shape (this bundle's own finding: 2,371 real / 243 not — a single sample
/// cannot settle which), one-off non-facet strings
/// (`Unfettered Eidolon Stat Selection`, `AsurendraAdditional`, …), and two
/// corpus typos (`Spelllike`, `SpecialAttck`) are deliberately **not**
/// modelled here — each needs its own per-record read, not a vocabulary
/// entry guessed from one sample (`t9-onboarding` cycle receipt, "What
/// remains").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterAbilityFacet {
    SpecialAttack,
    SpecialQuality,
    /// `TYPE:Weakness.Extraordinary` etc — a stated vulnerability or
    /// drawback line (`Akata ~ Deaf`, `Bodak ~ Vulnerability to Sunlight`).
    Weakness,
    /// `TYPE:Defensive.Extraordinary` etc — a passive defensive trait that
    /// PCGen's own vocabulary distinguishes from `SpecialQuality`
    /// (`Chaos Beast ~ Resistant to Transformation`).
    Defensive,
    /// `TYPE:Aura.Supernatural` etc — an area effect centred on the monster
    /// (`Quickwood ~ Fear Aura`, `Winterwight ~ Aura of Cold`).
    Aura,
    /// `TYPE:Sense.Supernatural` etc — a perception trait
    /// (`Dragon Horse ~ Know Alignment`, `Banshee ~ Hear Heartbeat`).
    Sense,
    /// `TYPE:Communicate.Supernatural` etc — a communication-only trait
    /// (`Orsheval ~ Truespeech`).
    Communicate,
}

impl MonsterAbilityFacet {
    /// The wire/display token, spelled exactly as the corpus `TYPE:` segment.
    pub fn corpus_token(self) -> &'static str {
        match self {
            MonsterAbilityFacet::SpecialAttack => "SpecialAttack",
            MonsterAbilityFacet::SpecialQuality => "SpecialQuality",
            MonsterAbilityFacet::Weakness => "Weakness",
            MonsterAbilityFacet::Defensive => "Defensive",
            MonsterAbilityFacet::Aura => "Aura",
            MonsterAbilityFacet::Sense => "Sense",
            MonsterAbilityFacet::Communicate => "Communicate",
        }
    }
}

/// How the ability is delivered — the `Supernatural` / `Extraordinary` /
/// `SpellLike` segment of the same `TYPE:` token. `None` when the row does not
/// say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterAbilityDelivery {
    Supernatural,
    Extraordinary,
    SpellLike,
}

impl MonsterAbilityDelivery {
    pub fn corpus_token(self) -> &'static str {
        match self {
            MonsterAbilityDelivery::Supernatural => "Supernatural",
            MonsterAbilityDelivery::Extraordinary => "Extraordinary",
            MonsterAbilityDelivery::SpellLike => "SpellLike",
        }
    }
}

/// One `monster_ability` record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterAbilityRecord {
    /// The corpus `KEY:` token — the identity. Falls back to the display name
    /// only for rows that carry no `KEY:`, which is what PCGen itself does.
    pub key: &'static str,
    pub name: &'static str,
    pub facet: MonsterAbilityFacet,
    pub delivery: Option<MonsterAbilityDelivery>,
    /// Remaining `TYPE:` segments that are neither facet nor delivery
    /// (`Aura`, `Immunity`), kept verbatim.
    pub traits: &'static [&'static str],
    /// The row's `DESC:` text. `None` when the row carries none.
    pub description: Option<&'static str>,
    /// The `DESC:` token's trailing variable list, which is what the `%1`
    /// placeholders in `description` refer to.
    pub description_variables: &'static [&'static str],
    pub source_page: Option<&'static str>,
    /// Every monster in this book whose row (or whose namespace, for a
    /// `<Monster> ~ <Ability>` key) claims this ability.
    pub owners: &'static [&'static str],
    /// The abilities-`.lst` file this record was read from, as a bare file
    /// name relative to the book directory.
    ///
    /// The exact counterpart of [`MonsterStatBlock::source_file`], and added
    /// for the same reason one book later: a book is not guaranteed one
    /// abilities file either. Inner Sea Gods splits its 161 ability rows 145/16
    /// across `isg_abilities_races.lst` and `support/isg_abilities_races_b4.lst`
    /// — so `source_line` alone does not identify a row, and the generator that
    /// re-reads the cited line to verify it must be told which file to open.
    ///
    /// Until this field existed the generator took the abilities file from a
    /// single per-book spec string (`MonsterBookSpec::abilities_lst`), which is
    /// correct only for a one-file book; the nine books registered before it
    /// were all one-file, so the singular spelling was right by coincidence
    /// rather than by rule.
    pub source_file: &'static str,
    /// The 1-based line of [`Self::source_file`] this record was read from.
    pub source_line: u32,
    /// `true` when [`Self::key`]/[`Self::name`] are a Codex-generated
    /// neutral identity rather than the printed name (`decisions.md §24`):
    /// the row's own name matched a Product Identity term, so it ships
    /// de-identified instead of being dropped. `false` for every
    /// ordinarily-named record.
    pub codex_generated_name: bool,
    /// `Some("name_pi_blocked")` exactly when [`Self::codex_generated_name`]
    /// is `true`. `§24b`-4: the divergence record stops at the coordinate
    /// (`Self::source_file`/`Self::source_line`) -- never the original
    /// string, which is why there is no field here that could carry it.
    pub rename_reason: Option<&'static str>,
    /// `Some("<book>:<file>:<line>")` alongside [`Self::rename_reason`] --
    /// the exact citation the rename applies to.
    pub rename_coordinate: Option<&'static str>,
}

/// One monster stat block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterStatBlock {
    pub key: &'static str,
    pub name: &'static str,
    pub size: Option<&'static str>,
    pub speeds: &'static [Speed],
    pub race_type: Option<&'static str>,
    pub race_subtype: Option<&'static str>,
    /// The `CR:` token verbatim (`"3"`, `"1"`), not a parsed number — the
    /// corpus spells fractional CRs as `1/2`.
    pub challenge_rating: Option<&'static str>,
    /// The `MONSTERCLASS:` token (`"Undead:4"`), which is what AC/HP/saves are
    /// computed from and this ingest deliberately does not compute.
    pub monster_class: Option<&'static str>,
    pub source_page: Option<&'static str>,
    pub natural_attacks: &'static [NaturalAttack],
    /// Every `BONUS:STAT|<abbrev-list>|<amount>` token on the row, one record
    /// per ability (`companion_chassis::StatAdjustment`, reused rather than
    /// duplicated — the two chassis kinds parse the identical PCGen token).
    ///
    /// **An adjustment, never a score** (SD31-E6-F1-002, `OPEN-ISSUES.md` row
    /// 26). PCGen computes a monster's actual ability scores at runtime from a
    /// base template plus these tokens plus whatever the row's other `BONUS:`
    /// fields add, and this ingest does not compute the result — exactly as
    /// [`Self::monster_class`] carries the hit-dice token without computing hit
    /// points. A token whose amount is not a literal integer (a formula, e.g.
    /// `BONUS:STAT|STR|MutagenicMaulerMutagenStatBonus`) is **skipped**, not
    /// guessed: there is no formula interpreter here, and a wrong number in an
    /// ability column is worse than an absent one.
    pub stat_adjustments: &'static [StatAdjustment],
    /// Whether the row carries a `BONUS:VAR|SLA_CL|<...>` token — PCGen's
    /// encoding of PF1's Spell-Like Abilities universal monster rule (caster
    /// level = Hit Dice, or an arithmetic wrapper of it).
    ///
    /// **Not** the more general `SPELLS:` token, deliberately: Linnorm
    /// (Crag) (`b1_races.lst:269`) carries `BONUS:VAR|SLA_CL|HD` and its
    /// spell-like effects (`True Seeing ~ Constant`) reach the row only
    /// through an `ABILITY:` cross-reference, with no `SPELLS:` token
    /// anywhere on the line — gating on `SPELLS:` answered `false` for one
    /// of this seam's own already-committed fixtures (TDD red/green anchor,
    /// `run_monster_bar_check_clears_every_committed_monster_fixture`).
    ///
    /// A presence check only — never a count or a list of the spells
    /// themselves, which this ingest does not capture. Exists so a consumer
    /// can tell a monster with no spell-like abilities at all from one whose
    /// `SLA_CL` token simply was not parsed into anything else on this
    /// struct, which is the same "absence must be honest" reasoning
    /// `external_ability_refs` already carries for named abilities this book
    /// does not define. `spell_like_ability_caster_level` in
    /// `derived_evaluator_fixture_check` reads this field before applying PF1's
    /// Spell-Like Abilities universal monster rule, so a monster with no
    /// spell-like abilities at all is never served a caster level it has no
    /// use for (SD31-E6-F1-002, `OPEN-ISSUES.md` row 44).
    pub has_spell_like_abilities: bool,
    /// The row's `BONUS:VAR|SLA_CL|<value>` trailing value, verbatim, when
    /// [`Self::has_spell_like_abilities`] is `true`; `None` otherwise.
    ///
    /// **Load-bearing, not decorative** (SD31-E6-F9-003, `OPEN-ISSUES.md` row
    /// 44's own follow-on, already forecast in `progress.md`'s
    /// `SD31-E6-F11-002` receipt as "a follow-on within THIS seam"). PF1's
    /// Universal Monster Rule states a spell-like ability's caster level
    /// "equal[s] its Hit Dice" **unless otherwise noted**, and the corpus
    /// routinely notes otherwise: `BONUS:VAR|SLA_CL|HD` and
    /// `BONUS:VAR|SLA_CL|max(TL,1)` (PCGen's own two equivalent spellings of
    /// "apply the generic rule") coexist, record by record, with a bare
    /// literal override — Couatl (`b1_races.lst:74`) carries
    /// `BONUS:VAR|SLA_CL|9` while its own `MONSTERCLASS:Couatl Outsider:12`
    /// states 12 Hit Dice; Demon (Glabrezu) (`b1_races.lst:95`) carries
    /// `BONUS:VAR|SLA_CL|14` against 12 HD. Neither is a defect — both are
    /// the corpus correctly stating the printed stat block's actual SLA
    /// caster level, which sometimes differs from HD exactly as the rule's
    /// own "unless otherwise noted" clause anticipates. Before this field
    /// existed, `spell_like_ability_caster_level` had no way to see the
    /// override and always applied the generic HD rule, which is silently
    /// WRONG for every monster whose row carries one (re-derived corpus-wide
    /// this cycle: the majority of the registry's own already-shipped,
    /// `has_spell_like_abilities`-true monsters carry an override, not the
    /// bare `HD`/`max(TL,1)` spelling — verified sampling every derived-
    /// grounded `monster` unit's own corpus row, not assumed from the shape
    /// of the 7 already-committed fixtures, all of which happen to be the
    /// bare-`HD` case and so never surfaced the gap).
    pub sla_cl_token: Option<&'static str>,
    /// Every spell this row grants as a spell-like ability, one record per
    /// spell, read from the row's `SPELLS:` tokens
    /// ([`MonsterSpellLikeAbility`]).
    ///
    /// Distinct from [`Self::has_spell_like_abilities`]/[`Self::sla_cl_token`],
    /// which are about the row's `BONUS:VAR|SLA_CL|` token — the two encode
    /// different halves of PF1's Spell-Like Abilities universal monster rule
    /// and a row may carry either without the other. Linnorm (Crag)
    /// (`b1_races.lst:269`) carries `BONUS:VAR|SLA_CL|HD` and **no** `SPELLS:`
    /// token at all; Aboleth (`b1_races.lst:7`) carries `SPELLS:` grants and
    /// **no** `BONUS:VAR|SLA_CL|` token. Neither field may be derived from the
    /// other.
    pub spell_like_abilities: &'static [MonsterSpellLikeAbility],
    /// Keys into this book's `monster_abilities`, in row order.
    pub ability_keys: &'static [&'static str],
    /// Ability names this row cites that this book does not define.
    pub external_ability_refs: &'static [&'static str],
    /// The races-`.lst` file this record was read from, relative to the book
    /// directory.
    ///
    /// A book is not guaranteed one monster file. Inner Sea World Guide splits
    /// its 14 monsters 7/7 across `iswg_races.lst` and `iswg_races_bestiary.lst`
    /// — so `source_line` alone does not identify a row, and the generator that
    /// re-reads the cited line to verify it must be told which file to open.
    /// Before this field existed the generator took the file from a single
    /// per-book spec string, which is correct only for a one-file book.
    pub source_file: &'static str,
    /// The 1-based line of [`Self::source_file`] this record was read from.
    pub source_line: u32,
}

/// One ingested monster book: its corpus directory id and its two tables.
///
/// Every field is *data*, never behaviour — the resolve/link rules below are
/// identical across books because they are properties of PCGen's `.lst` format.
/// That is what makes a row here the whole cost of registering a book.
#[derive(Debug, Clone, Copy)]
pub struct MonsterBook {
    /// The corpus directory this book's records file under, which is also the
    /// `engine_book` `v06_work_inventory` joins on and the namespace every
    /// served key carries.
    pub corpus_book: &'static str,
    pub monsters: &'static [MonsterStatBlock],
    pub monster_abilities: &'static [MonsterAbilityRecord],
    /// Monster NAMES a shipped [`MonsterAbilityRecord::owners`] entry in this
    /// book may cite even though `monsters` above holds no
    /// [`MonsterStatBlock`] for them -- their stat block ships from a
    /// DIFFERENT compiled table entirely (`decisions.md §58.3`'s
    /// CROSS-TABLE-OWNER class, `SD31-W23-MONSTER-001`). Empty for every
    /// book except `beastiary`, whose 46 legacy Bestiary 1 monsters ship
    /// from `rules_tables::beastiary1` instead of from here -- see that
    /// book's own `cross_table_owner_names()` for the derivation. An owner
    /// naming a monster in neither `monsters` nor this list is still a real
    /// defect `the_chassis_link_resolves_in_both_directions_for_every_book`
    /// catches; this only widens what counts as a KNOWN, cited owner.
    pub cross_table_owner_names: &'static [&'static str],
}

impl MonsterBook {
    /// The stat block with this corpus key, if this book defines one.
    pub fn monster_resolve(&self, key: &str) -> Option<&'static MonsterStatBlock> {
        self.monsters.iter().find(|m| m.key == key)
    }

    /// The ability record with this corpus key, if this book defines one.
    pub fn monster_ability_resolve(&self, key: &str) -> Option<&'static MonsterAbilityRecord> {
        self.monster_abilities.iter().find(|a| a.key == key)
    }

    /// The abilities a monster holds, resolved through its own `ability_keys`.
    pub fn abilities_of(&self, monster: &MonsterStatBlock) -> Vec<&'static MonsterAbilityRecord> {
        monster
            .ability_keys
            .iter()
            .filter_map(|key| self.monster_ability_resolve(key))
            .collect()
    }

    /// The abilities this book's own `MonsterAbilityRecord::owners` names a
    /// given monster NAME as granting, independent of whether this table
    /// holds that monster's own [`MonsterStatBlock`].
    ///
    /// [`Self::abilities_of`] walks a `MonsterStatBlock`'s own `ability_keys`
    /// field forward from the monster; this walks `owners` backward from the
    /// ability instead, so it also resolves a CROSS-TABLE OWNER row -- one
    /// whose owning stat block ships from a *different* compiled table
    /// (`rules_tables::beastiary1`'s 46 legacy Bestiary 1 monsters, here) and
    /// so has no `ability_keys` list in THIS table to walk
    /// (`scripts/transcribe_monster_tables.py`'s own cross-table-owner
    /// screen doc comment on the generated `bestiary` table names the exact
    /// 55 rows this exists for). A monster this table itself defines could
    /// call this too, but should not: it would silently skip any ability
    /// this book's OWN generator dropped from that monster's `ability_keys`
    /// (an unscreenable `DESC:` shape, e.g.) while `owners` still names it,
    /// serving a record `abilities_of` correctly refuses. Reserved for a
    /// monster with no `ability_keys` list of its own to begin with.
    pub fn abilities_owned_by_name(&self, name: &str) -> Vec<&'static MonsterAbilityRecord> {
        self.monster_abilities
            .iter()
            .filter(|a| a.owners.contains(&name))
            .collect()
    }
}

/// Every book whose `monster` / `monster_ability` rows this repo has ingested.
///
/// Adding a book here is what makes its records reach the work inventory, the
/// corpus cache, the monster catalog and the reach gate at once — none of those
/// consumers names a book of its own.
pub const MONSTER_BOOKS: &[MonsterBook] = &[
    MonsterBook {
        corpus_book: "bonus_bestiary",
        monsters: super::bonus_bestiary::monsters_static(),
        monster_abilities: super::bonus_bestiary::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "monster_codex",
        monsters: super::monster_codex::monsters_static(),
        monster_abilities: super::monster_codex::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "book_of_the_damned_volume_1",
        monsters: super::book_of_the_damned_volume_1::monsters_static(),
        monster_abilities: super::book_of_the_damned_volume_1::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "book_of_the_damned_volume_2",
        monsters: super::book_of_the_damned_volume_2::monsters_static(),
        monster_abilities: super::book_of_the_damned_volume_2::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "inner_sea_world_guide",
        monsters: super::inner_sea_world_guide::monsters_static(),
        monster_abilities: super::inner_sea_world_guide::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // SD-29 Epic 5 extend, round 4. Bestiary 2 -- 316 monsters and 402 owned
    // abilities, four times every book above it put together. The registry
    // absorbs it as one more row, which is the property the chassis was built
    // for; the book's own module records why 64 of its 466 ability rows are not
    // here.
    MonsterBook {
        corpus_book: "bestiary_2",
        monsters: super::bestiary_2::monsters_static(),
        monster_abilities: super::bestiary_2::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // SD-29 Epic 5 extend, round 5. Bestiary 3 -- 261 monsters and 27 owned
    // abilities, and the first book in the registry to lose no monster row at
    // all: no Product Identity row, no `.COPY=` delta. Its 13 excluded ability
    // rows are orphans, pinned by line in `rules_tables::bestiary_3`.
    MonsterBook {
        corpus_book: "bestiary_3",
        monsters: super::bestiary_3::monsters_static(),
        monster_abilities: super::bestiary_3::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // SD-29 Epic 5 extend, round 6. Bestiary 4 -- 206 monsters and 543 owned
    // abilities, the largest reachable book left in the lane. It is the first
    // book in the registry to lose monster rows to Product Identity: 14 of its
    // 220 corpus rows declare `NAMEISPI:YES` and are unique named personas
    // rather than species. That drop cascades -- 73 of its 225 excluded ability
    // rows are well-formed and owned, and unreachable only because their owner
    // is one of the 14. `rules_tables::bestiary_4` derives both figures.
    MonsterBook {
        corpus_book: "bestiary_4",
        monsters: super::bestiary_4::monsters_static(),
        monster_abilities: super::bestiary_4::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // SD-29 Epic 5 extend, round 7. Inner Sea Bestiary -- 38 monsters and 152
    // owned abilities. The first book in the registry to lose monster rows to
    // the Product Identity of the abilities they NAME rather than of their own
    // name: the emitted `ability_keys` array carries the ability's key, so a
    // monster naming a deity-namespaced ability cannot be emitted either.
    // `rules_tables::inner_sea_bestiary` derives it, and records that this makes
    // `classify_monster_ability_rows.py`'s `reachable remainder` an upper bound
    // rather than an equality for any book with that shape.
    MonsterBook {
        corpus_book: "inner_sea_bestiary",
        monsters: super::inner_sea_bestiary::monsters_static(),
        monster_abilities: super::inner_sea_bestiary::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // SD-29 Epic 5 extend, round 9. Inner Sea Gods -- 39 monsters and 77 owned
    // abilities. The first book in this registry whose corpus rows do not all
    // live in the book's root directory: 3 monster rows come from
    // `support/isg_races_b4.lst`, which is why both the transcriber and the
    // generator now RESOLVE a `source_file` basename against the book tree
    // instead of joining it onto the root. Its module header records the
    // 16-row `Race Traits ~` bundle finding -- abilities with a real owner that
    // the ownership pass cannot see because the corpus states the link through
    // a `CATEGORY:Internal` row rather than on the monster row itself.
    MonsterBook {
        corpus_book: "inner_sea_gods",
        monsters: super::inner_sea_gods::monsters_static(),
        monster_abilities: super::inner_sea_gods::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // SD-29 Epic 5 extend, round 8. Bestiary 1 -- 284 monsters and 323 owned
    // abilities, the largest single row in this registry and the only book
    // served by TWO compiled monster tables. `rules_tables::beastiary1` (SD-22)
    // holds 46 hand-modelled stat blocks of the same book; this chassis holds
    // the complement, per `decisions.md §58.3`'s ALONGSIDE ruling, and
    // `rules_tables::bestiary`'s `no_creature_is_served_by_both_bestiary_1_tables`
    // is the guard that keeps the two disjoint.
    //
    // `corpus_book` is `beastiary`, NOT `bestiary`: every consumer of this field
    // reads a `data/corpus/` directory -- `gen_book_cache`'s output root and
    // `reach_gate`'s denominator -- and this book's directory has been spelled
    // `beastiary` since SD-22. Registering the source spelling would write a
    // SECOND corpus directory for a book that already has one, which is the
    // defect `decisions.md §54.3` records the companion lane catching.
    MonsterBook {
        corpus_book: "beastiary",
        monsters: super::bestiary::monsters_static(),
        monster_abilities: super::bestiary::monster_abilities_static(),
        cross_table_owner_names: super::bestiary::cross_table_owner_names(),
    },
    // SD-29 Epic 5 extend, round 10. Ultimate Psionics (Dreamscarred Press) --
    // 21 monsters and 13 owned abilities, and the first NON-PAIZO book in this
    // registry. It is also the first whose `RuleSetId` was already compiled for
    // other kinds: `RuleSetId::Upsi` has served this book's feats, equipment and
    // archetypes since SD-28 E29, so registering its monsters added no rule set,
    // no corpus directory and no work-inventory book entry.
    //
    // Both `.lst` files sit at the book root, so `resolve_book_file` is not
    // load-bearing here. Its module header records this book's share of the
    // `Racial Traits ~` bundle class (`decisions.md §62.4`, measured
    // corpus-wide in `§64.1`): 2 of its 66 orphans are owned in the corpus
    // through a `CATEGORY:Internal` row and are pinned by an executing test.
    MonsterBook {
        corpus_book: "ultimate_psionics",
        monsters: super::ultimate_psionics::monsters_static(),
        monster_abilities: super::ultimate_psionics::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // SD-29 Epic 5 extend, FINAL round. Horror Adventures -- 3 monsters and 6
    // owned abilities, the smallest row in this registry and the last book in
    // the lane with any workable unit at all.
    //
    // Like `ultimate_psionics` its `RuleSetId` was already compiled for other
    // kinds (`RuleSetId::Ha`: `race_trait` since Epic 6 round 3, `companion`
    // since Epic 7), so registering its monsters adds no rule set, no corpus
    // directory and no work-inventory book entry.
    //
    // It is the first book in this registry whose monster rows state part of
    // their ATTACK list through the `ABILITY:Internal|AUTOMATIC|` bundle token
    // rather than through `NATURALATTACKS:` -- `ha_races.lst:4` prices one Claw
    // attack and names Bite and Tail Slap only in the bundle. That is the same
    // token `decisions.md §64.1` measures corpus-wide for OWNERSHIP; here it is
    // read for its attack segments, and the two uses are independent.
    MonsterBook {
        corpus_book: "horror_adventures",
        monsters: super::horror_adventures::monsters_static(),
        monster_abilities: super::horror_adventures::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // `decisions.md §20` no_record-to-zero, round 3. Five of the eight
    // zero-monster books `decisions.md §17a`'s re-derive found unregistered
    // (`scripts/classify_monster_ability_rows.py`'s own "ZERO-monster books"
    // line): every ability row in each ships owner-less by construction,
    // since no monster row of the book exists to own it. `mythic_adventures`
    // (21 rows) is still deferred -- its `rules_tables/` module directory
    // does not exist yet.
    MonsterBook {
        corpus_book: "ultimate_wilderness",
        monsters: super::ultimate_wilderness::monsters_static(),
        monster_abilities: super::ultimate_wilderness::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "ultimate_intrigue",
        monsters: super::ultimate_intrigue::monsters_static(),
        monster_abilities: super::ultimate_intrigue::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "ultimate_magic",
        monsters: super::ultimate_magic::monsters_static(),
        monster_abilities: super::ultimate_magic::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "bestiary_6",
        monsters: super::bestiary_6::monsters_static(),
        monster_abilities: super::bestiary_6::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "bestiary_5",
        monsters: super::bestiary_5::monsters_static(),
        monster_abilities: super::bestiary_5::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // `decisions.md §20` no_record-to-zero, round 4: the last two of the
    // original 8 zero-monster books, now registered. Both already have a
    // dedicated `gen_book_cache.rs` generator function for their OTHER
    // families (`gen_pathfinder_unchained`/`gen_advanced_race_guide`),
    // extended this round to also call `gen_monster_book` after its existing
    // writes.
    MonsterBook {
        corpus_book: "pathfinder_unchained",
        monsters: super::pathfinder_unchained::monsters_static(),
        monster_abilities: super::pathfinder_unchained::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    MonsterBook {
        corpus_book: "advanced_race_guide",
        monsters: super::advanced_race_guide::monsters_static(),
        monster_abilities: super::advanced_race_guide::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // `decisions.md §20` no_record-to-zero, round 5: the last of the original
    // 8 zero-monster books, deferred by round 4 pending its `rules_tables/`
    // module (created since by a sibling `spell` lane); the mechanism itself
    // is unchanged.
    MonsterBook {
        corpus_book: "mythic_adventures",
        monsters: super::mythic_adventures::monsters_static(),
        monster_abilities: super::mythic_adventures::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
    // `decisions.md §27b` — EVERYTHING: overturns four cycles' worth of
    // "correctly out of scope" for this book's 5 `monster_ability` units,
    // reasoning that a reachability finding (negated `PRECAMPAIGN` gate) is
    // not an ingest exemption. All 5 ship owner-less, same honest shape
    // `mythic_adventures` above ships; see `occult_adventures/monster_data.rs`
    // for the keys and `reach_gate.rs::UNREACHED_RECORD_FINDINGS` for the
    // pinned non-reach.
    MonsterBook {
        corpus_book: "occult_adventures",
        monsters: super::occult_adventures::monsters_static(),
        monster_abilities: super::occult_adventures::monster_abilities_static(),
        cross_table_owner_names: &[],
    },
];

/// The registered book with this corpus directory id.
pub fn monster_book(corpus_book: &str) -> Option<&'static MonsterBook> {
    MONSTER_BOOKS.iter().find(|b| b.corpus_book == corpus_book)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A book registered twice, or a book whose tables were wired to another
    /// book's statics, is a copy-paste defect the registry cannot otherwise
    /// see. Both are cheap to make and expensive to find on a screen.
    #[test]
    fn every_registered_book_is_distinct_and_non_empty() {
        let mut ids: Vec<_> = MONSTER_BOOKS.iter().map(|b| b.corpus_book).collect();
        let before = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), before, "a corpus book id is registered twice");
        for book in MONSTER_BOOKS {
            assert!(
                !book.monsters.is_empty() || !book.monster_abilities.is_empty(),
                "{} registers two empty tables",
                book.corpus_book
            );
        }
    }

    /// Keys are namespaced per book on the wire, but a key must still be unique
    /// *within* its book or `monster_resolve` silently returns the first of two.
    #[test]
    fn keys_are_unique_within_every_book() {
        for book in MONSTER_BOOKS {
            let mut keys: Vec<_> = book.monsters.iter().map(|m| m.key).collect();
            let before = keys.len();
            keys.sort_unstable();
            keys.dedup();
            assert_eq!(keys.len(), before, "{}: duplicate monster key", book.corpus_book);

            let mut keys: Vec<_> = book.monster_abilities.iter().map(|a| a.key).collect();
            let before = keys.len();
            keys.sort_unstable();
            keys.dedup();
            assert_eq!(keys.len(), before, "{}: duplicate ability key", book.corpus_book);
        }
    }

    /// The chassis link, held closed in both directions for every book: an
    /// ability a monster names is defined here, an ability listed as external is
    /// not, and every NAMED owner on a defined ability resolves back to a
    /// monster that names it (or is a known cross-table exception). An orphan
    /// means the link was transcribed wrong in one direction and the catalog
    /// would serve a record no monster row reaches. A CROSS-TABLE-OWNER ability
    /// (`SD31-W23-MONSTER-001`, `decisions.md §58.3`) is the one owner shape
    /// that legitimately has no `ability_keys` back-reference in THIS book --
    /// its `MonsterStatBlock` ships from a different one -- and the book's own
    /// `cross_table_owner_names` is what tells this test apart a real
    /// dangling owner from that known, cited exception.
    ///
    /// **Superseded `decisions.md §20` (no_record-to-zero wave 2)**: this no
    /// longer requires every ability to carry an owner at all. An
    /// intentionally owner-less record (no monster row of its book claims
    /// it) now SHIPS with `owners: &[]` rather than being dropped, because an
    /// un-ingested row's shape cannot be measured and Gate 1's DoD needs
    /// every unit's shape measured; `list_monster_catalog` only ever walks a
    /// monster's own `ability_keys`, so it never surfaces one. Each book that
    /// ships owner-less records pins the EXACT set in its own module (e.g.
    /// `bestiary::tests::every_owner_less_ability_is_a_named_and_pinned_non_reach`)
    /// — that is where a silent new arrival or disappearance is caught; this
    /// test's remaining job is that every NON-empty owner list still
    /// resolves correctly both ways.
    #[test]
    fn the_chassis_link_resolves_in_both_directions_for_every_book() {
        for book in MONSTER_BOOKS {
            for monster in book.monsters {
                for key in monster.ability_keys {
                    assert!(
                        book.monster_ability_resolve(key).is_some(),
                        "{}: {} names ability {key:?}, which the book does not define",
                        book.corpus_book,
                        monster.name
                    );
                }
                for key in monster.external_ability_refs {
                    assert!(
                        book.monster_ability_resolve(key).is_none(),
                        "{}: {} lists {key:?} as external, but the book defines it",
                        book.corpus_book,
                        monster.name
                    );
                }
            }
            for ability in book.monster_abilities {
                for owner in ability.owners {
                    match book.monster_resolve(owner) {
                        Some(monster) => assert!(
                            monster.ability_keys.contains(&ability.key),
                            "{}: {} claims owner {owner:?}, which does not name it back",
                            book.corpus_book,
                            ability.key
                        ),
                        // No `MonsterStatBlock` for this owner in THIS book --
                        // fine only when the book's own registry row names it
                        // as a known cross-table owner (`SD31-W23-MONSTER-001`,
                        // `decisions.md §58.3`): its stat block ships from a
                        // DIFFERENT compiled table, so it has no `ability_keys`
                        // list here to name the ability back with, by
                        // construction, not by omission. Anything NOT in that
                        // named list is still the real dangling-owner defect
                        // this test exists to catch.
                        None => assert!(
                            book.cross_table_owner_names.contains(owner),
                            "{}: owner {owner:?} is not a monster in this book and not in \
                             cross_table_owner_names either",
                            book.corpus_book
                        ),
                    }
                }
            }
        }
    }

    /// A transcription that dropped a name would show an empty heading on the
    /// catalog; a `damage_dice` of `""` would show a blank where a die
    /// expression belongs. Neither is representable after this test.
    #[test]
    fn no_record_carries_an_empty_string_where_a_value_is_claimed() {
        for book in MONSTER_BOOKS {
            for monster in book.monsters {
                assert!(!monster.key.trim().is_empty());
                assert!(!monster.name.trim().is_empty());
                for attack in monster.natural_attacks {
                    assert!(!attack.name.trim().is_empty());
                    if let Some(dice) = attack.damage_dice {
                        assert!(!dice.trim().is_empty(), "{}: empty damage dice", book.corpus_book);
                    }
                }
            }
            for ability in book.monster_abilities {
                assert!(!ability.key.trim().is_empty());
                assert!(!ability.name.trim().is_empty());
                if let Some(desc) = ability.description {
                    assert!(!desc.trim().is_empty(), "{}: empty description", book.corpus_book);
                }
            }
        }
    }

    /// Monster Codex's link shape is the one Bonus Bestiary never had: none of
    /// its 3 ability rows is named by a monster row's `ABILITY:Special Ability`
    /// token — the owner is the first segment of the ability's own namespaced
    /// key. A transcriber that only read the monster row would have produced 3
    /// orphans and a book with no reachable abilities.
    #[test]
    fn monster_codex_abilities_link_through_their_namespaced_key() {
        let book = monster_book("monster_codex").expect("Monster Codex is registered");
        assert_eq!(book.monsters.len(), 2);
        assert_eq!(book.monster_abilities.len(), 3);
        for ability in book.monster_abilities {
            let (owner, leaf) = ability.key.split_once(" ~ ").expect("key is namespaced");
            assert_eq!(ability.name, leaf);
            assert_eq!(ability.owners, &[owner]);
        }
        let seru = book.monster_resolve("Seru").expect("Seru is in this book");
        let names: Vec<_> = book.abilities_of(seru).iter().map(|a| a.name).collect();
        assert_eq!(names, vec!["Poison", "Spit Venom"]);
    }

    /// **Mutation-proves the ability-score-bonus widening** (SD31-E6-F1-002).
    /// Re-reads Demon (Balor)'s own row from the pinned PCGen oracle (never
    /// the corpus JSON cache, never this crate's own output — the independent
    /// upstream source) and asserts the static table's `stat_adjustments`
    /// matches it token for token. The static table and this re-read are two
    /// independently produced artifacts (one baked in by the Python
    /// transcriber at generation time, one parsed fresh here at test time); a
    /// corrupted or invented value in either one fails this test, which is
    /// what makes it a mutation-proof rather than a self-check. Skips (rather
    /// than fails) when the pinned oracle checkout is absent, matching this
    /// program's existing convention for oracle-dependent tests — `verify.sh`
    /// bootstraps the oracle before this test suite runs.
    #[test]
    fn demon_balor_stat_adjustments_match_the_live_pinned_corpus_row() {
        let root = std::env::var("PCGEN_CORPUS_ROOT").unwrap_or_else(|_| {
            format!("{}/workspace/repos/pcgen/data", std::env::var("HOME").expect("HOME is set"))
        });
        let path = std::path::Path::new(&root)
            .join("pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst");
        let Ok(text) = std::fs::read_to_string(&path) else {
            eprintln!("skip: pinned oracle not present at {path:?}");
            return;
        };
        let line = text.lines().nth(92).expect("b1_races.lst has at least 93 lines"); // 1-based line 93
        assert!(
            line.starts_with("Demon (Balor)"),
            "b1_races.lst:93 is no longer Demon (Balor) — the oracle moved: {line:?}"
        );

        // The SAME parse `parse_stat_adjustments` in
        // `scripts/transcribe_monster_tables.py` performs, re-implemented
        // independently here in Rust rather than shelling out to the Python —
        // an independent re-derivation is the point of a mutation proof.
        let mut expected: Vec<(String, i16)> = Vec::new();
        for field in line.split('\t') {
            let field = field.trim();
            let Some(rest) = field.strip_prefix("BONUS:STAT|") else { continue };
            let parts: Vec<&str> = rest.split('|').collect();
            if parts.len() < 2 {
                continue;
            }
            let Ok(amount) = parts[1].trim().parse::<i16>() else { continue };
            for ability in parts[0].split(',') {
                expected.push((ability.trim().to_string(), amount));
            }
        }
        assert!(!expected.is_empty(), "Demon (Balor)'s row carries no readable BONUS:STAT token");

        let book = monster_book("beastiary").expect("bestiary chassis is registered");
        let balor = book
            .monster_resolve("Demon (Balor)")
            .expect("Demon (Balor) is a registered monster in the bestiary chassis");
        let actual: Vec<(String, i16)> =
            balor.stat_adjustments.iter().map(|a| (a.ability.to_string(), a.amount)).collect();
        assert_eq!(
            actual, expected,
            "the static table's stat_adjustments diverges from a fresh, independent parse of \
             the live pinned corpus row"
        );
    }

    /// The presence signal `spell_like_ability_caster_level` gates on: a
    /// monster with a genuine `BONUS:VAR|SLA_CL|` token is `true`, one with
    /// none is `false` — never guessed from
    /// `ability_keys`/`external_ability_refs`, which name abilities but do
    /// not distinguish a spell-like one from any other kind (SD31-E6-F1-002,
    /// `OPEN-ISSUES.md` row 44).
    #[test]
    fn has_spell_like_abilities_is_true_only_for_a_row_with_an_sla_cl_token() {
        let book = monster_book("beastiary").expect("bestiary chassis is registered");
        let balor = book
            .monster_resolve("Demon (Balor)")
            .expect("Demon (Balor) is a registered monster in the bestiary chassis");
        assert!(
            balor.has_spell_like_abilities,
            "Demon (Balor)'s row carries BONUS:VAR|SLA_CL|HD"
        );

        // Linnorm (Crag) is the TDD anchor for why the gate is `SLA_CL`, not
        // the more general `SPELLS:` token: its spell-like effects
        // (`True Seeing ~ Constant`) reach the row only through an
        // `ABILITY:` cross-reference, and the row carries no `SPELLS:` token
        // at all -- an earlier version of this gate answered `false` here
        // and broke one of this seam's own already-committed fixtures.
        let linnorm = book
            .monster_resolve("Linnorm (Crag)")
            .expect("Linnorm (Crag) is a registered monster in the bestiary chassis");
        assert!(
            linnorm.has_spell_like_abilities,
            "Linnorm (Crag)'s row (b1_races.lst:269) carries BONUS:VAR|SLA_CL|HD but no \
             SPELLS: token at all"
        );

        let animated_object = book
            .monster_resolve("Animated Object (Medium)")
            .expect("Animated Object (Medium) is a registered monster in the bestiary chassis");
        assert!(
            !animated_object.has_spell_like_abilities,
            "Animated Object (Medium)'s row (b1_races.lst:13) carries no SLA_CL token — it \
             has no spell-like abilities"
        );
    }

    /// **Pinning test for the T9 `MonsterAbilityFacet` widening
    /// (`decisions.md §16`'s caution, applied).** Hashes every currently-
    /// shipped `(corpus_book, ability_key, facet)` triple across every
    /// registered book and pins the digest. Adding `Weakness`/`Defensive`/
    /// `Aura`/`Sense`/`Communicate` to the enum must not change ONE existing
    /// record's facet — those variants are reached only by
    /// `scripts/transcribe_monster_tables.py` re-running against rows that
    /// were previously unparseable (`facet is None` → `SystemExit`), never by
    /// reinterpreting a row that already resolved to `SpecialAttack`/
    /// `SpecialQuality`.
    ///
    /// **This test was proven to actually fail, not just pass by
    /// construction**, by temporarily widening `parse_type` to a naive
    /// first-non-facet-segment-wins rule (mirroring the `refine_kind`
    /// unsafe-widening shape `decisions.md §16` names) and re-running
    /// `transcribe_monster_tables.py` for `bestiary_2` — the run reclassified
    /// `Denizen of Leng ~ Planar Fast Healing` from an unmodelled row into a
    /// wrong `SpecialQuality` (its true first TYPE segment is `ModifyHP`, not
    /// a real facet), and THIS test caught it (digest mismatch) before the
    /// change reached this file. The naive widening was reverted; the
    /// deliberate, per-shape widening actually shipped here does not trigger
    /// it. The failure branch is real for ANY book in [`MONSTER_BOOKS`], not
    /// only the one used to prove it: the assertion iterates the whole
    /// registry, and a naive widening's failure mode (misreading whichever
    /// row happens first in `TYPE:` order) is not specific to `bestiary_2`.
    ///
    /// **Pin history.** `2214` (this test's original pin, immediately after
    /// the enum widened but before any of the five books were re-transcribed)
    /// -> `2656` (+442, after re-running `transcribe_monster_tables.py` for
    /// `bestiary`/`bestiary_2`/`bestiary_3`/`inner_sea_bestiary`/
    /// `inner_sea_gods`). The 442 new records were verified additions-only —
    /// every one of the original 2214 `(book, key)` pairs still carries the
    /// SAME facet in the regenerated files, checked by diffing each touched
    /// book's `monster_data.rs` against its pre-regen `git show HEAD:` content
    /// (0 removed, 0 changed, 442 added, exactly matching this test's own
    /// count delta) — this test's own before/after count is one more
    /// independent confirmation of the same fact.
    #[test]
    fn widening_the_facet_vocabulary_does_not_reclassify_any_existing_record() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut triples: Vec<(&str, &str, &str)> = Vec::new();
        for book in MONSTER_BOOKS {
            for ability in book.monster_abilities {
                triples.push((book.corpus_book, ability.key, ability.facet.corpus_token()));
            }
        }
        triples.sort_unstable();

        let mut hasher = DefaultHasher::new();
        triples.hash(&mut hasher);
        let digest = hasher.finish();

        assert_eq!(
            triples.len(),
            3806,
            // 3706 -> 3711 (`decisions.md §27b`, +5): `occult_adventures`
            // registered for the first time, all 5 rows owner-less.
            // 3711 -> 3726 (`decisions.md §24`/round 7, +15): the T9
            // name-PI/desc-PI `monster_ability` group closes. 13 rows whose
            // own name/key matched the blacklist now ship under a
            // Codex-generated neutral name/key (`decisions.md §24`) rather
            // than being dropped; 2 rows whose CLEAN name/key had an
            // undeclared blacklist hit confined to `DESC:` now ship with
            // the description redacted, the same path `DESCISPI:YES`
            // already used. Both are strictly additive: no PREVIOUSLY-
            // SHIPPED triple's facet/delivery moved, only 15 rows that were
            // previously DROPPED now ship — confirmed by `git diff
            // --stat`'s 21-book regeneration showing only new
            // `MonsterAbilityRecord` blocks appended, zero removed, across
            // the 3 books (`inner_sea_bestiary`, `inner_sea_gods`,
            // `inner_sea_world_guide`) this group's population lives in.
            // 3726 -> 3749 (`decisions.md §27`/round 8, +23): the
            // `TYPE:`-facet-vocabulary-gap group closes via the operator's
            // provisional-default ruling — a row whose `TYPE:` segments name
            // no modeled facet now ships with `facet:
            // MonsterAbilityFacet::SpecialQuality` (a PROVISIONAL default,
            // not a measured shape; marked via `data.shape_provisional_
            // default`/`data.shape_provisional_reason` on the shipped JSON
            // record, `workflow-instruction.md §6a`) instead of being
            // dropped. 22 of the 23 are the T9 round 6/7-named 22-unit
            // remaining population (`bestiary` +1, `bestiary_2` +7,
            // `bestiary_3` +11, `inner_sea_bestiary` +2, `inner_sea_gods`
            // +1); the 23rd (`bestiary_2`'s `Bunyip ~ Blood Rage`) is a
            // `.COPY=`-shaped row `docs/work-inventory.json` already counted
            // `text-complete` by evidence alone with no backing corpus
            // record — this cycle's regen incidentally backs that claim
            // with a real record for the first time, same mechanism, same
            // provisional marker. Strictly additive: every one of the 23
            // was previously ABSENT from the set entirely (raised
            // `UnmodelledFacet` and was dropped, never shipped under a
            // different facet), so no existing triple's facet/delivery
            // value changed — confirmed by `git diff --stat`'s 5-book
            // regeneration showing only new `MonsterAbilityRecord` blocks
            // appended, zero removed (`bestiary`, `bestiary_2`,
            // `bestiary_3`, `inner_sea_bestiary`, `inner_sea_gods`), and by
            // this test's own digest re-derivation below from the live
            // failing run, never guessed, per `decisions.md §17a`.
            "the number of currently-shipped monster_ability records changed — re-derive \
             this pin (and the digest below) only from a real corpus regen, never to make a \
             facet-widening change pass. 2656 -> 2836 (`decisions.md §20`, no_record-to-zero \
             wave 2): +180 owner-less `bestiary` records now ship for shape measurement \
             rather than being dropped as orphans — see \
             `bestiary::tests::every_owner_less_ability_is_a_named_and_pinned_non_reach`. \
             2836 -> 3537 (`decisions.md §20`, no_record-to-zero wave 2 FOLLOW-ON, +701): the \
             identical owner-less-ship mechanism applied to the 8 remaining registered books \
             (`bestiary_2` +85, `bestiary_3` +266, `bestiary_4` +187, `horror_adventures` +56, \
             `inner_sea_bestiary` +28, `inner_sea_gods` +2, `inner_sea_world_guide` +13, \
             `ultimate_psionics` +64) — each book's own module pins its exact owner-less set, \
             see `every_owner_less_ability_is_a_named_and_pinned_non_reach` in each. \
             3537 -> 3613 (`decisions.md §20` round 3, +76): 5 PREVIOUSLY-UNREGISTERED \
             zero-monster books added to `MONSTER_BOOKS` for the first time \
             (`ultimate_wilderness` +2, `ultimate_intrigue` +6, `ultimate_magic` +13, \
             `bestiary_6` +16, `bestiary_5` +39) — structurally cannot reclassify an existing \
             record's facet, since no pre-existing `MonsterBook` entry was modified, only new \
             ones added. 3613 -> 3683 (`decisions.md §20` round 4, +70): the last 2 of the \
             original 8 unregistered zero-monster books, `pathfinder_unchained` (+69, 3 of the \
             72 orphan candidates refused during transcription as an unscreenable multi-DESC: \
             shape — see `pathfinder_unchained/monster_data.rs`'s own header) and \
             `advanced_race_guide` (+1) — again structurally additive only, reached via each \
             book's own `gen_book_cache.rs` generator function extended to also call \
             `gen_monster_book`. 3683 -> 3704 (`decisions.md §20` round 5, +21): the last of \
             the original 8 unregistered zero-monster books, `mythic_adventures` (+21, all 21 \
             orphan candidates shipped, 0 refused) — structurally additive only, reached \
             entirely through `gen_book_cache.rs`'s generic `monster_book_spec` fallback arm, \
             no new generator code. 3704 -> 3706 (`decisions.md §22`/round 6, +2): two \
             `type_segments` upstream-data corrections -- a comma-delimiter row \
             (`bestiary`'s `Spectre ~ Create Spawn`, `TYPE:SpecialAttack,Supernatural`) and a \
             misspelled facet segment (`bestiary_2`'s `Tick Swarm ~ Cling`, \
             `TYPE:SpecialAttck.Extraordinary`) each now resolve a real, already-modelled \
             facet (`SpecialAttack`) instead of raising `UnmodelledFacet` -- structurally \
             additive only: no existing `MonsterBook` entry's other rows changed, confirmed by \
             `git status --porcelain` showing exactly the 2 new `??` corpus files plus the 2 \
             regenerated `monster_data.rs` files, zero deletions. 3706 -> 3711 \
             (`decisions.md §27b`, +5): `occult_adventures` registered for the first time -- \
             `decisions.md §27b` overturns the repeatedly-reconfirmed \"correctly out of scope\" \
             disposition for this book's 5 `monster_ability` units (a reachability finding \
             about a negated `PRECAMPAIGN` gate, not an ingest exemption). Structurally \
             additive only: no pre-existing `MonsterBook` entry was modified, only one new one \
             (`occult_adventures`) appended, all 5 rows ship owner-less. 3749 -> 3806 \
             (`decisions.md §27b` round 9, +57): the last `monster_ability` `no_record` group \
             closes -- the multi-`DESC:` `PRERULE`/`PREVAREQ`/`PREVARGT`/`PRESIZE*`/`PREHD`/ \
             `PRERACE`/`PRETEMPLATE`/`PREABILITY`-gated parse-refusal shape, via `parse_desc`'s \
             new generalised sixth branch (`_concat_desc_variants`), which concatenates every \
             `DESC:` token's own verbatim text instead of guessing which one wins. 56 real \
             `no_record` units plus 1 bonus (`bestiary`'s `Lycanthrope ~ Change Shape`, already \
             counted `text-complete` by inventory evidence alone with no backing corpus record \
             -- same shape as round 8's `Bunyip ~ Blood Rage`). Structurally additive only: \
             every one of the 57 was previously ABSENT from the set entirely (raised \
             `UnmodelledDesc` and was dropped, never shipped under a different facet), so no \
             existing triple's facet/delivery value changed -- confirmed by `git diff --stat`'s \
             8-book regeneration (`bestiary`, `bestiary_2`, `bestiary_3`, `bestiary_4`, \
             `bestiary_5`, `horror_adventures`, `inner_sea_bestiary`, `pathfinder_unchained`) \
             showing only new `MonsterAbilityRecord` blocks appended, zero removed, and by this \
             test's own digest re-derivation below from the live failing run, never guessed, \
             per `decisions.md §17a`."
        );
        assert_eq!(
            digest, 0x874e_04c1_47ee_bb76,
            "an EXISTING record's facet moved. `Weakness`/`Defensive`/`Aura`/`Sense`/\
             `Communicate` may only be reached by rows that previously raised \
             `parse_type`'s SystemExit — if this fires, some already-shipped \
             SpecialAttack/SpecialQuality row was reclassified, which is exactly the \
             defect this test exists to catch (decisions.md §16). 0x7f1fd137006b6cbd -> \
             0xada455b5de6bafc7 (`decisions.md §20`): the digest moves whenever the SORTED \
             triple set gains members, even with zero reclassification — independently \
             confirmed zero-reclassification here via `gen_book_cache beastiary`'s own report \
             (`0 new monsters ... 529 already on disk, left untouched, 180 new monster \
             abilities`) and a `git diff` of `bestiary/monster_data.rs` showing every \
             pre-existing record's fields byte-identical (only file-position reordering, from \
             orphans keeping their real `source_line` instead of being dropped). \
             0xada455b5de6bafc7 -> 0x5c2ee6087da263c9 (`decisions.md §20` round 3): the digest \
             moves because the sorted triple set gains 76 new members from 5 newly-registered \
             books — zero reclassification, since every pre-existing `MonsterBook` entry (and \
             every triple it contributes) is byte-unchanged; only new `MonsterBook` rows were \
             appended to `MONSTER_BOOKS`. 0x5c2ee6087da263c9 -> 0x2fa5c4578c0267bb \
             (`decisions.md §20` round 4): the sorted triple set gains 70 new members from 2 \
             newly-registered books (`pathfinder_unchained`/`advanced_race_guide`) — zero \
             reclassification, same reasoning as round 3, only new `MonsterBook` rows appended. \
             0x2fa5c4578c0267bb -> 0xd732c20ec4c2a946 (`decisions.md §20` round 5): the sorted \
             triple set gains 21 new members from the last newly-registered book \
             (`mythic_adventures`) — zero reclassification, same reasoning as rounds 3-4, only \
             one new `MonsterBook` row appended. 0xd732c20ec4c2a946 -> 0x38f4aedd6de1caf3 \
             (`decisions.md §22` round 6): two `type_segments` upstream-data corrections, zero \
             reclassification, same additive reasoning as rounds 3-5. 0x38f4aedd6de1caf3 -> \
             0xc4c144e1483d297d (`decisions.md §27b`): the sorted triple set gains 5 new \
             members from the newly-registered `occult_adventures` — zero reclassification, \
             same reasoning as rounds 3-5, only one new `MonsterBook` row appended. \
             0xc4c144e1483d297d -> 0xc7f55369ed187098 (`decisions.md §24`/round 7): the digest moves \
             because the sorted triple set gains 15 more members (the name-PI/desc-PI group \
             closing) — zero reclassification: every one of the 15 was previously ABSENT from \
             the set entirely (dropped, not shipped under a different facet), so no existing \
             triple's facet/delivery value changed; re-derived live from this test's own \
             failing run after merging both concurrent lanes' changes, never guessed, per \
             `decisions.md §17a`. 0xc7f55369ed187098 -> 0xfc5121106900558e (`decisions.md §27`/ \
             round 8): the digest moves because the sorted triple set gains 23 more members (the \
             `TYPE:`-facet-gap group closing via the provisional `SpecialQuality` default) — zero \
             reclassification: every one of the 23 was previously ABSENT from the set entirely \
             (raised `UnmodelledFacet` and was dropped, never shipped under a different facet), \
             so no existing triple's facet/delivery value changed; re-derived live from this \
             test's own failing run, never guessed, per `decisions.md §17a`. \
             0xfc5121106900558e -> 0x8b2ca909f9675cd5 (`decisions.md §27b` round 9): the digest \
             moves because the sorted triple set gains 57 more members (the multi-`DESC:` \
             `PREVAREQ`/`PREVARGT`-gated group closing via `parse_desc`'s new generalised sixth \
             branch) — zero reclassification: every one of the 57 was previously ABSENT from \
             the set entirely (raised `UnmodelledDesc` and was dropped, never shipped under a \
             different facet), so no existing triple's facet/delivery value changed; re-derived \
             live from this test's own failing run, never guessed, per `decisions.md §17a`. \
             0x8b2ca909f9675cd5 -> 0x874e04c147eebb76 (t9-onboarding, \
             corpus-literal-sweep-remainder cycle, round 10): the FIRST round where the triple \
             COUNT above does not move (3806 -> 3806) yet the digest does — because this round is \
             a genuine, DELIBERATE reclassification of 4 already-shipped triples, not an \
             addition. `f76242cc69` (row 17's own closure cycle, `decisions.md §27`/`§27a`/ \
             `§27b`) individually re-derived each of the 23 `§27`-provisional-default \
             `monster_ability` units against corpus/oracle evidence and found 4 of them \
             genuinely `SpecialAttack`, not the provisional `SpecialQuality` default they had \
             shipped under: `bestiary_2 ~ Aurumvorax ~ Rake`, `bestiary_2 ~ Bunyip ~ Blood \
             Rage`, `bestiary_2 ~ Yrthak ~ Sonic Lance`, `bestiary_2 ~ Howler ~ Abyssal Strike` \
             — each corroborated by a genuinely-declared sibling record per that commit's own \
             message, applied through the newly-sanctioned `_MONSTER_ABILITY_FACET_OVERRIDES` \
             mechanism in `transcribe_monster_tables.py`, not by hand-editing `monster_data.rs`, \
             and mutation-proved live (`§1a`) in that same cycle. This is exactly the class of \
             change this test's own doc comment (`decisions.md §16`) exists to CATCH when it is \
             an ACCIDENT of careless vocabulary widening — it is not that here: it is the \
             intended, evidence-backed output of the row 17 categorization epic itself, verified \
             per-record against the pinned oracle rather than asserted, and it is the reason the \
             count assert above stays fixed at 3806 while this digest alone moves. Re-derived \
             live from this test's own failing run at HEAD (`f76242cc69`, already merged to \
             `origin/tranche/12` — this cycle only updates the stale ratchet pin left behind), \
             never guessed, per `decisions.md §17a`."
        );
    }
}
