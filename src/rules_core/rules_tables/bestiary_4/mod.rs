//! Bestiary 4 (`SOURCESHORT:B4`) — `monster` + `monster_ability` + `companion`.
//!
//! The `companion` family was added by SD-29 Epic 7 round 5 and is documented at
//! the bottom of this file; it draws on three `.lst` files none of the monster
//! text below mentions. The two families share only a `RuleSetId`.
//!
//! **206 of this book's 220 monster rows and 543 of its 768 ability rows ship**
//! — 749 records, the largest reachable book left in the lane when round 6 took
//! it.
//!
//! ```text
//! python3 scripts/classify_monster_ability_rows.py bestiary_4
//! book         mon  abil row-named prefix ORPHAN   PI COPY
//! bestiary_4   220   768         0    543    225   14    0
//! ```
//!
//! `206 + 543 = 749` is exactly the classifier's `reachable remainder`
//! (`988 − 225 − 14 − 0`), so what ships and what the lane's ceiling says
//! should ship are the same number, derived two ways.
//!
//! Corpus unit counts are the inventory's own, never a line count over the
//! `.lst`:
//! `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
//! print(sum(1 for u in d['units'] if u['book']=='bestiary_4'
//! and u['kind']=='monster'))"` → 220, `monster_ability` → 768.
//!
//! # 14 Product Identity rows — and why the rule that predicted zero was right
//!
//! This is the **first `roleplaying_game/` bestiary in the lane that carries
//! any** `NAMEISPI:YES` row: `grep -c NAMEISPI:YES b4_races.lst
//! b4_abilities_race.lst` → `14` and `0`.
//!
//! Rounds 4 and 5 each recorded `ogl-pi-blacklist.md` §2's prediction in a
//! **book-location** form — "a `roleplaying_game/` bestiary carries zero PI
//! rows" — and each was right about its own book. Bestiary 2 and Bestiary 3
//! really do carry none. But the location form is not what the blacklist says,
//! and this book is where the difference shows.
//!
//! §2.1's predicate is **per record**: a *generic SRD species name* ("Goblin",
//! "Owlbear") is presumptively Open Game Content; the blacklist entry is for
//! "*non-bestiary* uses of a monster's proper name (e.g. a unique named NPC
//! monster)". All 14 rows here are unique named personas, not species:
//!
//! | rows | `b4_races.lst` lines | what they are |
//! |---|---|---|
//! | 3 | 40, 41, 42 | Demon Lords — Dagon, Kostchtchie, Pazuzu |
//! | 3 | 66, 67, 68 | Empyreal Lords — Cernunnos, Korada, Vildeis |
//! | 3 | 110, 111, 112 | Great Old Ones — Bokrug, Cthulhu, Hastur |
//! | 3 | 139, 140, 141 | Kaiju — Agyra, Bezravnis, Mogaru |
//! | 2 | 219, 222 | Spawn of Yog-Sothoth, Star-Spawn of Cthulhu |
//!
//! Not one generic species among them. The book-location form of the rule would
//! have shipped all 14; the per-record form the corpus itself declares drops
//! them. The prediction is **refined, not contradicted** — and the transferable
//! point is that two rounds validated a predicate only where it happened to be
//! right, which is the failure shape this lane keeps recording.
//!
//! # 225 orphans, 73 of which this round's own PI screen created
//!
//! An ability reaches a player only underneath its monster, so a row no shipped
//! monster of this book claims would load and never be shown — the
//! record-that-is-never-seen class `decisions.md §44.2` is about.
//!
//! The 225 split into two causes, and the split is derived rather than assumed:
//!
//! | class | count |
//! |---|---|
//! | orphans in their own right — no monster row ever named them | 152 |
//! | **cascade** — namespaced to one of the 14 dropped PI monsters | **73** |
//! | total | 225 |
//!
//! `Demon Lord (Dagon) ~ Breath Weapon` (`b4_abilities_race.lst:439`) is a
//! cascade row: it is perfectly well-formed and owned, and it is unreachable
//! only because its owner is Product Identity. That reproduces, by an
//! independent route, the `152 → 225` figure the round-4 queue note carried.
//!
//! **83 of the 152 live in a second file this book ships nothing from.**
//! `b4_abilities_races_ce.lst` contributes 83 orphan rows and 0 shipped records,
//! while all 543 shipped abilities come from `b4_abilities_race.lst`. That is
//! not an artifact of the transcriber reading one file: it takes its unit set
//! from the inventory, across every source file, exactly as it must for
//! Inner Sea World Guide's 7/7 monster split. The rows are generic reusable
//! abilities that no monster row names — checked at the point of the confident
//! claim rather than inferred:
//! `grep -c 'ABILITY:Special Ability|AUTOMATIC|Immunity to Calm Emotions'
//! b4_races.lst` → `0`, and the file's own second line reads
//! `#This should probably go into ce_abilities_race.lst`.
//!
//! # The `§55.1` measurement round 5 asked a successor to run
//!
//! `decisions.md §55.1` found that `v06_work_inventory::file_kind` types a row
//! by its **first** `TYPE:` segment only, so a monster's special quality lands
//! in `race_trait` or `monster_ability` depending on which segment the book
//! happened to write first — mis-filing units the lane's denominator then never
//! counts. Round 5 measured **bestiary_3** and asked that the same measurement
//! be run on `bestiary_4`, `bestiary` and `inner_sea_bestiary` "before anyone
//! treats 1,767 as the lane's true size". Round 6 ran it. See this round's
//! `progress.md` receipt for the command.
//!
//! | book | `race_trait` units | namespaced | **owned by a monster of the book** |
//! |---|---|---|---|
//! | `bestiary_4` | 86 | 79 | **61** |
//! | `bestiary` | 21 | 19 | **9** |
//! | `inner_sea_bestiary` | 4 | 3 | **2** |
//! | `bestiary_3` (round 5's book) | 799 | 779 | **625**, not 341 — see below |
//!
//! **The answer to round 5's question is that the understatement is almost
//! entirely bestiary_3's.** The three books it named contribute **72** mis-filed
//! units between them, so 1,767 is very nearly right for the rest of the lane
//! and does not need re-drawing before work continues.
//!
//! **Round 5's own 341 is corrected to 625 here**, and the correction is a
//! predicate, not an arithmetic slip. 341 is reproduced *exactly* under round
//! 5's predicate — match the namespace prefix against a monster's `KEY:` — so
//! the figure was right for what it measured. But this corpus namespaces an
//! ability by the monster's **display name** while the monster's `KEY:` carries
//! a taxonomic prefix, which is the `key-differs-from-name` trap the trap report
//! raises 1,009 times on this very book's sibling:
//!
//! ```text
//! race_trait `Aghash ~ …`      -> monster KEY `Div (Aghash)`
//! race_trait `Androsphinx ~ …` -> monster KEY `Sphinx (Androsphinx)`
//! race_trait `Bone Golem ~ …`  -> monster KEY `Golem (Bone)`
//! ```
//!
//! Matching on `KEY:` alone misses every monster whose key differs from its
//! name — 284 further units in bestiary_3. Name-matching is the weaker
//! predicate in general and was checked before being used: across all four books
//! exactly **one** monster display name is ambiguous (`Unfettered Eidolon`, twice
//! in bestiary_3), and none of the other three books has any.
//!
//! Still **not** reclassified here, for the reason round 5 gave and this round
//! agrees with: moving them changes `file_kind`, which redraws the `race_trait`
//! and `monster_ability` denominators for every book in two lanes at once.

