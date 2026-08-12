//! Bestiary 3 (`SOURCESHORT:B3`) — `monster` + `monster_ability` + `companion`.
//!
//! The `companion` family was added by SD-29 Epic 7 round 4 and is documented at
//! the bottom of this file; it draws on four `.lst` files none of the monster
//! text below mentions. The two families share only a `RuleSetId`.
//!
//! **261 of this book's 261 monster rows and 27 of its 40 ability rows ship.**
//! Every exclusion is one class — an ability no monster row of this book owns —
//! and there are 13 of them. No Product Identity row, no `.COPY=` delta, no
//! monster excluded for any reason at all: this is the cleanest book the lane
//! has taken.
//!
//! ```text
//! python3 scripts/classify_monster_ability_rows.py bestiary_3
//! book         mon  abil row-named prefix ORPHAN   PI COPY
//! bestiary_3   261    40         0     27     13    0    0
//! ```
//!
//! Corpus unit counts are the inventory's own, never a line count over the
//! `.lst`:
//! `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
//! print(sum(1 for u in d['units'] if u['book']=='bestiary_3'
//! and u['kind']=='monster'))"` → 261, `monster_ability` → 40.
//!
//! # Zero Product Identity rows
//!
//! `grep -c 'NAMEISPI:YES' b3_races.lst b3_abilities_race.lst` → `0` and `0`,
//! and the term-list screen finds nothing either. `ogl-pi-blacklist.md` §2
//! predicts exactly that for a `roleplaying_game/` bestiary, and Bestiary 2
//! (`decisions.md §52`) read the same way. The absence is held by a test against
//! the LIVE blacklist rather than by the grep above, which is a statement about
//! today.
//!
//! # The first book reached ENTIRELY by the namespaced-prefix link
//!
//! `row-named` is **0** and `prefix` is **27**: every shipped ability of this
//! book is reached because its own `KEY:` is `<Monster> ~ <Ability>` and the
//! prefix is a monster row here. The four books before it lean the other way.
//!
//! That 0 is not an absence of the token. `b3_races.lst` carries **100**
//! `ABILITY:Special Ability|AUTOMATIC|` tokens
//! (`grep -c 'ABILITY:Special Ability|AUTOMATIC|' b3_races.lst` → 100). They
//! name real ability rows in `b3_abilities_race.lst`. None of them is in this
//! book's `monster_ability` key set, because of the following.
//!
//! # The scope finding: 341 of this book's `race_trait` units are monster
//! abilities
//!
//! `b3_abilities_race.lst` holds **838** inventory units, and the inventory
//! splits them **798 `race_trait` / 40 `monster_ability`**. The split is made by
//! `v06_work_inventory::file_kind`, which reads only the **first** `TYPE:`
//! segment:
//!
//! ```text
//! b3_abilities_race.lst:289  TYPE:SpecialQuality.Extraordinary.AdaroRacial      -> monster_ability
//! b3_abilities_race.lst:703  TYPE:AghashRacialAbility.SpecialQuality.Supernatural -> race_trait
//! ```
//!
//! Both rows are a monster's special quality, namespaced to a monster of this
//! book, and differ only in which segment the book happened to write first.
//!
//! Re-derived, one row at a time, against the corpus rather than any doc:
//!
//! | measure | count |
//! |---|---|
//! | `race_trait` units in `b3_abilities_race.lst` | 798 |
//! | …whose `KEY:` is namespaced `<X> ~ <Y>` | 778 |
//! | …whose `<X>` is a **bestiary_3 monster** | **341** |
//! | …and which also carry `SpecialQuality`/`SpecialAttack` in a later `TYPE:` segment | 340 |
//!
//! The command is recorded in this round's `progress.md` receipt.
//!
//! **Why this matters beyond this book.** The race-trait lane's `decisions.md
//! §44.4` counted this book's 799 `race_trait` units among the **2,894** that
//! "belong to races with no chassis", concluding that "no amount of race-trait
//! ingest grounds those" because `RaceCorpus::resolve` returns `None` without a
//! race chassis. That is correct for a player race trait and wrong for these
//! 341: they are not race traits, their owners are monsters, and **this round
//! gives those owners a chassis**. They are reachable by the monster catalog's
//! existing ability rendering, not by a race chassis that will never exist for
//! a Bestiary 3 monster.
//!
//! **They are NOT ingested here, deliberately.** Moving them is a change to
//! `file_kind`'s classification, which redraws the `race_trait` and
//! `monster_ability` denominators for every book in two lanes at once; doing it
//! inside an ingest round would mean this card's own numbers could not be
//! reconciled against the round before it. It is recorded as a finding with its
//! derivation so a successor can price it, which is what `§45.1` asks a round to
//! do before it commits to a book.
//!
//! # 13 orphans
//!
//! An ability reaches a player only underneath its monster, so a row no monster
//! row of this book claims would load and never be shown — the
//! record-that-is-never-seen class `decisions.md §44.2` is about. They stay
//! `not-ingested`, which is their honest status, and are cited by line in
//! `monster_data.rs`'s generated header.
//!
//! One of them is worth naming because it changed the transcriber.
//! `b3_abilities_race.lst:1663` (`Jiang-Shi Vampire`) carries **11** `DESC:`
//! tokens, none gated on `DisplayFullAbility` — an acquired template written as
//! 11 sections. `parse_desc` refuses to pick one by position, and until this
//! book that refusal was a `SystemExit` raised from inside the Product Identity
//! screen, which parses **every** ability row including the ones the orphan pass
//! is about to discard. So a row that was never going to be emitted stopped the
//! transcription of a whole book. The refusal is now deferred: unscreenable rows
//! are collected, and the transcription stops only if one **survives** to be
//! emitted. Nothing is waived — the base creature row this one templates is
//! commented out at `b3_races.lst:293`, so it has no owner and is dropped by the
//! pass that actually applies to it.

