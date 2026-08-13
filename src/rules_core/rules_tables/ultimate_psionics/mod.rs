//! Ultimate Psionics (UPsi), Dreamscarred Press. SD-28 Epic 29
//! (`epic-29-upsi-complete`) -- the last Ultimate book, and the first
//! non-Paizo one, from-scratch book ingest, first slice: the 221-record
//! feat catalog. See `feat_tables`'s own module doc comment for the
//! license-posture check, the catalog, its own-category-enum ruling, and
//! the corpus-shape findings (a source-disabled record excluded on the
//! data team's own annotation, one cross-book collision with Ultimate
//! Combat, one corpus typo corrected and documented, and this book's own
//! DESC:-is-complete convention distinguished from a stub).
//!
//! # `monster_data` — SD-29 Epic 5 extend, round 10
//!
//! The monster / monster-ability chassis (`corpus-work-channels.md §9.2`), and
//! the first non-Paizo book in that lane.
//!
//! **21 monsters + 13 monster abilities = 34 records**, against corpus unit
//! counts of 21 and 79. Derived, never assumed:
//! `python3 scripts/classify_monster_ability_rows.py ultimate_psionics` →
//! `ultimate_psionics  21  79  3  10  66  0  0`, reachable remainder **34**.
//! The transcriber independently reports the same 66 orphans, so both screens
//! agree on composition as well as on the total — unlike `inner_sea_gods`,
//! where they agreed only on the sum (`decisions.md §62.5`).
//!
//! # The cheapest registration this lane has done, and why
//!
//! `RuleSetId::Upsi` has been in `COMPILED_RULE_SETS` since SD-28 E29, serving
//! this book's feats, equipment and archetypes. This is the first book the
//! monster lane has taken whose rule set, corpus directory, work-inventory book
//! entry and `corpus_ingest_diagnostic` row **all already existed** for another
//! family's sake — so the round's cost is a data module, a `MONSTER_BOOKS` row,
//! a `MonsterBookSpec`, two reach claims, a wire code and two diagnostic keys.
//!
//! Both `.lst` files sit at the book **root** (`up_races.lst`,
//! `up_abilities_race.lst`), so round 9's `resolve_book_file` widening
//! (`decisions.md §62.2`) resolves each in one hop and is explicitly **not**
//! load-bearing here. `up_races_apg.lst` also exists in the book directory and
//! contributes **zero** `monster` units; the transcriber takes its unit set
//! from `docs/work-inventory.json`, never from a file glob, so the third file
//! cannot leak in.
//!
//! # Zero Product Identity rows, and the predicate that predicts it
//!
//! `grep -c NAMEISPI:YES up_races.lst up_abilities_race.lst` → `0 0`, and the
//! classifier's own Product Identity screen returns 0 as well. That is what
//! `ogl-pi-blacklist.md` §2.1's **per-record** predicate predicts for this
//! book: its creatures are generic psionic species (Blue, Dromite, Elan,
//! Half-Giant, Maenad, Noral, Ophiduan, Xeph) and construct templates, not
//! named personae. `decisions.md §57.1` is the correction that made this a
//! per-record rather than a per-book-location prediction.
//!
//! # The 66 orphans, and the 2 of them the corpus does own
//!
//! 63 of the 66 are the Astral Construct menu (`Astral_*`) and three
//! namespaced rows (`Energy Touch ~ …`, `Astral Warrior ~ …`, `Horror ~ …`)
//! whose namespace is not a monster key of this book. They stay `not-ingested`,
//! which is their honest status.
//!
//! **Two are not orphans in the corpus** — `Naturally Psionic` and
//! `Psionic Aptitude`. Ten of this book's monster rows carry
//! `ABILITY:Internal|AUTOMATIC|Racial Traits ~ <Race>`, and those
//! `CATEGORY:Internal` bundle rows name both abilities. That is exactly the
//! ownership class `decisions.md §62.4` recorded for `inner_sea_gods` and asked
//! a successor to scan for; round 10 ran the scan corpus-wide and it is **229
//! units across six books**, not the 16 §62.4 measured in one file. See
//! `decisions.md §64.1`. Following the hop is an ownership-pass widening that
//! changes what five already-registered books ship, so this round did not take
//! it — the same reasoning §62.4 used, now with the real number attached.

