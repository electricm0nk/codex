//! Canonical Paizo-table store. SD-19 foundation slice.
//!
//! CRB (`crb`), APG (`apg`), ACG (`acg`), Bestiary 1 (`beastiary1`),
//! Advanced Race Guide (`advanced_race_guide`), and Pathfinder Unchained
//! (`pathfinder_unchained`) are the populated rule sets today. Future rule
//! books (UM, ...) get sibling directories and their own `RuleSetId`
//! variants in their own STC sub-bundle — see
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9 and
//! `SD-22-content-source-ingest-and-dm-toolkit/decisions.md` §5.

pub mod acg;
/// Adventurer's Guide. SD-31 wave-29 (`lane5-book-onboard` lane) -- the
/// book's FIRST compiled rule set of any kind, first record family: base
/// spell declarations transcribed from `ag_spells.lst`
/// (`rules_tables::adventurers_guide::spell_list`). See
/// `src/bin/ingest_adventurers_guide_spells.rs` for the ingest path.
pub mod adventurers_guide;
pub mod advanced_race_guide;
pub mod apg;
pub mod archetype_swap;
pub mod beastiary1;
/// Bestiary 1's monster/monster-ability **chassis**, holding the 284 rows
/// [`beastiary1`] does not — SD-29 Epic 5 extend round 8, `decisions.md §58.3`.
/// The two modules serve one book from two tables on purpose; see this one's
/// header. The near-homograph with `beastiary1` is the corpus's own spelling
/// split (`decisions.md §54.3` lists all four), not a typo in either name.
pub mod bestiary;
pub mod bestiary_2;
pub mod bestiary_3;
pub mod bestiary_4;
pub mod bestiary_5;
pub mod bestiary_6;
pub mod bonus_bestiary;
pub mod book_of_the_damned_volume_1;
pub mod book_of_the_damned_volume_2;
pub mod class_spell_levels;
pub mod companion_chassis;
pub mod crb;
pub mod equipment_gap_tables;
pub mod feat_gap_tables;
pub mod feats_all;
pub mod horror_adventures;
pub mod inner_sea_bestiary;
pub mod inner_sea_combat;
/// Inner Sea Faiths. SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this book's
/// FIRST compiled rule set of any kind, first record family: base spell
/// declarations transcribed from `isf_spells.lst`
/// (`rules_tables::inner_sea_faiths::spell_list`). See
/// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
pub mod inner_sea_faiths;
pub mod inner_sea_gods;
pub mod inner_sea_intrigue;
/// Inner Sea Races. SD-32 `decisions.md §20`, no_record-to-zero wave --
/// this book's FIRST compiled rule set of any kind, first record family:
/// base spell declarations transcribed from `isr_spells.lst`
/// (`rules_tables::inner_sea_races::spell_list`). See
/// `src/bin/ingest_spells.rs` for the ingest path.
pub mod inner_sea_races;
/// Inner Sea Magic. SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this book's
/// FIRST compiled rule set of any kind, first record family: base spell
/// declarations transcribed from `ism_spells.lst`
/// (`rules_tables::inner_sea_magic::spell_list`). See
/// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
pub mod inner_sea_magic;
/// Inner Sea Temples. SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this book's
/// FIRST compiled rule set of any kind, first record family: base spell
/// declarations transcribed from `istem_spells.lst`
/// (`rules_tables::inner_sea_temples::spell_list`). See
/// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
pub mod inner_sea_temples;
pub mod inner_sea_world_guide;
pub mod monster_chassis;
pub mod monster_codex;
/// Mythic Adventures. SD-32 `decisions.md §20`, no_record-to-zero wave --
/// this book's FIRST compiled rule set of any kind, first record family:
/// base spell declarations transcribed from `ma_spells.lst`
/// (`rules_tables::mythic_adventures::spell_list`). See
/// `src/bin/ingest_spells.rs` for the ingest path.
pub mod mythic_adventures;
pub mod occult_adventures;
pub mod pathfinder_unchained;
pub mod ultimate_campaign;
pub mod ultimate_equipment;
pub mod ultimate_intrigue;
pub mod ultimate_combat;
pub mod ultimate_magic;
/// Ultimate Magic — Words of Power example combined spells. SD-32
/// `decisions.md §20`, `no_record`-to-zero wave: a second, distinct
/// source `.lst` file for the SAME shipped book (`ultimate_magic`); see
/// this module's own doc comment for why it is a separate Rust module.
pub mod ultimate_magic_wordsofpower;
pub mod ultimate_psionics;
pub mod ultimate_wilderness;