mod companion_data;
mod monster_data;

pub use super::companion_chassis::{CompanionAbilityRecord, CompanionRecord};
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

    /// What ships is 206 and 619, against corpus unit counts of 220 and 768
    /// (+ `b4_abilities_races_ce.lst`'s own rows, added `SD-32 card 11` T9
    /// onboarding). Asserting 220 here would assert that this book ships
    /// fourteen Product Identity personas; asserting the raw `.lst` total
    /// would assert it ships records nothing can reach.
    ///
    /// 543 -> 577 (SD31-W21-MONSTER-001, +34): the `CATEGORY:Internal`
    /// bundle-row ownership hop (`transcribe_monster_tables.py::
    /// find_internal_bundle_ability_refs`) resolved 34 of this book's
    /// previously-orphaned ability rows, owned only indirectly through a
    /// bundle row a monster's `ABILITY:Internal|AUTOMATIC|` token names.
    ///
    /// 577 -> 619 (SD-32 card 11, T9 onboarding, `decisions.md §19` sign-off
    /// / `§17` generic-pass discipline, +42): `gen_book_cache.rs`'s
    /// `MonsterBookSpec` for this book named only `b4_abilities_race.lst`;
    /// `b4_abilities_races_ce.lst` is loaded by the SAME `_bestiary_4.pcc`
    /// (line 59, ungated) and was simply never registered. Re-running
    /// `transcribe_monster_tables.py bestiary_4` against the fresh inventory
    /// (`t9-onboarding_cycle-1_cycle_receipt.md`) found these 42 newly-
    /// reachable rows citing it; widening `abilities_lsts` to include it and
    /// re-running `gen_book_cache -- bestiary_4` shipped them. Re-derive:
    /// `python3 scripts/transcribe_monster_tables.py bestiary_4 && cargo run
    /// --locked --release --bin gen_book_cache -- bestiary_4`.
    #[test]
    fn the_book_ships_two_hundred_six_monsters_and_six_hundred_nineteen_abilities() {
        assert_eq!(monsters().len(), 206);
        assert_eq!(monster_abilities().len(), 619);
    }

    /// The shipped total, pinned directly. 749 -> 783 -> 825 (+42, same cause
    /// as the test above); re-derive with `python3 scripts/classify_monster_
    /// ability_rows.py bestiary_4` (whose own "remaining"/"reachable
    /// remainder" framing answers a different, inventory-status-relative
    /// question and is no longer the live source for this number) or
    /// `scripts/scan_monster_ability_bundle_rows.py bestiary_4` rather than
    /// re-deriving by hand.
    #[test]
    fn the_shipped_total_is_the_books_real_measured_count() {
        assert_eq!(monsters().len() + monster_abilities().len(), 825);
    }

    /// Every transcribed ability row is owned by a monster row of this book.
    /// The book has 225 orphans; the point of this test is that none got in.
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
    /// Not implied by the test above, and this is the book where the difference
    /// bites: 73 ability rows name an owner that exists as a corpus row and is
    /// **not shipped**, because the owner is Product Identity. An ability
    /// pointing at one of them would satisfy "owners is non-empty" and still
    /// name a creature the catalog cannot render.
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

    /// The 14 Product Identity personas are not records, pinned by the corpus
    /// line each one is so a regeneration that stops screening them fails here
    /// naming the line that returned.
    ///
    /// Pinned by line rather than by name deliberately: naming them in source
    /// is what `decisions.md §52.5` records turning a concurrent lane's
    /// `pi-sweep` red, and `pi-sweep` does not read intent.
    #[test]
    fn the_fourteen_product_identity_rows_are_not_records() {
        for line in [40u32, 41, 42, 66, 67, 68, 110, 111, 112, 139, 140, 141, 219, 222] {
            assert!(
                !monsters().iter().any(|m| m.source_line == line),
                "b4_races.lst:{line} declares NAMEISPI:YES and must not ship"
            );
        }
    }

    /// Not one shipped monster carries a term from the LIVE Product Identity
    /// blacklist. The `NAMEISPI:YES` test above is a statement about what the
    /// corpus declares; this is a statement about what this crate screens, and
    /// the two catch different things.
    #[test]
    fn no_shipped_monster_name_carries_a_product_identity_term() {
        for monster in monsters() {
            for term in crate::rules_core::pi_screening::PI_BLACKLIST_TERMS {
                assert!(
                    !monster.name.contains(term) && !monster.key.contains(term),
                    "shipped monster {} carries blacklisted term {term}",
                    monster.key
                );
            }
        }
    }

    /// A representative pin over the orphan classes, by the corpus line each
    /// row is, so a regeneration that quietly pulls one back in fails here.
    ///
    /// One row from each cause: `b4_abilities_races_ce.lst:11` from the file
    /// that ships nothing, `b4_abilities_race.lst:324` from the rows no monster
    /// ever named, and `b4_abilities_race.lst:439`
    /// (`Demon Lord (Dagon) ~ Breath Weapon`) from the 73-row cascade this
    /// round's own PI screen created.
    #[test]
    fn the_orphan_rows_are_not_records() {
        for line in [324u32, 325, 336, 338, 439, 1413, 1414, 1415, 1416] {
            assert!(
                !monster_abilities().iter().any(|a| a.source_line == line),
                "b4_abilities_race.lst:{line} is owned by no SHIPPED monster row of this book \
                 and must not ship"
            );
        }
    }

    /// Every shipped ability of this book is reached by the namespaced-prefix
    /// link rather than by a monster row naming it — EXCEPT the 34 named
    /// below, added by `SD31-W21-MONSTER-001`'s `CATEGORY:Internal` bundle-row
    /// hop. Bundle-owned abilities are Core Essentials' generic "Universal
    /// Monster Rule" catalog entries (`Fortification`, `Powerful Blows
    /// (Bite)`, `Immunity to Calm Emotions`, …) — several carry no `" ~ "`
    /// namespace at all, which the pre-hop test could not have anticipated
    /// (it would `panic!` on the unwrap, not fail an assertion). This still
    /// holds for the `row-named 0 / prefix 543` population the classifier
    /// originally reported; only the hop's own additions are excepted.
    #[test]
    fn every_shipped_ability_is_reached_by_its_namespaced_key() {
        const BUNDLE_OWNED_EXCEPTIONS: &[&str] = &[
            "Breath Weapon ~ Cone of Electricity",
            "Breath Weapon ~ Cone of Poison",
            "Breath Weapon ~ Line of Cold",
            "Detect Scrying ~ Constant",
            "Detect Undead ~ Constant",
            "Dragon ~ Starflight",
            "Endure Elements ~ Constant",
            "Feather Fall ~ Constant",
            "Fortification",
            "Grab ~ Bite/Tail Slap",
            "Greater Invisibility ~ Constant",
            "Haste (self only) ~ Constant",
            "Immunity to Calm Emotions",
            "Immunity to Dazzled",
            "Immunity to Effects Targeting Specific Numbers of Creatures",
            "Immunity to Inhaled Poisons",
            "Immunity to Nauseated",
            "Immunity to Scent-Based Attacks",
            "Immunity to Sickened",
            "Lurking Ray ~ Smother",
            "Mage Hand ~ Constant",
            "Powerful Blows (Bite)",
            "Powerful Blows (Hoof)",
            "Powerful Blows (Slam)",
            "Protection from Evil ~ Constant",
            "Protection from Law ~ Constant",
            "Read Magic ~ Constant",
            "Regeneration ~ Good Artifacts/Effects/Spells",
            "Regeneration ~ Negative Energy",
            "Regeneration ~ Unarmed Strikes or Natural Weapons",
            "Shield ~ Constant",
            "Smother",
            "Unnatural Aura",
            "Water Walk ~ Constant",
        ];
        for ability in monster_abilities() {
            if BUNDLE_OWNED_EXCEPTIONS.contains(&ability.key) {
                assert!(
                    !ability.owners.is_empty(),
                    "{} is a bundle-owned exception but carries no owner at all",
                    ability.key
                );
                continue;
            }
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
// SD-29 Epic 7 round 5 (`SD29-E7-F2-006`) — this book's `companion` family.
//
// The second family Bestiary 4 contributes, sharing nothing with the monsters
// above but a `RuleSetId`: different `.lst` files, different chassis, different
// catalog screen (`decisions.md §51.5`). The monster lane compiled
// `RuleSetId::B4` for this book in `52da4bc3`, so this registration cost no
// scope flip — the same free registration `bestiary` (round 3) and
// `bestiary_3` (round 4) had.
//
// **78 of the book's 80 companion units ship** — 34 creature rows and 44 of its
// 46 ability rows, drawn from three `.lst` files. `78` is exactly the
// `reachable remainder` `scripts/classify_companion_rows.py bestiary_4` prints,
// so what ships and what the lane's ceiling says should ship are the same
// number derived two ways.
//
// The two exclusions are `.COPY=` DELTA rows, not orphans: `Pooka ~ Change
// Shape` and `Psychopomp (Nosoi) ~ Change Shape` each state a delta on a base
// record that lives elsewhere, so transcribing one verbatim ships a card with
// almost every field empty (`decisions.md §59.2`, adopting the monster lane's
// screen). This book's `companions` family IS surfaced, so it correctly carries
// no `OPEN_FINDINGS` entry — that list is per FAMILY, and a family that reaches
// a player is not an unsurfaced one.
//
// The round was dispatched with five orphans on the board and, like round 4,
// found they were never orphans. `Familiar (Giant Flea)` does not name
// `Flea (Giant) ~ Disease`; it names `Racial Traits ~ Flea (Giant)`, a
// `CATEGORY:Internal` row of `b4_abilities_companion.lst`, and THAT row carries
// the `ABILITY:Special Ability|AUTOMATIC|Flea (Giant) ~ Disease|…` token. The
// relay is a corpus row that is **not an inventory unit**, so shape 4 — which
// walks unit to unit — has nothing to stand on. Reading it is ownership shape 6
// (`decisions.md §59.1`); `Familiar (Pipefox)` and `Familiar (Ratling)` reach
// the three `~ Constant` rows of `b4_abilities_race_ce_companion.lst` the same
// way.
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

#[cfg(test)]
mod companion_tests {
    use super::*;

    /// 34 creature rows + 44 ability rows = 78, which is exactly the
    /// `reachable remainder` `scripts/classify_companion_rows.py bestiary_4`
    /// prints (`80 − 2` `.COPY=` delta rows). Two routes sharing no intermediate
    /// artifact, pinned rather than left as a coincidence in prose.
    #[test]
    fn the_reachable_seventy_eight_companion_units_ship() {
        assert_eq!(companions().len(), 34);
        assert_eq!(companion_abilities().len(), 44);
        assert_eq!(companions().len() + companion_abilities().len(), 78);
    }

    /// The two `.COPY=` rows are NOT records, and nothing may serve them.
    /// `verified_citation_line` refuses them at generation time; this pins that
    /// they also never re-enter through a hand edit, and that no creature is
    /// left naming a key the table does not define.
    #[test]
    fn the_two_copy_delta_rows_are_not_records() {
        for key in ["Pooka ~ Change Shape", "Psychopomp (Nosoi) ~ Change Shape"] {
            assert!(
                !companion_abilities().iter().any(|a| a.key == key),
                "{key} states a delta on a base record elsewhere and must not ship"
            );
            assert!(
                !companions().iter().any(|c| c.ability_keys.contains(&key)),
                "{key} is not transcribed, so no creature row may still name it"
            );
        }
    }

    /// Ownership shape 6, by name. These five rows are exactly the ORPHAN list
    /// the classifier printed for this book BEFORE the shape existed, and each
    /// one's owner is a creature reached across a `CATEGORY:Internal` relay.
    /// If shape 6 regresses, these are the records that vanish.
    #[test]
    fn the_five_relay_owned_rows_have_their_relay_owner() {
        for (key, owner) in [
            ("Flea (Giant) ~ Disease", "Familiar (Giant Flea)"),
            ("Flea (Giant) ~ Uncanny Leap", "Familiar (Giant Flea)"),
            ("Comprehend Languages ~ Constant", "Familiar (Pipefox)"),
            ("Read Magic ~ Constant", "Familiar (Ratling)"),
            (
                "Speak with Animals (Rodents only) ~ Constant",
                "Familiar (Ratling)",
            ),
        ] {
            let record = companion_abilities()
                .iter()
                .find(|a| a.key == key)
                .unwrap_or_else(|| panic!("{key} is not a shipped companion ability record"));
            assert!(
                record.owners.contains(&owner),
                "{key} is reached from {owner} across a CATEGORY:Internal relay row, \
                 but {owner} is not among its owners: {:?}",
                record.owners
            );
        }
    }

    /// This book's companion rows and its monster rows are different records
    /// even where they share a species name — `Flea (Giant)` is a monster KEY in
    /// `b4_races.lst` AND the namespace prefix of two companion ability rows.
    /// Nothing may serve one as the other.
    #[test]
    fn the_companion_rows_are_not_this_module_s_monster_rows() {
        for companion in companions() {
            assert!(
                !monsters().iter().any(|m| m.key == companion.key),
                "{} is registered as both a companion and a monster of this book",
                companion.key
            );
        }
        // `Read Magic ~ Constant` is a VERIFIED exception, not a defect
        // (`SD31-W21-MONSTER-001`): Core Essentials ships this generic
        // spell-like-ability template TWICE in the pinned oracle, byte-for-
        // byte identical, once in each family's own abilities file --
        // `b4_abilities_races_ce.lst:33` (this book's monster side, owned by
        // `Contemplative` via the `CATEGORY:Internal` bundle-row hop) and
        // `b4_abilities_race_ce_companion.lst:8` (the companion side, owned
        // by `Ratling`/`Familiar (Ratling)`) -- confirmed identical byte-for-
        // byte against the pinned oracle before excepting it here, not
        // assumed from the shared name alone.
        const CROSS_FAMILY_DUPLICATE_EXCEPTIONS: &[&str] = &["Read Magic ~ Constant"];
        for ability in companion_abilities() {
            if CROSS_FAMILY_DUPLICATE_EXCEPTIONS.contains(&ability.key) {
                continue;
            }
            assert!(
                !monster_abilities().iter().any(|a| a.key == ability.key),
                "{} is registered as both a companion ability and a monster ability",
                ability.key
            );
        }
    }
}
