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
//! Both books in the registry carry rows whose `KEY:` differs from the first
//! column — Bonus Bestiary has 6 (`Caryatid Column ~ Immunity to Magic`),
//! Monster Codex has all 3 of its abilities (`Seru ~ Poison`). Joining on the
//! display name would merge `Poison` with every other book's rule of that name.

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

/// Which of `monster_ability`'s facets a record is, read from the first segment
/// of its corpus `TYPE:` token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterAbilityFacet {
    SpecialAttack,
    SpecialQuality,
}

impl MonsterAbilityFacet {
    /// The wire/display token, spelled exactly as the corpus `TYPE:` segment.
    pub fn corpus_token(self) -> &'static str {
        match self {
            MonsterAbilityFacet::SpecialAttack => "SpecialAttack",
            MonsterAbilityFacet::SpecialQuality => "SpecialQuality",
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
    /// The 1-based abilities-`.lst` line this record was read from.
    pub source_line: u32,
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
    /// Keys into this book's `monster_abilities`, in row order.
    pub ability_keys: &'static [&'static str],
    /// Ability names this row cites that this book does not define.
    pub external_ability_refs: &'static [&'static str],
    /// The 1-based races-`.lst` line this record was read from.
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
    },
    MonsterBook {
        corpus_book: "monster_codex",
        monsters: super::monster_codex::monsters_static(),
        monster_abilities: super::monster_codex::monster_abilities_static(),
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
    /// not, and every defined ability has at least one owner. An orphan means
    /// the link was transcribed wrong in one direction and the catalog would
    /// serve a record no monster row reaches.
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
                assert!(
                    !ability.owners.is_empty(),
                    "{}: {} ({}) is owned by no monster row",
                    book.corpus_book,
                    ability.name,
                    ability.key
                );
                for owner in ability.owners {
                    let monster = book
                        .monster_resolve(owner)
                        .unwrap_or_else(|| panic!("{}: owner {owner:?} is not a monster in this book", book.corpus_book));
                    assert!(
                        monster.ability_keys.contains(&ability.key),
                        "{}: {} claims owner {owner:?}, which does not name it back",
                        book.corpus_book,
                        ability.key
                    );
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
}
