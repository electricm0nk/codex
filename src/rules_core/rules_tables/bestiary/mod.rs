//! Bestiary 1 monster/monster-ability **chassis** module. SD-29 Epic 5 extend,
//! round 8 — `corpus-work-channels.md §9.2`, executing `decisions.md §58.3`.
//!
//! # This is the SECOND table serving one book, and that is the point
//!
//! Bestiary 1 is the only book this repo serves from two compiled monster
//! tables. SD-22's [`super::beastiary1`] holds **46** hand-modelled stat blocks
//! with natural-attack provenance and a `beastiary1:monster:<slug>` key space;
//! this module holds the book's **complement** — the other 284 rows — in the
//! ordinary chassis shape.
//!
//! `decisions.md §58.3` ruled ALONGSIDE rather than ABSORB, and the reason is
//! player-visible: absorbing means emitting all 330 rows, which puts two records
//! for 46 creatures into one catalog under one wire code (`B1`) unless SD-22's
//! tables, their `cache_gen`, their `natural_attack_provenance`, their
//! `data/corpus/beastiary/monster/` records and their key space are retired with
//! them. That is a cross-bundle retirement of shipped, grounded, player-visible
//! records, and it churns content that is already correct.
//!
//! The complement is not maintained by hand. The transcriber derives it from the
//! other table's **own shipped records** —
//! `scripts/transcribe_monster_tables.py::cross_table_served_monster_keys` reads
//! `data/corpus/beastiary/monster/` and withholds every row already written in
//! the pre-`key` Shape B v1 shape (`data.id`, no `data.key`). That is the same
//! denominator `reach_gate::monsters_reach` reads, so the two cannot disagree,
//! and it is stable under this generator's own output because the two record
//! shapes are distinguishable.
//!
//! # What ships, and the arithmetic that says so
//!
//! **280 monsters + 323 monster abilities = 603 records**, against corpus unit
//! counts of 330 and 523. Derived, never assumed:
//! `python3 scripts/classify_monster_ability_rows.py bestiary` →
//! `bestiary  284  523  375  2  146  0  0`.
//!
//! | class | monsters | abilities |
//! |---|---|---|
//! | shipped here | 280 | 323 |
//! | **cross-table owner** — the other table's, or owned only by rows that are | 46 | 54 |
//! | `.MOD`-only overlay — a delta on a record defined elsewhere | 4 | — |
//! | orphan — nothing in the book owns it | — | 146 |
//! | corpus total | 330 | 523 |
//!
//! The classifier reports **661** reachable for this book (`807 − 146 orphan −
//! 0 PI − 0 .COPY=`). It counts 284 monster *units* while resolving ownership
//! against all **330** corpus rows, so 54 of the abilities it calls reachable are
//! reachable only through a monster this table does not hold, and 4 of the
//! monsters it counts are overlays rather than stat blocks. `661 − 54 − 4 = 603`,
//! and each term of the residue has a name rather than being a rounding error.
//!
//! # `.MOD`-only rows, a second delta shape beside `.COPY=`
//!
//! Four of this book's monster units exist in `b1_races.lst` only as
//! `<Record>.MOD` overlays — `:239`, `:241`, `:251`, `:257`. An overlay states a
//! delta on a record defined elsewhere, exactly as a `.COPY=` row does, and
//! transcribing one verbatim yields the delta's few tokens under the record's
//! name. `gen_book_cache::verified_citation_line` refuses them outright anyway,
//! because the row's first column reads `<Record>.MOD` and not the record's
//! name — which is how this round found them.
//!
//! The work inventory has always classed them itself, in its `origin` field, so
//! the screen reads that field rather than re-deriving it. Scope over the whole
//! corpus, derived before the screen was written: of 4,377 `monster` /
//! `monster_ability` units, **4** are `mod_only` and all four are these; 2 are
//! `copy` (Bestiary 2's, already screened); the other 4,371 are `declared`.
//!
//! A cross-table-owner row is NOT an orphan. An orphan is a row nothing in the
//! book owns and its remedy is a link that does not exist. These 54 are
//! well-formed and owned; they are unreachable from here only because every
//! monster naming them lives in the other table, which has no ability family at
//! all. The remedy is to widen that table or migrate it, and it belongs to
//! whoever rules on retiring the SD-22 key space — not to an ingest round.
//!
//! # Product Identity
//!
//! Zero, in both signals: `grep -c NAMEISPI:YES b1_races.lst
//! b1_abilities_race.lst` → `0`, `0`, and the classifier's `PI` column reads 0.
//! That is what `docs/governance/ogl-pi-blacklist.md` §2's PER-RECORD predicate
//! (`decisions.md §57.1`) predicts for a `roleplaying_game/` bestiary: its
//! creatures are generic SRD species, not unique named personae.
//!
//! # Two `DESC:` shapes widened for this book, one row each
//!
//! Both are `decisions.md §46`'s summary-versus-full pair in a row that carries
//! no `DisplayFullAbility` gate, and both are selections between two verbatim
//! corpus texts on a criterion the corpus itself states:
//!
//! * **superset** (`b1_abilities_race.lst:1183`) — one token's text literally
//!   begins with the other's, so the long one contains the short one whole.
//! * **variable-bearing** (`b1_abilities_race.lst:1068`) — exactly one token
//!   carries a pipe entry, and every entry names a variable the row's own
//!   `DEFINE:` declares, so the row's `DEFINE:`/`BONUS:VAR` machinery exists to
//!   fill that token's `%N` and no other's. Its ungated summary stops at "as
//!   ranged touch attacks" and drops the severing AC, the Fortitude DC and the
//!   Strength damage.
//!
//! Scope derived before widening, not after: over every registered book, 54
//! ability rows carry several `DESC:` tokens — 34 gated-full, 4 continuation,
//! 1 superset, 1 variable-bearing, 14 still refused, and not one of the 14 is a
//! row any book ships. Re-transcribing all nine previously registered books
//! after the widening changed **zero** records.
//!
//! # Provenance
//!
//! `bestiary.pcc` carries the book's own `COPYRIGHT` block and the roleplaying-
//! game `OGL.txt`; the records ship `License::Ogl` like every other book in this
//! registry. The `data/corpus/beastiary/LICENSE.json` this pass writes to is
//! shared with four earlier lanes, so the screening note is appended, never
//! replaced — `decisions.md §54.4`.

