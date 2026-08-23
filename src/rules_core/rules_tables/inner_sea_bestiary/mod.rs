//! Inner Sea Bestiary book-level module. SD-29 Epic 5 extend, round 7 — the
//! monster / monster-ability chassis (`corpus-work-channels.md §9.2`).
//!
//! # What ships, and what the corpus holds
//!
//! **38 monsters + 152 monster abilities = 190 records**, against corpus unit
//! counts of 40 and 190. Derived, never assumed:
//! `python3 scripts/classify_monster_ability_rows.py inner_sea_bestiary` →
//! `inner_sea_bestiary  40  190  157  0  26  7  0`.
//!
//! Its link shape is the ROW-NAMED one — 157 of the 190 ability rows are named
//! by an `ABILITY:Special Ability|AUTOMATIC|` token on a monster row and **zero**
//! reach through the namespaced-prefix pass, the exact inverse of
//! `rules_tables::bestiary_3`'s `row-named 0 / prefix 27`. Both shapes have been
//! in the transcriber since round 2; this is the first book since the Bonus
//! Bestiary pilot that exercises only the first.
//!
//! # The classifier's reachable remainder over-reports this book by 7
//!
//! `classify_monster_ability_rows.py` reports `reachable = 230 − 26 − 7 − 0 =
//! 197`; what ships is **190**. The residue is exactly 7 and it is not a
//! transcription shortfall — it is a **difference of Product Identity
//! predicate** between the two screens, and the direction matters:
//!
//! * The classifier screens a monster row's own **key and name** (plus
//!   `NAMEISPI:`). Neither of the two dropped rows carries a blacklisted term in
//!   either.
//! * The transcriber screens the values it is about to **emit**, which for a
//!   monster include the ability keys the row NAMES. Two rows of this book name
//!   seven `ABILITY:Special Ability|AUTOMATIC|` keys that are themselves Product
//!   Identity — the ability's namespace is a named deity of this setting, one of
//!   `pi_screening::PI_BLACKLIST_TERMS` — so the monster cannot be emitted
//!   without emitting that term in its own `ability_keys` array. The term is
//!   NOT spelled here: `pi-sweep` rejects a Product Identity term anywhere under
//!   `rules_tables/` regardless of the sentence around it, and it does not read
//!   intent (`decisions.md §52.5`). The corpus line is the better identifier
//!   anyway — see `the_product_identity_rows_are_not_records` below.
//!
//! So this book runs `decisions.md §57.2`'s cascade **backwards**: there, a
//! dropped Product Identity monster orphaned 73 well-formed abilities; here,
//! Product Identity *abilities* drop their own owning monsters, and those two
//! monsters' 5 remaining abilities are orphaned in turn. `2 + 5 = 7`, which
//! closes the residue with none left over.
//!
//! That is a correction to the lane's ceiling instrument, not to this book:
//! `reachable remainder` is an upper bound wherever a book's monster rows name a
//! Product Identity ability. Recorded rather than patched into the script — the
//! measurement it makes is still the right one for ranking a queue, and
//! narrowing it needs a deliberate pass over every book, not a round's side
//! effect (`decisions.md §58.2`).
//!
//! # The continuation `DESC:` shape, widened deliberately
//!
//! Three of this book's shipped rows carry two ungated `DESC:` tokens that are
//! one description split across tokens, each continuation beginning with a
//! space. `scripts/transcribe_monster_tables.py::parse_desc` refused them until
//! this round and now joins them in row order; the predicate is narrow — every
//! token must carry no pipe entry at all — so the three rows of this same file
//! that state *alternatives* under `%N` variables
//! (`isb_abilities_race.lst:203`, `:204`, `:206`) are still refused rather than
//! silently joined. See that function's doc comment for the derivation.
//!
//! # Provenance
//!
//! `inner_sea_bestiary.pcc` carries the book's own `COPYRIGHT` block and the
//! campaign-setting `OGL.txt`; the records ship `License::Ogl` like every other
//! book in this registry.

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

    /// What ships is 38 and 152, against corpus unit counts of 40 and 190.
    /// Asserting 40 here would assert that this book ships two monsters whose
    /// own emitted values carry a Product Identity name.
    #[test]
    fn the_book_ships_thirty_eight_monsters_and_one_hundred_fifty_two_abilities() {
        assert_eq!(monsters().len(), 38);
        // 152 owned + 28 owner-less (`decisions.md §20`, no_record-to-zero
        // wave 2 follow-on) = 180. The owner-less count is pinned separately
        // below (`every_owner_less_ability_is_a_named_and_pinned_non_reach`).
        let owned = monster_abilities()
            .iter()
            .filter(|a| !a.owners.is_empty())
            .count();
        assert_eq!(owned, 152);
        assert_eq!(monster_abilities().len(), 180);
    }

    /// The shipped total is the classifier's `reachable remainder` **minus the
    /// 7 units its narrower Product Identity predicate does not subtract** —
    /// `230 − 26 orphans − 7 PI − 0 .COPY=` → 197, less 2 monster rows dropped
    /// for the Product Identity ability keys they name and the 5 abilities
    /// orphaned by losing them.
    ///
    /// Spelled as the arithmetic rather than as a bare `190` so a divergence
    /// says which of the two screens moved. `bestiary_4`'s equivalent test
    /// (`the_shipped_total_is_the_classifiers_reachable_remainder`) closes with
    /// no residue because no monster row of that book names a Product Identity
    /// ability; this one is the first that does.
    #[test]
    fn the_shipped_total_is_the_classifiers_reachable_remainder_less_the_cascade() {
        // The classifier's fourth term, `.COPY=`, is 0 for this book and is
        // therefore ABSENT here rather than written as `- 0`: `clippy`'s
        // deny-by-default `identity_op` rejects the no-op subtraction, and a
        // gate warning is a worse way to carry the fourth term than this
        // comment is.
        let classifier_reachable = 230 - 26 - 7;
        let cascade = 2 + 5;
        let owned = monster_abilities()
            .iter()
            .filter(|a| !a.owners.is_empty())
            .count();
        assert_eq!(monsters().len() + owned, classifier_reachable - cascade);
        assert_eq!(monsters().len() + owned, 190);
        // 190 owned/reachable + 28 owner-less (`decisions.md §20`) = 218, the
        // book's real total shipped count.
        assert_eq!(monsters().len() + monster_abilities().len(), 218);
    }

    /// **Superseded `decisions.md §20` (no_record-to-zero wave 2 follow-on).**
    /// The 28 rows no shipped monster row of this book claims now SHIP with
    /// `owners: &[]`, and this test pins the EXACT set of records that carry
    /// one — a silent new arrival OR a silent disappearance both fail here,
    /// by name. `list_monster_catalog` never walks these directly (only a
    /// monster's own `ability_keys`), so shipping them does not surface a
    /// stub; each key is pinned separately, by name, in `reach_gate.rs::
    /// UNREACHED_RECORD_FINDINGS` under
    /// `("inner_sea_bestiary", "monster_abilities")` as a proven non-reach,
    /// not a silent claim of reachability.
    #[test]
    fn every_owner_less_ability_is_a_named_and_pinned_non_reach() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut unowned: Vec<&str> = monster_abilities()
            .iter()
            .filter(|a| a.owners.is_empty())
            .map(|a| a.key)
            .collect();
        unowned.sort_unstable();

        assert_eq!(
            unowned.len(),
            28,
            "the number of owner-less (unreachable-by-design) monster_ability records \
             changed — re-derive this pin from a real \
             `scripts/transcribe_monster_tables.py inner_sea_bestiary` run, and update the \
             matching `reach_gate.rs::UNREACHED_RECORD_FINDINGS` entry to the same key set"
        );

        let mut hasher = DefaultHasher::new();
        unowned.hash(&mut hasher);
        let digest = hasher.finish();
        assert_eq!(
            digest, 0xfb2e_b118_c0c5_f992,
            "the owner-less key SET changed (same count, different members) — re-derive and \
             update `reach_gate.rs::UNREACHED_RECORD_FINDINGS` to match exactly"
        );
    }

    /// Every owner named by a shipped ability is itself a shipped monster —
    /// the stronger form, which this book needs for the same reason
    /// `bestiary_4` does: two of its monster rows exist in the corpus and are
    /// deliberately not shipped.
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

    /// The Product Identity rows are not records, pinned by the corpus line
    /// each one is rather than by name — naming them in source is what
    /// `decisions.md §52.5` records turning a concurrent lane's `pi-sweep` red,
    /// and `pi-sweep` does not read intent.
    ///
    /// Two monster lines and the seven ability lines whose namespace carries a
    /// deity's proper name.
    #[test]
    fn the_product_identity_rows_are_not_records() {
        for line in [78u32, 79] {
            assert!(
                !monsters().iter().any(|m| m.source_line == line),
                "isb_races.lst:{line} is Product Identity and must not ship"
            );
        }
        for line in [312u32, 313, 314, 315, 316, 317, 318] {
            assert!(
                !monster_abilities().iter().any(|a| a.source_line == line),
                "isb_abilities_race.lst:{line} is Product Identity and must not ship"
            );
        }
    }

    /// Not one shipped record carries a term from the LIVE Product Identity
    /// blacklist. The line pins above state what this round screened; this
    /// states what the crate screens now, and the two catch different things.
    #[test]
    fn no_shipped_record_carries_a_product_identity_term() {
        for term in crate::rules_core::pi_screening::PI_BLACKLIST_TERMS {
            for monster in monsters() {
                assert!(
                    !monster.name.contains(term) && !monster.key.contains(term),
                    "shipped monster {} carries blacklisted term {term}",
                    monster.key
                );
            }
            for ability in monster_abilities() {
                assert!(
                    !ability.name.contains(term) && !ability.key.contains(term),
                    "shipped ability {} carries blacklisted term {term}",
                    ability.key
                );
                for owner in ability.owners {
                    assert!(
                        !owner.contains(term),
                        "shipped ability {} names owner carrying blacklisted term {term}",
                        ability.key
                    );
                }
            }
        }
    }

    /// The `row-named 157 / prefix 0` split the classifier reports is NOT
    /// observable from this table, and asserting it from here would be a
    /// mis-stated test rather than a weak one.
    ///
    /// A first draft asserted "no shipped ability has its namespace prefix as
    /// its only owner", reading that as the prefix pass having contributed
    /// nothing. It fails at **96 of 152** rows, and the rows are all correct:
    /// when a monster row names an ability whose namespace is that same
    /// monster, the row-named pass already recorded the owner and the prefix
    /// pass adds nothing, leaving `owners == [prefix]` — indistinguishable in
    /// the table from a prefix-only reach. What is assertable is the property
    /// the split actually guarantees, which the two tests above already carry:
    /// every ability has an owner, and every owner ships. Recorded here so the
    /// next book does not re-derive the same wrong test.
    ///
    /// The three continuation-`DESC:` rows ship their SECOND sentence, not just
    /// their first. Before round 7 widened `parse_desc` these rows stopped the
    /// transcription outright; taking the first token alone would have shipped
    /// the trigger and dropped the effect.
    #[test]
    fn a_continuation_desc_row_ships_both_of_its_tokens() {
        let record = monster_abilities()
            .iter()
            .find(|a| a.source_line == 227)
            .expect("isb_abilities_race.lst:227 ships");
        let text = record.description.expect("the row carries DESC: text");
        assert!(
            text.contains("blood and pus spews forth from the wound.")
                && text.contains("20-foot radius"),
            "the record serves only part of its description: {text:?}"
        );
    }
}
