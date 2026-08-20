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
        assert_eq!(monster_abilities().len(), 467);
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
        assert_eq!(monsters().len() + monster_abilities().len(), 747);
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

    /// Every transcribed ability row is owned by a monster row this table
    /// holds. The book has 200 ability rows nothing here owns — 146 orphans and
    /// 54 cross-table-owner rows; the point of this test is that none got in.
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

    /// The stronger form, which this book needs more than any before it: 46 of
    /// its monster rows exist in the corpus, are deliberately not shipped here,
    /// and are named as owners by 54 ability rows. An owner this table does not
    /// hold is a link the catalog cannot follow.
    #[test]
    fn every_owner_named_by_a_shipped_ability_is_a_shipped_monster() {
        for ability in monster_abilities() {
            for owner in ability.owners {
                assert!(
                    monsters().iter().any(|m| m.key == *owner),
                    "{} names owner {owner}, which is not a shipped monster of this table",
                    ability.key
                );
            }
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