mod companion_data;
mod monster_data;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
    NaturalAttack, Speed,
};
pub use super::companion_chassis::{CompanionAbilityRecord, CompanionRecord};

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

    /// What ships is 261 and 27, against corpus unit counts of 261 and 40.
    /// Asserting 40 here would assert that this book ships thirteen records
    /// nothing can reach.
    #[test]
    fn the_book_ships_every_monster_and_twenty_seven_linked_abilities() {
        assert_eq!(monsters().len(), 261);
        assert_eq!(monster_abilities().len(), 27);
    }

    /// The first book in the lane to lose NO monster row: no `NAMEISPI:YES`, no
    /// `PI_BLACKLIST_TERMS` hit, no `.COPY=` delta. Stated as an assertion
    /// rather than a comment so a regeneration that starts dropping monsters
    /// fails here instead of quietly shipping a smaller book.
    #[test]
    fn every_corpus_monster_row_of_this_book_ships() {
        assert_eq!(
            monsters().len(),
            261,
            "all 261 `monster` units of bestiary_3 ship; a drop means a screen changed"
        );
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
    /// Not implied by the test above: an ability pointing at a monster this
    /// book does not ship would satisfy "owners is non-empty" and still name a
    /// creature the catalog cannot render.
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

    /// The 13 orphan rows are pinned individually **by the corpus line each one
    /// is**, so a regeneration that quietly pulls one back in fails here naming
    /// the line that returned.
    ///
    /// `b3_abilities_race.lst:1663` is in this list and is also the row that
    /// made the transcriber defer its `DESC:` refusal — see this module's
    /// header. If a future widening teaches `parse_desc` that shape, this test
    /// still holds: the row is excluded because nothing owns it, not because it
    /// could not be parsed.
    #[test]
    fn the_thirteen_orphan_rows_are_not_records() {
        for line in [
            304u32, 380, 381, 389, 390, 391, 394, 395, 396, 397, 1150, 1448, 1663,
        ] {
            assert!(
                !monster_abilities().iter().any(|a| a.source_line == line),
                "b3_abilities_race.lst:{line} is owned by no monster row of this book and must \
                 not ship"
            );
        }
    }

    /// Every shipped ability of this book is reached by the namespaced-prefix
    /// link rather than by a monster row naming it — the first book in the lane
    /// for which that is true, and the property that makes its `row-named`
    /// column read 0 while `b3_races.lst` carries 100 `ABILITY:Special
    /// Ability|AUTOMATIC|` tokens.
    ///
    /// Those 100 tokens name rows the inventory files under `race_trait`,
    /// because `file_kind` reads only the first `TYPE:` segment. See this
    /// module's header for the 341-unit scope finding that follows from it.
    #[test]
    fn every_shipped_ability_is_reached_by_its_namespaced_key() {
        for ability in monster_abilities() {
            let (prefix, _) = ability
                .key
                .split_once(" ~ ")
                .unwrap_or_else(|| panic!("{} is not a namespaced key", ability.key));
            assert!(
                ability.owners.contains(&prefix),
                "{} is namespaced to {prefix}, which is not among its owners",
                ability.key
            );
        }
    }
}

// ---------------------------------------------------------------------------
// SD-29 Epic 7 round 4 (`SD29-E7-F2-005`) — this book's `companion` family.
//
// The second family Bestiary 3 contributes, and it shares nothing with the
// monsters above but a `RuleSetId`: different `.lst` files, different chassis,
// different catalog screen. `decisions.md §51.5` rules that two lanes
// registering families of one book is the designed path, and the monster lane
// having already compiled `RuleSetId::B3` in `9595bd82` is what made this
// registration free of any scope flip.
//
// **All 85 companion units ship** — 31 creature rows and all 54 ability rows,
// with no `OPEN_FINDINGS` shortfall, drawn from four `.lst` files.
//
// The round expected 19 orphans here and found none. Six creature rows of this
// book carry an `OUTPUTNAME:` that differs from their `KEY:`
// (`KEY:Kyton (Augur)` / `OUTPUTNAME:Augur`, and likewise `Archon (Harbinger)`,
// `Div (Doru)`, `Dragon (Faerie)`, `Oni (Spirit)`, `Rakshasa (Raktavarna)`), and
// their ability rows namespace by the display name — `Augur ~ Spell-Like
// Abilities`. Those six own all 19 rows the classifier had reported as orphans.
// Reading the token is ownership shape 5 (`decisions.md §56.1`).
// ---------------------------------------------------------------------------

/// Every companion creature this book defines, in corpus row order.
pub const fn companions_static() -> &'static [CompanionRecord] {
    companion_data::COMPANIONS
}

/// Every companion ability record this book defines, in corpus row order.
pub const fn companion_abilities_static() -> &'static [CompanionAbilityRecord] {
    companion_data::COMPANION_ABILITIES
}

/// Every companion creature this book defines, in corpus row order.
pub fn companions() -> &'static [CompanionRecord] {
    companions_static()
}

/// Every companion ability record this book defines, in corpus row order.
pub fn companion_abilities() -> &'static [CompanionAbilityRecord] {
    companion_abilities_static()
}