mod monster_data;
/// SD-32 `decisions.md §20`: Bestiary's custom spell-variant declarations
/// (`core_essentials/ce_spells.lst`, real `SCHOOL:`/`DESC:`-bearing rows,
/// not monster-ability text) -- generated by `src/bin/ingest_spells.rs`'s
/// config-driven `BOOKS` table, same shape as every other book's own
/// `spell_list` module.
pub mod spell_list;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
    NaturalAttack, Speed,
};

/// Every monster stat block this chassis defines, in corpus row order.
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this chassis defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}

/// Every monster stat block this chassis defines, in corpus row order.
pub fn monsters() -> &'static [MonsterStatBlock] {
    monsters_static()
}

/// Every monster-ability record this chassis defines, in corpus row order.
pub fn monster_abilities() -> &'static [MonsterAbilityRecord] {
    monster_abilities_static()
}

/// The 46 legacy Bestiary 1 monster NAMES `rules_tables::beastiary1` ships a
/// [`beastiary1::MonsterStatBlock`](super::beastiary1::MonsterStatBlock) for
/// -- the real owner of the 55 CROSS-TABLE-OWNER ability rows this chassis's
/// own `monster_abilities()` transcribes (`decisions.md §58.3`,
/// `SD31-W23-MONSTER-001`). `MonsterBook::cross_table_owner_names` (this
/// book's registry row) reads this so the generic cross-book invariant
/// (`monster_chassis::the_chassis_link_resolves_in_both_directions_for_
/// every_book`) can tell a genuinely-dangling owner from one whose stat
/// block simply lives in the OTHER table on purpose.
///
/// A hand-kept `&'static str` literal list, not `beastiary1::MonsterId::ALL`
/// re-derived at this call site: that function returns OWNED
/// [`beastiary1::MonsterStatBlock`]s (heap `String` fields), which cannot be
/// a `const` a `MONSTER_BOOKS` registry entry can embed. Guarded against
/// drift by `cross_table_owner_names_matches_the_real_beastiary1_roster_
/// exactly`, which re-derives the true 46 from `beastiary1::MonsterId::ALL`
/// itself and asserts this list is byte-identical to it (order-independent).
pub(super) const fn cross_table_owner_names() -> &'static [&'static str] {
    &[
        "Ghoul", "Gnoll", "Goblin Dog", "Lizardfolk", "Wolf", "Darkmantle", "Horse", "Hyena",
        "Octopus", "Spider Swarm", "Bat Swarm", "Boar", "Boggard", "Bugbear", "Cave Fisher",
        "Choker", "Crocodile", "Dark Creeper", "Iron Cobra", "Morlock", "Rat Swarm", "Sahuagin",
        "Shark", "Shocker Lizard", "Skum", "Squid", "Troglodyte", "Vargouille", "Wolverine",
        "Worg", "Yellow Musk Creeper", "Ankheg", "Assassin Vine", "Centaur", "Cockatrice",
        "Derro", "Doppelganger", "Dryad", "Ettercap", "Gelatinous Cube", "Hell Hound", "Lion",
        "Ogre", "Pegasus", "Rust Monster", "Shadow",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::{beastiary1, RuleSetId};

    /// What ships is the book's complement, less its overlay rows: 280 of 330
    /// monster rows and (**corrected `SD31-E6-F9-005`**, was 323) 399 of 523
    /// ability rows -- the transcriber used to `raise SystemExit` the instant
    /// it found ANY owned ability row with a `parse_desc`-unmodelled
    /// multi-`DESC:` shape (3 such rows, `OPEN-ISSUES.md` row 157), crashing
    /// the WHOLE book's transcription and silently capping this table at 323
    /// even though 76 MORE owned, cleanly-parseable ability rows existed. The
    /// fix drops only the 3 ambiguous rows (named in this book's own module
    /// doc comment above), the same treatment a Product Identity row already
    /// gets, and the other 76 now ship for real.
    #[test]
    fn the_chassis_ships_the_books_complement() {
        assert_eq!(monsters().len(), 280);
        // 399 -> 467 (SD31-W21-MONSTER-001): the `CATEGORY:Internal` bundle-row
        // ownership hop (`transcribe_monster_tables.py::find_internal_bundle_
        // ability_refs`) resolved 68 previously-orphaned ability rows this
        // book's monsters name only indirectly, through a bundle row.
        // 467 -> 522 (SD31-W23-MONSTER-001): the cross-table-owner remedy
        // `decisions.md §58.3` named and left unbuilt -- 55 ability rows
        // whose owner's OWN stat block ships from `rules_tables::beastiary1`
        // (this book's OTHER, 46-monster table) now transcribe here too,
        // keyed to that real owner's name rather than dropped. This table's
        // own `monsters()` count above is UNCHANGED (still 280 -- these 55
        // rows' owners are still not among them, by the same `§58.3` ruling
        // this test already asserts on the line above).
        // 522 -> 529 (T9 `MonsterAbilityFacet` widening cycle): re-running
        // `scripts/transcribe_monster_tables.py bestiary` against the widened
        // facet vocabulary (`Weakness`/`Defensive`/`Aura`/`Sense`/
        // `Communicate` added to `FACETS`) shipped 7 more owned, reachable
        // ability rows that previously carried a `TYPE:` shape the chassis
        // did not model. 2 owned rows remain excluded and named on stderr
        // (`Morlock ~ Sneak Attack`, bare `Internal`; `Spectre ~ Create
        // Spawn`, comma-joined `TYPE:SpecialAttack,Supernatural` — a likely
        // corpus typo for `.`, deliberately not auto-corrected).
        // 529 -> 709 (`decisions.md §20`, no_record-to-zero wave 2): 180 of
        // the 197 rows no monster row of this book claims now SHIP with
        // `owners: &[]` rather than being dropped — an un-ingested row's
        // shape cannot be measured, and Gate 1's DoD needs every unit's
        // shape measured. The other 17 are excluded for an UNRELATED reason
        // that now applies to them too, because they are no longer dropped
        // before reaching those screens: `unscreenable`'s multi-`DESC:`
        // shape (22 hits total, some against already-owned rows) and
        // `unmodelled_facet` (2 hits total, both already-owned) — re-derive
        // with `python3 scripts/transcribe_monster_tables.py bestiary`.
        // `no_shipped_ability_is_an_orphan` below is rewritten to pin the
        // exact owner-less set instead of forbidding it; reachability is
        // NOT claimed for the 180 — each is pinned by exact key in
        // `reach_gate.rs::UNREACHED_RECORD_FINDINGS` under
        // `("bestiary1", "monster_abilities")`.
        // 709 -> 710 (`decisions.md §22`/round 6, +1): the comma-delimiter
        // `TYPE:` upstream correction resolved `Spectre ~ Create Spawn`'s
        // facet for the first time -- this file's own pin was missed when
        // round 6 bumped the identical delta in `apps/desktop/src-tauri/
        // src/reach_gate.rs` and `corpus_ingest_diagnostic.rs`; re-derived
        // here, not caused by this cycle's own diff (`git diff --stat` for
        // `bestiary/monster_data.rs` shows zero deletions, only the 3
        // trailing `codex_generated_name`/`rename_*` fields appended per
        // record).
        // 710 -> 711 (`decisions.md §27`/round 8, +1): `Morlock ~ Sneak
        // Attack` (`TYPE:Internal`, no facet/delivery segment) now ships
        // with a PROVISIONAL `SpecialQuality` facet default instead of
        // being dropped -- `reason: type_internal_only_no_facet_no_delivery`.
        // 711 -> 733 (`decisions.md §27b` round 9, +22): the multi-`DESC:`
        // `PREVAREQ`/`PREVARGT`-gated parse-refusal group closes via
        // `parse_desc`'s new generalised sixth branch -- every token's own
        // text ships, concatenated verbatim, rather than guessing which
        // variant wins. 21 real `no_record` units plus `Lycanthrope ~
        // Change Shape` (already `text-complete` by inventory evidence
        // alone, same shape as round 8's `Bunyip ~ Blood Rage`).
        assert_eq!(monster_abilities().len(), 733);
    }

    /// The four `.MOD`-only overlay rows are not records, pinned by the corpus
    /// line each one is. An overlay states a delta on a record defined
    /// elsewhere; shipping one puts a card with a name and almost nothing else
    /// in front of a player.
    #[test]
    fn the_mod_only_overlay_rows_are_not_records() {
        for line in [239u32, 241, 251, 257] {
            assert!(
                !monsters().iter().any(|m| m.source_line == line),
                "b1_races.lst:{line} is a `.MOD` overlay and must not ship"
            );
        }
    }

    /// The shipped total, pinned directly rather than re-derived through
    /// `classify_monster_ability_rows.py`'s own arithmetic.
    ///
    /// **CORRECTED `SD31-E6-F9-005`.** The prior version of this test derived
    /// 603 from `807 (classifier "remaining") - 146 orphans - 54 cross-table -
    /// 4 .MOD overlays`. Re-deriving that formula against `SD31-E6-F9-005`'s
    /// own fix (which unblocked 76 more real ability rows, `679` total)
    /// surfaced a genuine, previously-unknown limitation in the classifier
    /// script itself: `classify_monster_ability_rows.py` computes its
    /// `row-named`/`prefix` "reachable" counts purely from a monster's own
    /// `ABILITY:`/prefix token, with NO awareness of `CROSS_TABLE_MONSTER_
    /// RECORDS` -- so it counts an ability as reachable even when its
    /// OWNING monster (e.g. `Ankheg`, one of SD-22's 46 `beastiary1`-served
    /// monsters) is one this chassis deliberately does not ship
    /// (`decisions.md §58.3`). Corpus-wide re-check: of the classifier's 135
    /// `row-named`+`prefix` units for this book, exactly 59 are owned only
    /// through a cross-table monster and the transcriber correctly does NOT
    /// ship them (135 - 59 = 76, exactly this fix's own measured delta) --
    /// `OPEN-ISSUES.md` names the classifier gap as its own follow-up. The
    /// arithmetic-derivation shape this test used is retired in favor of a
    /// direct pin, which cannot silently inherit the same blind spot again.
    #[test]
    fn the_shipped_total_is_the_books_real_measured_count() {
        // 679 -> 747 (SD31-W21-MONSTER-001, +68 bundle-hop-owned abilities;
        // see `the_chassis_ships_the_books_complement`'s own comment).
        // 747 -> 802 (SD31-W23-MONSTER-001, +55 cross-table-owner ability
        // rows -- see `the_chassis_ships_the_books_complement`'s own comment
        // and `cross_table_owner_names` above). `monsters().len()` (280) is
        // UNCHANGED: these 55 rows' real owners still ship from
        // `beastiary1`, never from here.
        // 802 -> 809 (T9 `MonsterAbilityFacet` widening cycle, +7 abilities;
        // see `the_chassis_ships_the_books_complement`'s own comment).
        // 809 -> 989 (`decisions.md §20`, +180 owner-less abilities; see
        // `the_chassis_ships_the_books_complement`'s own comment).
        // 989 -> 990 (`decisions.md §22`/round 6, +1; see
        // `the_chassis_ships_the_books_complement`'s own comment on the
        // identical, previously-unpinned delta here).
        // 990 -> 991 (`decisions.md §27`/round 8, +1; see
        // `the_chassis_ships_the_books_complement`'s own comment on the
        // identical delta here).
        // 991 -> 1013 (`decisions.md §27b` round 9, +22; see
        // `the_chassis_ships_the_books_complement`'s own comment on the
        // identical delta here).
        assert_eq!(monsters().len() + monster_abilities().len(), 1013);
    }

    /// **The ruling, as a test.** Not one creature is served twice. This is the
    /// defect `decisions.md §58.3` chose ALONGSIDE over ABSORB to avoid, and it
    /// is player-visible: both tables reach `list_monster_catalog` under the
    /// same wire code `B1`, so a row held by both would appear twice in one
    /// catalog.
    #[test]
    fn no_creature_is_served_by_both_bestiary_1_tables() {
        let sd22: Vec<String> = beastiary1::MonsterId::ALL
            .iter()
            .filter_map(|&id| beastiary1::monster_resolve(id, RuleSetId::Bestiary1))
            .map(|block| block.name)
            .collect();
        assert_eq!(sd22.len(), 46, "the other table's roster is 46 stat blocks");
        for block in monsters() {
            assert!(
                !sd22.iter().any(|name| name == block.name),
                "{} is served by rules_tables::beastiary1 as well as by this chassis",
                block.key
            );
        }
    }

    /// **Superseded `decisions.md §20` (no_record-to-zero wave 2).** Until
    /// this cycle every transcribed ability row named at least one owner --
    /// a monster row this table holds, or one of the 55 cross-table-owner
    /// rows whose real owner ships from `beastiary1` instead -- because an
    /// unowned row was dropped rather than shipped. `§20` overturned that:
    /// an un-ingested row's shape cannot be measured, so the 180 rows no
    /// monster row of this book claims now SHIP with `owners: &[]`, and this
    /// test's job changes from "forbid an empty owner list" to "pin the
    /// EXACT set of records that carry one" -- a silent new arrival OR a
    /// silent disappearance both fail here, by name, the same discipline
    /// `monster_chassis::tests::widening_the_facet_vocabulary_does_not_
    /// reclassify_any_existing_record` already established for the facet
    /// axis. `list_monster_catalog` never walks these directly (only a
    /// monster's own `ability_keys`), so shipping them does not surface a
    /// stub; each key is pinned separately, by name, in `reach_gate.rs::
    /// UNREACHED_RECORD_FINDINGS` under `("bestiary1", "monster_abilities")`
    /// as a proven non-reach, not a silent claim of reachability.
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
            197,
            "the number of owner-less (unreachable-by-design) monster_ability records \
             changed — re-derive this pin from a real \
             `scripts/transcribe_monster_tables.py bestiary` run, and update the matching \
             `reach_gate.rs::UNREACHED_RECORD_FINDINGS` entry to the same key set. 180 -> 197 \
             (`decisions.md §27b` round 9, +17): the multi-DESC: parse-refusal group closes; \
             13 `Permanency Spell / *` rows, `Outsider`/`Swarm`/`Undead Traits Output`, and \
             `Lycanthrope ~ Change Shape` (the round-8-shaped bonus unit, already \
             `text-complete` by inventory evidence alone) are shared reference-library text no \
             single stat block in this book owns, same shape as the existing 180."
        );

        let mut hasher = DefaultHasher::new();
        unowned.hash(&mut hasher);
        let digest = hasher.finish();
        assert_eq!(
            digest, 0x0bce_5246_54f6_6a5d,
            "the SET of owner-less keys moved even though the count held — a row gained or \
             lost its owner. This does not mean a defect on its own (an in-book monster row \
             could legitimately start/stop claiming one of these), but it means \
             `reach_gate.rs::UNREACHED_RECORD_FINDINGS`'s pinned key list for \
             (\"bestiary1\", \"monster_abilities\") must move with it. 0x87d526f2aaeac3c6 -> \
             0x0bce524654f66a5d (`decisions.md §27b` round 9): the set gains 17 new members \
             (see the count assertion above), re-derived live from this test's own failing \
             run, never guessed, per `decisions.md §17a`; `reach_gate.rs`'s matching entry \
             gains the identical 17 keys."
        );
    }

    /// The stronger form, which this book needs more than any before it: 46 of
    /// its monster rows exist in the corpus, are deliberately not shipped here
    /// as `MonsterStatBlock`s, and are named as owners by 55 ability rows this
    /// chassis DOES ship anyway (`SD31-W23-MONSTER-001`, `decisions.md
    /// §58.3`'s cross-table-owner class). An owner named by neither `monsters`
    /// NOR `cross_table_owner_names` is a link the catalog truly cannot
    /// follow -- that is still what this test catches.
    #[test]
    fn every_owner_named_by_a_shipped_ability_is_a_shipped_monster() {
        let cross_table = cross_table_owner_names();
        for ability in monster_abilities() {
            for owner in ability.owners {
                assert!(
                    monsters().iter().any(|m| m.key == *owner)
                        || cross_table.contains(owner),
                    "{} names owner {owner}, which is not a shipped monster of this table \
                     and not in cross_table_owner_names either",
                    ability.key
                );
            }
        }
    }

    /// Guards `cross_table_owner_names`'s own hand-kept literal list against
    /// drift from the real `beastiary1` roster it stands in for -- re-derives
    /// the true 46 names from `beastiary1::MonsterId::ALL` independently (the
    /// same derivation `no_creature_is_served_by_both_bestiary_1_tables`
    /// already trusts) and asserts set-equality, so a future `beastiary1`
    /// subset addition that forgets to update the literal list fails HERE
    /// rather than silently under-covering the cross-table-owner check above.
    #[test]
    fn cross_table_owner_names_matches_the_real_beastiary1_roster_exactly() {
        let real: std::collections::BTreeSet<String> = beastiary1::MonsterId::ALL
            .iter()
            .filter_map(|&id| beastiary1::monster_resolve(id, RuleSetId::Bestiary1))
            .map(|block| block.name)
            .collect();
        let listed: std::collections::BTreeSet<&str> =
            cross_table_owner_names().iter().copied().collect();
        assert_eq!(real.len(), 46);
        assert_eq!(listed.len(), 46, "cross_table_owner_names must not repeat a name");
        for name in &real {
            assert!(
                listed.contains(name.as_str()),
                "beastiary1 ships {name:?} but cross_table_owner_names does not list it"
            );
        }
        for name in &listed {
            assert!(
                real.contains(*name),
                "cross_table_owner_names lists {name:?}, which beastiary1 does not ship"
            );
        }
    }

    /// Every ability key a shipped monster names resolves in this table. The
    /// cross-table and orphan screens remove ability rows *after* the link pass
    /// builds each monster's `ability_keys`, so a screen that forgot to prune
    /// the owner's array would ship a stat block pointing at a record that does
    /// not exist — and `gen_book_cache` would write that dangling key into the
    /// corpus record verbatim.
    #[test]
    fn every_ability_key_a_shipped_monster_names_resolves_here() {
        for block in monsters() {
            for key in block.ability_keys {
                assert!(
                    monster_abilities().iter().any(|a| a.key == *key),
                    "{} names ability {key}, which this table does not define",
                    block.key
                );
            }
        }
    }

    /// The variable-bearing `DESC:` row ships the FULL text, not the ungated
    /// summary. Before round 8 widened `parse_desc` this row stopped the
    /// transcription outright; taking the first token would have served the
    /// range and dropped every mechanic.
    #[test]
    fn the_variable_bearing_desc_row_ships_its_full_text() {
        let record = monster_abilities()
            .iter()
            .find(|a| a.source_line == 1068)
            .expect("b1_abilities_race.lst:1068 ships");
        let text = record.description.expect("the row carries DESC: text");
        assert!(
            text.contains("severed by any amount of slashing damage")
                && text.contains("Fortitude save"),
            "the record serves only the ungated summary: {text:?}"
        );
        assert!(
            !record.description_variables.is_empty(),
            "the selected token's `%N` variable list did not survive selection"
        );
    }

    /// The superset `DESC:` row ships the containing token.
    #[test]
    fn the_superset_desc_row_ships_the_containing_token() {
        let record = monster_abilities()
            .iter()
            .find(|a| a.source_line == 1183)
            .expect("b1_abilities_race.lst:1183 ships");
        let text = record.description.expect("the row carries DESC: text");
        assert!(
            text.contains("rises from death 3 rounds later"),
            "the record serves only the shorter of the two tokens: {text:?}"
        );
    }
}