pub mod archetype_tables;
pub mod equipment_tables;
pub mod feat_tables;

mod monster_data;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
    NaturalAttack, Speed,
};

/// Every monster stat block this book defines, in corpus row order.
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this book defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}

/// Every monster stat block this book defines, in corpus row order.
pub fn monsters() -> &'static [MonsterStatBlock] {
    monsters_static()
}

/// Every monster-ability record this book defines, in corpus row order.
pub fn monster_abilities() -> &'static [MonsterAbilityRecord] {
    monster_abilities_static()
}

#[cfg(test)]
mod monster_tests {
    use super::*;
    use std::collections::HashSet;

    /// What ships is 21 and 13, against corpus unit counts of 21 and 79.
    ///
    /// Every corpus monster row of this book ships — no `NAMEISPI:YES`, no
    /// `.COPY=` delta and no `.MOD` overlay reaches the monster side here.
    #[test]
    fn the_shipped_counts_are_the_reachable_ones() {
        assert_eq!(monsters().len(), 21, "every corpus monster row of this book ships");
        assert_eq!(monster_abilities().len(), 13);
    }

    /// Every record cites one of this book's two `.lst` files, asserted on the
    /// records rather than on the spec — a spec naming a file no record cites
    /// would pass a spec-shaped test while shipping nothing.
    #[test]
    fn every_record_cites_one_of_this_books_files() {
        for monster in monsters() {
            assert_eq!(
                monster.source_file, "up_races.lst",
                "{} cites {}, which is not this book's races file",
                monster.key, monster.source_file
            );
        }
        for ability in monster_abilities() {
            assert_eq!(
                ability.source_file, "up_abilities_race.lst",
                "{} cites {}, which is not this book's abilities file",
                ability.key, ability.source_file
            );
        }
    }

    /// Every shipped ability has at least one owner, and every owner ships.
    #[test]
    fn every_ability_has_a_shipped_owner() {
        let monster_keys: HashSet<&str> = monsters().iter().map(|m| m.key).collect();
        for ability in monster_abilities() {
            assert!(
                !ability.owners.is_empty(),
                "{} ships with no owner -- an orphan reached the table",
                ability.key
            );
            for owner in ability.owners {
                assert!(
                    monster_keys.contains(owner),
                    "{} is owned by {owner}, which this book does not ship",
                    ability.key
                );
            }
        }
    }

    /// The `Racial Traits ~` bundle finding, pinned as an executing test rather
    /// than as prose alone — the countermeasure `decisions.md §62.4` applied to
    /// its own claim, applied here to this book's share of the same class.
    ///
    /// `Naturally Psionic` and `Psionic Aptitude` are owned in the corpus
    /// through a `CATEGORY:Internal` bundle row that neither ownership pass
    /// follows. They do not ship. A later round that widens the pass gets a
    /// failing test telling it this module's header — and the lane's
    /// REAL-ceiling arithmetic in `decisions.md §64.1` — is now stale.
    #[test]
    fn no_internal_bundle_ability_ships_yet() {
        let shipped: Vec<&str> = monster_abilities()
            .iter()
            .filter(|a| matches!(a.key, "Naturally Psionic" | "Psionic Aptitude"))
            .map(|a| a.key)
            .collect();
        assert!(
            shipped.is_empty(),
            "`Racial Traits ~` bundle rows now ship ({shipped:?}) -- this module's header \
             and `decisions.md §64.1`'s 229-unit ceiling arithmetic are stale and must be \
             rewritten"
        );
    }
}