/// Identifies which Paizo rule book a table cell or resolved corpus
/// record belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleSetId {
    Crb,
    Apg,
    Acg,
    Bestiary1,
    Arg,
    Pu,
    Uca,
    /// Ultimate Intrigue. SD-28 Epic 24 -- first record family (feats).
    Ui,
    /// Ultimate Equipment. SD-28 Epic 25 -- first record family (equipment).
    Ue,
    /// Ultimate Wilderness. SD-28 Epic 26 -- first record family (feats).
    Uw,
    /// Ultimate Combat. SD-28 Epic 27 -- first record family (feats).
    Uc,
    /// Ultimate Magic. SD-28 Epic 28 -- first record family (feats).
    Um,
    /// Ultimate Psionics. SD-28 Epic 29 -- first record family (feats).
    /// Dreamscarred Press, not Paizo -- the last Ultimate book.
    Upsi,
    /// Bonus Bestiary. SD-29 Epic 5 pilot -- first book to ingest the merged
    /// `monster` + `monster_ability` chassis (`corpus-work-channels.md §9.2`).
    BonusBestiary,
    /// Monster Codex. SD-29 Epic 6 pilot (race-trait lane, `decisions.md §43`)
    /// and Epic 5's second monster book (`rules_tables::monster_codex`, 2
    /// monsters + 3 monster abilities).
    ///
    /// Its `race_trait` records are still served off disk from
    /// `data/corpus/monster_codex/race_trait/` rather than from a compiled
    /// table: `decisions.md §24` rules out the formula interpreter a compiled
    /// race-trait table would need. So this book is the one place where the two
    /// halves of "the engine has compiled this book" are visibly different
    /// kinds of thing -- a compiled monster table and a disk-served race-trait
    /// family -- and `COMPILED_RULE_SETS` answers for both.
    MonsterCodex,
    /// Inner Sea Races. SD-29 Epic 6 round 2 (race-trait lane, extend). Like
    /// `MonsterCodex`, its one ingested family is `race_trait`, served off disk
    /// from `data/corpus/inner_sea_races/race_trait/` rather than from a
    /// compiled table (`decisions.md §24` rules out the formula interpreter a
    /// compiled race-trait table would need). It is the largest single
    /// alternate-racial-trait contribution after ARG's.
    Isr,
    /// Horror Adventures. SD-29 Epic 6 round 3 (race-trait lane, extend).
    /// Like `MonsterCodex` and `Isr`, its one ingested family is `race_trait`,
    /// served off disk from `data/corpus/horror_adventures/race_trait/` rather
    /// than from a compiled table (`decisions.md §24` rules out the formula
    /// interpreter a compiled race-trait table would need).
    ///
    /// **Only the book's main `ha_abilities_race.lst` is ingested.** Its
    /// `support/ha_abilities_race_oa.lst` is loaded by the pcc under
    /// `PRECAMPAIGN:1,INCLUDES=Occult Adventures`, a book this repo has not
    /// ingested, so that file's one further in-scope row is out of this rule
    /// set's scope by construction rather than by omission.
    Ha,
    /// Princes of Darkness -- Book of the Damned, Volume 1. SD-29 Epic 5
    /// extend, round 2 (`rules_tables::book_of_the_damned_volume_1`, 5
    /// monsters + 36 monster abilities). The first `campaign_setting/` book to
    /// carry the monster chassis, and one of only two remaining books in the
    /// lane whose ability rows are ALL owned by a monster row of the same book
    /// (`scripts/classify_monster_ability_rows.py`).
    Botd1,
    /// Lords of Chaos -- Book of the Damned, Volume 2. SD-29 Epic 5 extend,
    /// round 2 (`rules_tables::book_of_the_damned_volume_2`, 4 monsters + 17
    /// monster abilities). The book that found the two-`DESC:`-token row shape
    /// -- see its module doc.
    Botd2,
    /// Inner Sea World Guide. SD-29 Epic 5 extend, round 3
    /// (`rules_tables::inner_sea_world_guide`, 14 monsters + 25 LINKED monster
    /// abilities). The first book in this lane that is not orphan-free -- 5
    /// further ability rows are namespaced to an `iswg_templates.lst` template
    /// no monster row of this book applies, and are deliberately not ingested
    /// (`OPEN_FINDINGS`) rather than shipped as records nothing can reach. Also
    /// the first whose monsters live in two races files with colliding line
    /// numbers, which is why `MonsterStatBlock` carries a `source_file`.
    Iswg,
    /// Core Essentials. SD-29 Epic 6 round 4 (race-trait lane, extend). Like
    /// `MonsterCodex`, `Isr` and `Ha`, its one ingested family is `race_trait`,
    /// served off disk from `data/corpus/core_essentials/race_trait/` rather
    /// than from a compiled table (`decisions.md §24` rules out the formula
    /// interpreter a compiled race-trait table would need).
    ///
    /// **Only the two `<race>_abilities_race_subrace.lst` files are ingested,
    /// and that is a narrower claim than the book id suggests.** PCGen uses
    /// `core_essentials/races/<race>/` as physical storage for the shared
    /// racial-trait files of races that belong to Core Rulebook and Bestiary
    /// 1, and `ingest_races` correctly attributes those to `core_rulebook` and
    /// `beastiary`. What belongs to this rule set is the content no other book
    /// declares: Aasimar's and Tiefling's heritage (subrace) traits -- 16
    /// selectable heritages plus the 48 replacement rows they grant.
    ///
    /// `races/skinwalker/` carries the same shape and is deliberately out of
    /// scope: Skinwalker is not one of the 18 races this project models, so
    /// `RaceCorpus::resolve` would return `None` for its chassis whatever the
    /// ingest wrote.
    Ce,
    /// Inner Sea Combat. SD-29 Epic 7 pilot (companion lane,
    /// `rules_tables::inner_sea_combat`, 4 companion creatures + 6 companion
    /// abilities). The first book whose ONLY ingested family is `companion`, and
    /// therefore the first proof that the companion chassis stands on its own
    /// rather than riding a book some other lane had already compiled.
    Isc,
    /// Inner Sea Intrigue. SD-29 Epic 7 pilot round, extend half
    /// (`rules_tables::inner_sea_intrigue`, 2 familiars + 9 abilities).
    ///
    /// Its 11 units are the ones the race-trait lane handed back: they were
    /// typed `race_trait` by `file_kind`'s `_abilities_race` substring until
    /// that lane's round-2 classifier fix moved them, which left them owned by
    /// no lane at all. This is the lane that owns them.
    Isi,
    /// Bestiary 5. SD-29 Epic 7 round 2 (companion lane, extend;
    /// `rules_tables::bestiary_5`, 35 companion creatures + 22 companion
    /// abilities — all 57 of the book's `companion` units). A "bestiary" with
    /// **zero** monsters — its pcc's `CAMPAIGN` line says "Only Player
    /// Options Implemented" — so this rule set exists for its companion rows
    /// and no other family.
    ///
    /// Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): through
    /// 2026-08-23, `_bestiary_5.pcc:69`'s `support/b5_races_companion_oa.lst`
    /// load under `PRECAMPAIGN:1,Occult Adventures` excluded `Familiar (Brain
    /// Mole)` and `Familiar (Chuspiki)` on the premise that Occult Adventures
    /// was an uningested book (the same ruling `RuleSetId::Ha` records for
    /// the same gate on `race_trait`, `decisions.md §47.2`). That premise is
    /// now false — Occult Adventures is ingested (`RuleSetId::Oa`) — and
    /// `decisions.md §27b` separately overturned this exact exclusion shape,
    /// so both rows are now transcribed.
    B5,
    /// Bestiary 6. SD-29 Epic 7 round 2 (companion lane, extend;
    /// `rules_tables::bestiary_6`, 14 companion creatures + 12 companion
    /// abilities). The second monster-less bestiary, and the book on which both
    /// of the chassis's `named` and `prerace` ownership shapes fire on all
    /// twelve ability rows at once.
    B6,
    /// Bestiary 2. SD-29 Epic 7 round 2 (companion lane, extend;
    /// `rules_tables::bestiary_2`, 15 familiars + 1 ability). The lane's first
    /// FAMILIAR book: its creature rows are `*_races_familiar.lst`
    /// `TYPE:Companion.Familiar.Animal` rows rather than animal companions.
    ///
    /// **This rule set compiles the book's `companion` family and nothing
    /// else.** B2's 782 `monster` / `monster_ability` units belong to the
    /// monster lane (`decisions.md §46`); registering this rule set moves them
    /// from `not-started` to `engine-does-not-hold`, which states the engine's real
    /// relationship to the book more precisely and claims nothing about them.
    B2,
    /// Bestiary 3. SD-29 Epic 5 extend, round 5 (monster lane;
    /// `rules_tables::bestiary_3`, 261 monsters + 27 monster abilities). The
    /// cleanest book the lane has taken: no Product Identity row, no `.COPY=`
    /// delta, and every one of its 261 corpus monster rows ships.
    ///
    /// **This rule set compiles the book's `monster` and `monster_ability`
    /// families.** The book's 799 `race_trait` units are a separate question,
    /// and `rules_tables::bestiary_3`'s header records the finding that 341 of
    /// them are namespaced abilities of the monsters this rule set compiles —
    /// filed under `race_trait` only because `file_kind` reads the first
    /// `TYPE:` segment.
    B3,
    /// Bestiary 4. SD-29 Epic 5 extend, round 6 (monster lane;
    /// `rules_tables::bestiary_4`, 206 monsters + 543 monster abilities) — the
    /// largest reachable book left in the lane when round 6 took it.
    ///
    /// **This rule set compiles the book's `monster` and `monster_ability`
    /// families.** 14 of its 220 corpus monster rows declare `NAMEISPI:YES` and
    /// do not ship; they are unique named personas rather than species, which is
    /// what `ogl-pi-blacklist.md` §2.1's per-record predicate screens on. Their
    /// removal is also why 73 of the book's 225 orphan abilities are orphans —
    /// see `rules_tables::bestiary_4`'s header for both derivations.
    B4,
    /// Inner Sea Bestiary. SD-29 Epic 5 extend, round 7 (monster lane;
    /// `rules_tables::inner_sea_bestiary`, 38 monsters + 152 monster
    /// abilities) — the first `campaign_setting/` book in this lane that is a
    /// bestiary in its own right rather than a setting book with creatures in
    /// it.
    ///
    /// **This rule set compiles the book's `monster` and `monster_ability`
    /// families.** Two of its 40 corpus monster rows do not ship, and the
    /// reason is the one this book contributed to the lane: a monster row's
    /// emitted `ability_keys` array carries the KEYS of the abilities it names,
    /// and seven of this book's ability rows are namespaced to a named deity of
    /// this setting — a `pi_screening::PI_BLACKLIST_TERMS` term, deliberately
    /// not spelled here because `pi-sweep` does not read intent
    /// (`decisions.md §52.5`). The abilities are Product Identity, so the
    /// monsters that name them cannot be emitted either — `decisions.md §57.2`'s cascade
    /// running backwards, from ability to owner. See
    /// `rules_tables::inner_sea_bestiary`'s header.
    Isb,
    /// Inner Sea Gods. SD-29 Epic 5 extend, round 9 (monster lane;
    /// `rules_tables::inner_sea_gods`, 39 monsters + 77 monster abilities) —
    /// the first book in this lane whose corpus rows do NOT all live in the
    /// book's root directory.
    ///
    /// **This rule set compiles the book's `monster` and `monster_ability`
    /// families.** 3 of its 39 monster rows and 16 of its 161 ability rows sit
    /// under `support/`, loaded by `_inner_sea_gods.pcc:68`/`:70` under
    /// `PRECAMPAIGN:1,INCLUDES=Bestiary 4` — a gate this repo satisfies since
    /// round 6 registered `bestiary_4`, so those rows are in scope rather than
    /// excluded. The work inventory records every unit's `source_file` as a
    /// bare basename, so both the transcriber and the generator resolve a
    /// citation by searching the book directory rather than joining onto its
    /// root; see `rules_tables::inner_sea_gods`'s header for the derivation and
    /// for the 16-row `Race Traits ~` bundle finding it records.
    Isg,
    /// Occult Adventures. SD31-E6-F2-003 -- the book's FIRST compiled rule
    /// set of any kind (no prior lane has ingested any of its content), first
    /// record family: 144 base spells (`rules_tables::occult_adventures::
    /// spell_list`).
    ///
    /// **Without this variant, `v06_work_inventory::classify`'s book-level
    /// gate (`engine_book_for` -> `rule_set_for` -> `None`) short-circuits
    /// every one of this book's units to `not-started`/
    /// `no_compiled_rule_set_for_book` before the per-kind `Kind::Spell` arm
    /// ever runs its own `spell_levels` lookup** -- discovered via this
    /// cycle's own guarded regen (a real before/after measurement, not
    /// assumed): shipping `spell_resolver::spell_catalog_rows()` alone left
    /// every OA spell unit reading `not-started` unchanged. This is the
    /// SAME book-level gate `RuleSetId::Um` already crossed for Ultimate
    /// Magic's spells -- UM's variant just happened to exist already, from
    /// an EARLIER, unrelated feat-catalog cycle (SD-28 Epic 28), which is
    /// why this gate's existence was easy to miss until traced.
    Oa,
    /// Mythic Adventures. `SD31-E6-F2-007` -- the book's FIRST compiled rule
    /// set of any kind, first record family: 358 base feat records
    /// (`rules_tables::feat_gap_tables::MYTHIC_ADVENTURES_FEAT_GAP_ROWS`).
    ///
    /// Same book-level gate `RuleSetId::Oa` records above: without this
    /// variant, every `mythic_adventures` corpus unit reads `not-started`/
    /// `no_compiled_rule_set_for_book` regardless of what any per-kind lane
    /// ships.
    ///
    /// **Decision 10's AMENDMENT governs this book's collisions.** A Mythic
    /// feat's corpus `KEY:` is, by game-mechanic design, the SAME key as the
    /// base feat it upgrades (`Accursed Hex (Mythic)` carries
    /// `KEY:Accursed Hex`, exactly the Core Rulebook feat's own key) --
    /// this is not a reprint collision Decision 10's "newest wins" rule
    /// reaches; it is the paradigm variant case the AMENDMENT names
    /// (`feat:weapon_focus` vs its mythic version). Both stay in the
    /// denominator, both ship, and `feats_all.rs`'s collision test verifies
    /// the shape mechanically rather than accepting it on say-so: every
    /// colliding key's Mythic row carries a `PREABILITY:...,CATEGORY=FEAT,
    /// <that same key>` prerequisite, i.e. the corpus itself states you
    /// must already hold the base feat to take its mythic form.
    ///
    /// **Only `ma_feats.lst`'s non-`.MOD` declarations are ingested here.**
    /// The file's 208 `.MOD` rows (e.g. `Android ~ Vision.MOD`) target
    /// records this book files under `race_trait` elsewhere in the corpus
    /// (Android's racial Vision trait, not a feat) -- rescuing them as
    /// standalone `feat`-kind units would misattribute a race_trait
    /// overlay as a new feat. Reported, not ingested here (`OPEN-ISSUES.md`,
    /// out of this card's `race_trait` file territory).
    Mythic,
    /// Adventurer's Guide. SD-31 wave-29 (`lane5-book-onboard` lane) -- this
    /// book's FIRST compiled rule set of any kind, first record family: base
    /// spell records (`rules_tables::adventurers_guide::spell_list::
    /// SPELL_LIST`, transcribed from `ag_spells.lst`).
    ///
    /// Same book-level gate `RuleSetId::Oa`/`RuleSetId::Mythic` record
    /// above: without this variant, every `adventurers_guide` corpus unit
    /// (class_feature, spell, feat, equipment alike) reads `not-started`/
    /// `no_compiled_rule_set_for_book` regardless of what any per-kind
    /// table ships -- `THE-BOX.md` §2.1's G4 finding (`adventurers_guide`
    /// 699 `class_feature` units) is this exact gate.
    AdventurersGuide,
    /// Inner Sea Faiths. SD-32 Gate 0 book-onboarding precondition
    /// (`gate-0-book-onboarding-precondition`, AT-32-G0-003, one of the
    /// four books `epic-breakdown.md` Epic 4 names) -- this book's FIRST
    /// compiled rule set of any kind, first record family: base spell
    /// records (`rules_tables::inner_sea_faiths::spell_list::SPELL_LIST`,
    /// transcribed from `isf_spells.lst`). See
    /// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
    InnerSeaFaiths,
    /// Inner Sea Magic. SD-32 Gate 0 book-onboarding precondition
    /// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this
    /// book's FIRST compiled rule set of any kind, first record family:
    /// base spell records (`rules_tables::inner_sea_magic::spell_list::
    /// SPELL_LIST`, transcribed from `ism_spells.lst`).
    ///
    /// Same book-level gate `RuleSetId::AdventurersGuide` record above:
    /// without this variant, every `inner_sea_magic` corpus unit
    /// (class_feature, spell, feat, equipment, equipment_modifier, class
    /// alike) reads `not-started`/`no_compiled_rule_set_for_book`
    /// regardless of what any per-kind table ships -- this book's 218
    /// `class_feature` units are already ingested corpus-wide
    /// (`data/corpus/inner_sea_magic/class_feature/`) but were unreachable
    /// through this exact gate before this variant existed. See
    /// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
    InnerSeaMagic,
    /// Inner Sea Taverns. SD-32 Gate 0 book-onboarding precondition
    /// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this
    /// book's FIRST compiled rule set of any kind. Unlike the other three
    /// SD-32 Gate 0 books, this one has no `*_spells.lst` at all, so its
    /// first record family is `feat` instead: `istav_feats.lst`'s 9 base
    /// declarations, joined via the same `feat_gap_tables` mechanism
    /// `RuleSetId::Mythic` above uses (an empty `hand_authored_feat_tables`
    /// entry plus a `gen_feat_gap_tables::BOOK_INPUTS` row) -- so, like
    /// `Mythic`, this book has no dedicated `rules_tables::<book>` module
    /// directory of its own.
    InnerSeaTaverns,
    /// Inner Sea Temples. SD-32 Gate 0 book-onboarding precondition
    /// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this
    /// book's FIRST compiled rule set of any kind, first record family:
    /// base spell records (`rules_tables::inner_sea_temples::spell_list::
    /// SPELL_LIST`, transcribed from `istem_spells.lst`). See
    /// `src/bin/ingest_inner_sea_setting_spells.rs` for the ingest path.
    InnerSeaTemples,
}
