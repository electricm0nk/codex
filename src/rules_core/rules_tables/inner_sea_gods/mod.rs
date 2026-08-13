//! Inner Sea Gods book-level module. SD-29 Epic 5 extend, round 9 — the
//! monster / monster-ability chassis (`corpus-work-channels.md §9.2`).
//!
//! # What ships, and what the corpus holds
//!
//! **39 monsters + 77 monster abilities = 116 records**, against corpus unit
//! counts of 39 and 161. Derived, never assumed:
//! `python3 scripts/classify_monster_ability_rows.py inner_sea_gods` →
//! `inner_sea_gods  39  161  0  77  81  3  0`, reachable remainder **116**.
//!
//! 116 is exactly what ships, and the two screens reach it by different routes:
//! the classifier counts 81 orphans and 3 Product Identity rows, the transcriber
//! drops 79 orphans and 5 Product Identity rows. Both sum to 84 — the
//! transcriber's Product Identity screen reads the values it is about to
//! **emit** while the classifier reads the row's own key and name, so two rows
//! move between the buckets without moving the total. `decisions.md §57.2`
//! records the same predicate difference on `inner_sea_bestiary`.
//!
//! # The first book in this lane whose rows are not all at the book root
//!
//! Three of the 39 monster rows and sixteen of the 161 ability rows live under
//! `support/`. Derived, never assumed:
//!
//! ```text
//! find ~/workspace/repos/pcgen/data -ipath '*inner_sea_gods*' -name '*races*'
//!   isg_races.lst
//!   isg_abilities_races.lst
//!   support/isg_races_b4.lst
//!   support/isg_abilities_races_b4.lst
//! ```
//!
//! `v06_work_inventory` records every unit's `source_file` as a **bare
//! basename**, and both `MonsterStatBlock::source_file` and
//! `MonsterAbilityRecord::source_file` carry that basename verbatim. For the
//! nine books registered before this one the basename was also the file's
//! location, so joining it onto the book root was correct **by coincidence
//! rather than by rule**. Here it raises `FileNotFoundError` outright — a loud
//! failure, which is the only reason this is a widening rather than a silent
//! mis-citation. Both the transcriber
//! (`transcribe_monster_tables.py::resolve_book_file`) and the generator
//! (`gen_book_cache.rs::resolve_book_file`) now search the book tree and refuse
//! two cases rather than resolving them: a basename found nowhere, and a
//! basename found in more than one place. No book in the corpus trips the
//! second — verified over all fourteen books this lane has considered, every one
//! of which has zero duplicate `.lst` basenames — so the check is what makes the
//! first one that does fail loudly instead of shipping the wrong rules text.
//!
//! **The `support/` pair is neither unconditionally loaded nor out of scope.**
//! `_inner_sea_gods.pcc:68` and `:70` gate both on
//! `PRECAMPAIGN:1,INCLUDES=Bestiary 4`, and round 6 registered `bestiary_4`, so
//! this repo satisfies the gate. That is the `PRECAMPAIGN` hazard
//! `loop-instruction.md`'s corpus shape notes describe, read from the **pcc load
//! line** rather than from inside the `.lst`: `grep PRECAMPAIGN` over the two
//! `.lst` files themselves returns 0, so a lane that checks the file for its own
//! gate concludes, wrongly, that it is ungated.
//!
//! # The 16 `Race Traits ~` bundle rows: a scope finding, not an ingest
//!
//! **Zero of the sixteen `support/isg_abilities_races_b4.lst` ability rows
//! ship**, and the reason is worth more than the rows. The classifier and the
//! transcriber both call them orphans — no monster row of this book owns them —
//! and by each screen's own predicate that is correct. But the corpus **does**
//! state their owner, one hop further out than either screen looks:
//!
//! ```text
//! support/isg_races_b4.lst:6    The First Blade
//!     ABILITY:Internal|AUTOMATIC|Race Traits ~ First Blade
//! support/isg_abilities_races_b4.lst:8   Race Traits ~ First Blade
//!     CATEGORY:Internal
//!     ABILITY:Special Ability|AUTOMATIC|…|First Blade ~ Powerful Blows (Slam)
//!         |First Blade ~ Regeneration|First Blade ~ Bladed Slam|…
//! ```
//!
//! The monster row names a `CATEGORY:Internal` **bundle** row, and that bundle
//! row names the individual abilities. The transcriber's row-named pass reads
//! `ABILITY:Special Ability|AUTOMATIC|` tokens **on monster rows**, and its
//! prefix pass matches an ability's namespace against a monster **key** — and
//! the namespaces here are the creature's short name (`First Blade`,
//! `Skein Steward`, `Ahmuuth`) while the monster keys are longer
//! (`The First Blade`, `Steward of the Skein`, `Psychopomp (Ahmuuth)`). So
//! neither pass reaches them, and all sixteen fall out as orphans.
//!
//! This round did **not** widen the ownership pass to follow the bundle row, and
//! the reason is deliberate rather than an omission: this round already widened
//! file resolution, and a second mechanism change in the same gate is how a
//! round goes red. It is recorded here with its corpus lines so the next round
//! can execute it rather than re-derive it.
//!
//! **The consequence for the lane's ceiling is that 16 units this round reports
//! as unreachable are in fact reachable**, by a mechanism the corpus states and
//! no screen currently follows. The lane's REAL ceiling is understated by at
//! least that much, and the same shape may exist in other books: the predicate
//! to look for is an `ABILITY:Internal|AUTOMATIC|` token on a monster row.
//!
//! ## CORRECTED by round 10 — the class is 229, and 79 of them are this book's
//!
//! The scan above asked for was run (`scripts/scan_monster_ability_bundle_rows.py`,
//! checked in; `decisions.md §64.1`). **Sixteen is the
//! `support/isg_abilities_races_b4.lst` subset, not this book's figure.** The
//! hop reaches **79 of this book's 81 remaining orphans** — 63 of them in
//! `isg_abilities_races.lst`, at the book root, which the section above does
//! not mention at all — and **229 rows across six books**, five of them already
//! registered.
//!
//! The correction is left beside the original rather than overwriting it,
//! because the shape of the error is the transferable part: the finding was
//! stated as a property of the *file being read* rather than of the corpus, and
//! that is the third time this lane has paid for exactly that (`§60.3`,
//! `§62.1`, `§64.1`). `no_support_directory_ability_ships_yet` below guards the
//! subset, not the class.
//!
//! # Provenance
//!
//! `_inner_sea_gods.pcc:17` declares `ISOGL:YES`; the pcc carries 18 `COPYRIGHT`
//! lines and a real 9,547-byte `OGL.txt` sits beside it. **Zero** rows of any of
//! the four `.lst` files declare `NAMEISPI:YES`
//! (`grep -c NAMEISPI:YES isg_races.lst isg_abilities_races.lst
//! support/isg_races_b4.lst support/isg_abilities_races_b4.lst` → `0 0 0 0`).
//! The 5 ability rows the transcriber's screen drops are dropped for a
//! blacklisted deity name in an emitted value, which is exactly what
//! `ogl-pi-blacklist.md` §2.1's per-record predicate predicts for a
//! `campaign_setting/` book about deities. The records ship `License::Ogl` like
//! every other book in this registry.

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
    use std::collections::HashSet;

    /// What ships is 39 and 77, against corpus unit counts of 39 and 161.
    ///
    /// The monster count is the whole corpus set — this is the first book in
    /// the registry to lose no monster row at all to any screen: no
    /// `NAMEISPI:YES`, no `.COPY=` delta, no `.MOD` overlay, and no cascade from
    /// a Product Identity ability it names.
    #[test]
    fn the_shipped_counts_are_the_reachable_ones() {
        assert_eq!(monsters().len(), 39, "every corpus monster row of this book ships");
        assert_eq!(monster_abilities().len(), 77);
    }

    /// The three `support/` monster rows ship, and they are the reason this
    /// book needed file resolution at all.
    ///
    /// Asserted on the records rather than on the spec: a spec listing a file
    /// no record cites would pass a spec-shaped test while shipping nothing.
    #[test]
    fn the_support_directory_monsters_ship() {
        let from_support: Vec<&str> = monsters()
            .iter()
            .filter(|m| m.source_file == "isg_races_b4.lst")
            .map(|m| m.key)
            .collect();
        assert_eq!(
            from_support.len(),
            3,
            "3 monster rows come from support/isg_races_b4.lst, got {from_support:?}"
        );
        for key in ["The First Blade", "Steward of the Skein", "Psychopomp (Ahmuuth)"] {
            assert!(
                from_support.contains(&key),
                "{key} is a support/isg_races_b4.lst row and must ship: {from_support:?}"
            );
        }
    }

    /// Every ability record names a file this book actually has, and every
    /// monster does too.
    ///
    /// This is the property `MonsterBookSpec::abilities_lsts` is checked
    /// against in the generator; asserting it here means a bad transcription
    /// fails in the library's own tests rather than only when the cache is
    /// regenerated.
    #[test]
    fn every_record_cites_one_of_this_books_files() {
        let races: HashSet<&str> = ["isg_races.lst", "isg_races_b4.lst"].into_iter().collect();
        let abilities: HashSet<&str> =
            ["isg_abilities_races.lst", "isg_abilities_races_b4.lst"].into_iter().collect();
        for monster in monsters() {
            assert!(
                races.contains(monster.source_file),
                "{} cites {}, which is not a races file of this book",
                monster.key,
                monster.source_file
            );
        }
        for ability in monster_abilities() {
            assert!(
                abilities.contains(ability.source_file),
                "{} cites {}, which is not an abilities file of this book",
                ability.key,
                ability.source_file
            );
        }
    }

    /// Every shipped ability has at least one owner, and every owner ships.
    ///
    /// The orphan screen is what keeps the first half true; this asserts it on
    /// the table rather than trusting the transcriber's report of its own work.
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

    /// The `Race Traits ~` bundle finding, pinned as an executing test rather
    /// than as prose alone.
    ///
    /// The header explains why the sixteen `support/isg_abilities_races_b4.lst`
    /// ability rows do not ship and records the corpus lines that state their
    /// real owner. This asserts the *state* that explanation describes, so that
    /// a later round which widens the ownership pass to follow the bundle row
    /// is told by a failing test that the header is now stale — the failure
    /// mode a comment alone cannot catch.
    #[test]
    fn no_support_directory_ability_ships_yet() {
        let shipped: Vec<&str> = monster_abilities()
            .iter()
            .filter(|a| a.source_file == "isg_abilities_races_b4.lst")
            .map(|a| a.key)
            .collect();
        assert!(
            shipped.is_empty(),
            "support/isg_abilities_races_b4.lst rows now ship ({shipped:?}) -- the \
             `Race Traits ~` bundle finding in this module's header is stale and must be \
             rewritten, along with the lane's REAL-ceiling arithmetic that depends on it"
        );
    }
}
