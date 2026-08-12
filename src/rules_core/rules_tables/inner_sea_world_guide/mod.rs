//! Inner Sea World Guide (`SOURCESHORT:ISWG`) — `monster` + `monster_ability`.
//!
//! **9 of this book's 14 monster rows and 14 of its 30 ability rows are
//! ingested.** The other 21 units are excluded for two independent, evidenced
//! reasons, and both are findings rather than shortfalls.
//!
//! # 1. Product Identity, declared by the corpus itself
//!
//! Five monster rows carry `NAMEISPI:YES` — PCGen's own per-record marker that
//! the record's NAME is Product Identity:
//!
//! ```text
//! grep -n 'NAMEISPI:YES' iswg_races.lst iswg_races_bestiary.lst
//! iswg_races.lst:13           Daughter of Urgathoa
//! iswg_races.lst:14           Sandpoint Devil
//! iswg_races.lst:16           Treerazer
//! iswg_races_bestiary.lst:13  Boar (Sargavan)
//! iswg_races_bestiary.lst:14  Herd Animal (Storval Aurochs)
//! ```
//!
//! The marker and an independent reading agree, which is the check that makes
//! the marker usable rather than merely present: each name embeds a Golarion
//! deity, town, region or unique NPC — Product Identity under OGL §1(e) on its
//! face, not only by upstream declaration. Three of the excluded ability rows
//! (`Daughter of Urgathoa ~ …`) also match `PI_BLACKLIST_TERMS` outright.
//!
//! **They are dropped, not redacted.** `pi_screening` redacts a `description`;
//! it cannot redact a KEY, and `[redacted PI]` as a monster's key is a record
//! nobody can look up. Reclassifying a term is
//! `docs/governance/ogl-pi-blacklist.md` §3's per-book override — an operator
//! decision, not a transcriber's.
//!
//! **`NAMEISPI:YES` was invisible to this program until this book.** The four
//! books already carrying the chassis contain zero such rows
//! (`grep -c NAMEISPI:YES` over each one's races `.lst` → 0), so nothing
//! already shipped is affected *in this lane*. It is not clean corpus-wide: see
//! this round's receipt for the one shipped record another lane's ingest
//! carries with the marker set.
//!
//! # 2. Orphans — 13 ability rows no monster row of this book claims
//!
//! Rounds 1 and 2 took the books whose every ability row is claimed by a monster
//! row of the same book. From round 3 on there is no such book left:
//!
//! ```text
//! python3 scripts/classify_monster_ability_rows.py inner_sea_world_guide
//! book                    mon  abil row-named prefix ORPHAN
//! inner_sea_world_guide    14    30        25      0      5
//! ```
//!
//! Five are orphans against the whole book (`Nascent Demon Lord ~ …` and
//! `Clockwork ~ …`, namespaced to `iswg_templates.lst` templates this chassis
//! does not model). The other eight lost their owner to the PI screen above —
//! dropping a monster cascades to the abilities only it claimed, which is why
//! the two screens run in that order.
//!
//! An ability with no owner reaches no screen: the catalog renders an ability
//! underneath its monster. Shipping one would be the record-that-loads-and-is-
//! never-shown `decisions.md §44.2` was written about. `not-ingested` is their
//! honest status.
//!
//! # The first book whose monsters live in TWO files, and why that mattered
//!
//! 4 ingested monster rows are in `iswg_races.lst` (alongside this book's player
//! races) and 5 in `iswg_races_bestiary.lst`. Their line numbers **collide**:
//!
//! ```text
//! iswg_races.lst:10          -> Aluum
//! iswg_races_bestiary.lst:10 -> Fennec (Firefoot)
//! ```
//!
//! Before this book, `MonsterStatBlock` carried a `source_line` and
//! `gen_book_cache` took the *file* from a single per-book spec string. Under
//! that model every row of one of these two files would have been
//! citation-checked against the other. [`MonsterStatBlock::source_file`] and the
//! generator's per-record file lookup exist because of this book;
//! `the_two_races_files_carry_colliding_line_numbers` below is the regression.

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
mod tests {
    use super::*;

    /// The corpus unit counts come from `docs/work-inventory.json`, never a line
    /// count over the `.lst`:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='inner_sea_world_guide'
    /// and u['kind']=='monster'))"` -> 14, `monster_ability` -> 30.
    ///
    /// What ships is 9 and 14. Asserting 14 and 30 here would assert that this
    /// book ships five Product Identity names and thirteen records nothing can
    /// reach.
    #[test]
    fn the_book_ships_nine_monsters_and_fourteen_linked_abilities() {
        assert_eq!(monsters().len(), 9);
        assert_eq!(monster_abilities().len(), 14);
    }

    /// Every transcribed ability row is owned by a monster row of this book.
    /// The book has orphans; the point of this test is that none of them got in.
    #[test]
    fn no_shipped_ability_is_an_orphan() {
        for ability in monster_abilities() {
            assert!(
                !ability.owners.is_empty(),
                "{} reaches no monster and would load without ever being shown",
                ability.key
            );
        }
    }

    /// Every owner named by a shipped ability is itself a shipped monster.
    ///
    /// Not implied by the test above: dropping a monster for Product Identity
    /// while leaving an ability pointing at it would satisfy "owners is
    /// non-empty" and still name a creature the catalog cannot render.
    #[test]
    fn every_owner_named_by_a_shipped_ability_is_a_shipped_monster() {
        for ability in monster_abilities() {
            for owner in ability.owners {
                assert!(
                    monsters().iter().any(|m| m.key == *owner),
                    "{} names owner {owner}, which is not a shipped monster of this book",
                    ability.key
                );
            }
        }
    }

