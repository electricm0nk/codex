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
//! # A book is registered when EVERY one of its ability rows has an owner
//!
//! Same predicate `monster_chassis` states, for the same reason: an ability row
//! no creature row claims is a record that loads and is never shown.
//! `scripts/classify_companion_rows.py` classifies a candidate book's rows before
//! a round commits to it, per `decisions.md §45.1`. Run corpus-wide it reports
//! **808** orphan ability rows of the kind's 1,696 units — **765** of which no
//! creature row in *any* book claims — so the lane's reachable ceiling is 888
//! under the per-book predicate, not 1,696. That is a ceiling, not a backlog.
//!
//! # The three ownership shapes, every one stated by the corpus
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
    /// `BONUS:STAT` tokens the advancement package applies. Adjustments, never
    /// scores — see [`StatAdjustment`].
    pub stat_adjustments: &'static [StatAdjustment],
    pub source_page: Option<&'static str>,
    /// Every creature in this book whose row, `PRERACE:` gate or namespaced key
    /// claims this ability. Non-empty for every registered book's every row.
    pub owners: &'static [&'static str],
    /// The 1-based abilities-`.lst` line this record was read from.
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
    /// `BONUS:STAT` tokens on the creature's own row. Adjustments, never scores.
    pub stat_adjustments: &'static [StatAdjustment],
    /// `BONUS:VAR|AC_Natural_Armor|<n>|TYPE=Base`, when the row carries one.
    pub natural_armor: Option<i16>,
    pub source_page: Option<&'static str>,
    /// Keys into this book's `companion_abilities`, in creature-row order.
    pub ability_keys: &'static [&'static str],
    /// Ability names this row cites that this book does not define.
    pub external_ability_refs: &'static [&'static str],
    /// The 1-based races-`.lst` line this record was read from.
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
/// The eight below are every book with **zero** orphan ability rows that this
/// lane has reached, derived rather than assumed:
/// `python3 scripts/classify_companion_rows.py inner_sea_combat monster_codex
/// inner_sea_intrigue horror_adventures bestiary_5 bestiary_6 bestiary_2
/// bestiary`.
///
/// Round 2's three (`bestiary_5`, `bestiary_6`, `bestiary_2`) were held back
/// from round 1 because each needs its own `RuleSetId`, whose scope flip moves
/// several hundred units of OTHER kinds from `not-started` to `not-ingested`.
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
];

/// The registered book with this corpus directory id.
pub fn companion_book(corpus_book: &str) -> Option<&'static CompanionBook> {
    COMPANION_BOOKS.iter().find(|b| b.corpus_book == corpus_book)
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    !ability.owners.is_empty(),
                    "{}: {} ({}) is owned by no creature row and would load without ever \
                     being shown",
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
    /// is. Inner Sea Intrigue's `ClockworkFamiliarInstalledItem` rows are the
    /// three that exercise this.
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
            unmodelled, 3,
            "expected exactly Inner Sea Intrigue's three ClockworkFamiliarInstalledItem rows \
             to carry no modelled facet; a change here means a book's shape moved"
        );
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
}