    /// The five `NAMEISPI:YES` rows are named individually rather than counted,
    /// so a regeneration that quietly pulls one back in fails here with the name
    /// that returned — and a reader can check each against the cited line.
    #[test]
    fn the_five_product_identity_names_are_not_records() {
        for key in [
            "Daughter of Urgathoa",
            "Sandpoint Devil",
            "Treerazer",
            "Boar (Sargavan)",
            "Herd Animal (Storval Aurochs)",
        ] {
            assert!(
                !monsters().iter().any(|m| m.key == key),
                "{key} carries NAMEISPI:YES in its own corpus row and must not ship"
            );
        }
        for key in [
            "Daughter of Urgathoa ~ Disease",
            "Daughter of Urgathoa ~ Great Claw",
            "Daughter of Urgathoa ~ Spells",
        ] {
            assert!(
                !monster_abilities().iter().any(|a| a.key == key),
                "{key} carries a PI_BLACKLIST_TERMS term in its own key and must not ship"
            );
        }
    }

    /// No shipped value anywhere in this book's two tables carries a Product
    /// Identity term.
    ///
    /// The test above pins the rows this round excluded; this one is the
    /// property those exclusions exist to hold, checked against the live
    /// blacklist rather than against a list of names. A term added to
    /// `PI_BLACKLIST_TERMS` later fails here rather than shipping quietly.
    #[test]
    fn no_shipped_field_carries_a_product_identity_term() {
        use crate::rules_core::pi_screening::PI_BLACKLIST_TERMS;
        let mut haystack = String::new();
        for m in monsters() {
            haystack.push_str(m.key);
            haystack.push(' ');
            haystack.push_str(m.name);
            haystack.push(' ');
            for value in [m.race_type, m.race_subtype, m.monster_class, m.source_page] {
                haystack.push_str(value.unwrap_or(""));
                haystack.push(' ');
            }
            for attack in m.natural_attacks {
                haystack.push_str(attack.name);
                haystack.push(' ');
            }
            for key in m.ability_keys.iter().chain(m.external_ability_refs) {
                haystack.push_str(key);
                haystack.push(' ');
            }
        }
        for a in monster_abilities() {
            haystack.push_str(a.key);
            haystack.push(' ');
            haystack.push_str(a.name);
            haystack.push(' ');
            haystack.push_str(a.description.unwrap_or(""));
            haystack.push(' ');
            haystack.push_str(a.source_page.unwrap_or(""));
            haystack.push(' ');
            for value in a.traits.iter().chain(a.description_variables).chain(a.owners) {
                haystack.push_str(value);
                haystack.push(' ');
            }
        }
        let hits: Vec<&str> = PI_BLACKLIST_TERMS
            .iter()
            .copied()
            .filter(|term| haystack.contains(term))
            .collect();
        assert!(hits.is_empty(), "Product Identity terms in shipped values: {hits:?}");
    }

    /// This book's two races files use overlapping line numbers, so a record's
    /// `source_line` is only meaningful together with its `source_file`. Held as
    /// a test because the whole `source_file` field exists for it.
    #[test]
    fn the_two_races_files_carry_colliding_line_numbers() {
        let at = |file: &str, line: u32| {
            monsters()
                .iter()
                .find(|m| m.source_file == file && m.source_line == line)
                .unwrap_or_else(|| panic!("{file}:{line} is a shipped monster row of this book"))
        };
        assert_eq!(at("iswg_races.lst", 10).key, "Aluum");
        assert_eq!(at("iswg_races_bestiary.lst", 10).key, "Fennec (Firefoot)");

        assert_eq!(
            monsters().iter().filter(|m| m.source_file == "iswg_races.lst").count(),
            4
        );
        assert_eq!(
            monsters()
                .iter()
                .filter(|m| m.source_file == "iswg_races_bestiary.lst")
                .count(),
            5
        );
    }

    /// Verbatim spot-check against `iswg_races.lst:10`, checkable against the
    /// named line rather than merely self-consistent.
    #[test]
    fn the_aluum_matches_its_corpus_row() {
        let aluum = monsters()
            .iter()
            .find(|m| m.key == "Aluum")
            .expect("Aluum is in this book");
        assert_eq!(aluum.source_file, "iswg_races.lst");
        assert_eq!(aluum.source_line, 10);
        assert_eq!(aluum.name, "Aluum");
        assert_eq!(aluum.size, Some("L"));
        assert_eq!(aluum.race_type, Some("Construct"));
        assert_eq!(aluum.challenge_rating, Some("10"));
        assert_eq!(aluum.monster_class, Some("Construct:14"));
        assert_eq!(aluum.source_page, Some("p.306"));
        assert_eq!(aluum.speeds, &[Speed { mode: "Walk", feet: 30 }]);
        assert_eq!(
            aluum.natural_attacks,
            &[NaturalAttack { name: "Slam", damage_dice: Some("2d10") }]
        );
        assert_eq!(
            aluum.ability_keys,
            &[
                "Aluum ~ Immunity to Magic",
                "Aluum ~ Paralysis",
                "Aluum ~ Soul Shriek",
            ]
        );
        assert!(aluum.external_ability_refs.is_empty());
    }
}
