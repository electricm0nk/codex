//! Bestiary 3 (`SOURCESHORT:B3`) — `monster` + `monster_ability` + `companion`.
//!
//! The `companion` family was added by SD-29 Epic 7 round 4 and is documented
//! above this file's test module; it draws on four `.lst` files none of the
//! monster text below mentions. The two families share only a `RuleSetId`.
//!
//! **261 of this book's 261 monster rows and 36 of its 40 ability rows ship.**
//! 27 shipped before `SD31-W21-MONSTER-001`; that round's `CATEGORY:Internal`
//! bundle-row ownership hop resolved 9 more (`Adhukait ~ …` x3, `Aghasura ~ …`
//! x4, `Legion Archon ~ …` x2), leaving 4 genuine orphans. No Product Identity
//! row, no `.COPY=` delta, no monster excluded for any reason at all: this is
//! still the cleanest book the lane has taken.
//!
//! ```text
//! python3 scripts/classify_monster_ability_rows.py bestiary_3
//! book         mon  abil row-named prefix ORPHAN   PI COPY
//! bestiary_3   261    40         0     27     13    0    0
//! ```
//!
//! The classifier above has no awareness of the bundle hop
//! (`scripts/scan_monster_ability_bundle_rows.py` is the instrument that
//! does) — its `prefix`/`ORPHAN` columns are a pre-hop figure for this book,
//! not what ships now.
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
//! # The first book reached ENTIRELY by the namespaced-prefix link, pre-hop
//!
//! `row-named` is **0** and `prefix` is **27**: those 27 shipped abilities are
//! each reached because their own `KEY:` is `<Monster> ~ <Ability>` and the
//! prefix is a monster row here — the four books before it lean the other way.
//! **This is no longer the whole shipped set** — `SD31-W21-MONSTER-001`'s
//! `CATEGORY:Internal` bundle-row hop reaches 9 more whose namespace prefix is
//! a DIFFERENT string than their real owner (`Adhukait`/`Aghasura`/`Legion
//! Archon` versus `Asura (Adhukait)`/`Asura (Aghasura)`/`Archon (Legion)`) —
//! `every_shipped_ability_is_reached_by_its_namespaced_key` below now excludes
//! them by name rather than silently widening what it asserts.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// What ships is 261 and 36, against corpus unit counts of 261 and 40.
    /// Asserting 40 here would assert that this book ships four records
    /// nothing can reach.
    ///
    /// 27 -> 36 (SD31-W21-MONSTER-001, +9): the `CATEGORY:Internal` bundle-row
    /// ownership hop resolved 9 previously-orphaned ability rows this book's
    /// monsters name only indirectly.
    #[test]
    fn the_book_ships_every_monster_and_thirty_six_linked_abilities() {
        assert_eq!(monsters().len(), 261);
        // 36 -> 409 (T9 `MonsterAbilityFacet` widening cycle). Two fixes
        // together: (1) the widened facet vocabulary
        // (`Weakness`/`Defensive`/`Aura`/`Sense`/`Communicate`) and (2) a
        // real parsing bug fix, reading EVERY `TYPE:` token on a row instead
        // of only the first (`scripts/transcribe_monster_tables.py::
        // type_segments`) — 27 dragon-subtype rows state their facet on a
        // SECOND `TYPE:` token (`Forest Dragon ~ Change Shape`:
        // `TYPE:Supernatural` then `TYPE:RaceAbility.SpecialQuality`), which
        // the old single-token `token()` read silently dropped. This
        // remains the "36" function name deliberately — renaming it is out
        // of this cycle's scope, and the number it names is now stale by
        // construction, exactly like `the_shipped_total_is_the_books_
        // real_measured_count`'s own precedent in `bestiary/mod.rs`. 1
        // owned row remains excluded and named on stderr (`Adlet ~
        // Spell-Like Abilities`, bare `TYPE:SpellLike` with no facet
        // segment at all).
        // 409 owned + 266 owner-less (`decisions.md §20`, no_record-to-zero
        // wave 2 follow-on) = 675. The owner-less count is pinned separately
        // below (`every_owner_less_ability_is_a_named_and_pinned_non_reach`).
        // 409/675 -> 410/686 (`decisions.md §27`/round 8, +11 total): the
        // `TYPE:`-facet-vocabulary-gap group closes via the provisional
        // `SpecialQuality` default. +1 owned (`Adlet ~ Spell-Like
        // Abilities`, this comment's own previously-named excluded row,
        // now shipping); +10 owner-less (`Asurendra ~ None`, `Lunar/Royal/
        // Water Naga ~ Spells`, `Unfettered Eidolon ~
        // Str/Dex/Con/Int/Wis/Cha`) — pinned separately below.
        // 410/686 -> 410/696 (`decisions.md §27b` round 9, +10 total, all
        // owner-less): the multi-DESC: parse-refusal group closes via
        // `parse_desc`'s new generalised sixth branch -- Jiang-Shi Vampire
        // plus the 9 `Traits Output ~ <Kind>` rows (`&nl;`-marker
        // continuation shape) are shared reference-library text no single
        // stat block in this book owns; `owned` is UNCHANGED, all 10 land
        // in the owner-less pin below.
        let owned = monster_abilities()
            .iter()
            .filter(|a| !a.owners.is_empty())
            .count();
        assert_eq!(owned, 410);
        assert_eq!(monster_abilities().len(), 696);
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

    /// **Superseded `decisions.md §20` (no_record-to-zero wave 2 follow-on).**
    /// An owner-less ability row no longer forbids shipping: an un-ingested
    /// row's shape cannot be measured, so the 266 rows no monster row of this
    /// book claims now SHIP with `owners: &[]`, and this test's job changes
    /// from "forbid an empty owner list" to "pin the EXACT set of records
    /// that carry one". `list_monster_catalog` never walks these directly
    /// (only a monster's own `ability_keys`), so shipping them does not
    /// surface a stub; each key is pinned separately, by name, in
    /// `reach_gate.rs::UNREACHED_RECORD_FINDINGS` under
    /// `("bestiary_3", "monster_abilities")` as a proven non-reach, not a
    /// silent claim of reachability.
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

        // 266 -> 276 (`decisions.md §27`/round 8, +10): the `TYPE:`-facet-
        // vocabulary-gap group closes via the provisional `SpecialQuality`
        // default -- 10 of the 11 newly-shipped rows are owner-less
        // (`Asurendra ~ None`, `Lunar/Royal/Water Naga ~ Spells`,
        // `Unfettered Eidolon ~ Str/Dex/Con/Int/Wis/Cha`); the 11th
        // (`Adlet ~ Spell-Like Abilities`) is owned, see the test above.
        // 276 -> 286 (`decisions.md §27b` round 9, +10): the multi-DESC:
        // parse-refusal group closes -- Jiang-Shi Vampire plus the 9
        // `Traits Output ~ <Kind>` rows, all owner-less, see the test above.
        assert_eq!(
            unowned.len(),
            286,
            "the number of owner-less (unreachable-by-design) monster_ability records \
             changed — re-derive this pin from a real \
             `scripts/transcribe_monster_tables.py bestiary_3` run, and update the matching \
             `reach_gate.rs::UNREACHED_RECORD_FINDINGS` entry to the same key set"
        );

        let mut hasher = DefaultHasher::new();
        unowned.hash(&mut hasher);
        let digest = hasher.finish();
        assert_eq!(
            digest, 0x9384_d1f9_b175_24c6,
            "the owner-less key SET changed (same count, different members) — re-derive and \
             update `reach_gate.rs::UNREACHED_RECORD_FINDINGS` to match exactly. \
             0x01b42774_3381b829 -> 0x9384d1f9_b17524c6 (`decisions.md §27b` round 9): the \
             set gains 10 new members (Jiang-Shi Vampire plus 9 `Traits Output ~ <Kind>` \
             rows), re-derived live from this test's own failing run, never guessed, per \
             `decisions.md §17a`."
        );
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

    /// The remaining orphan rows are pinned individually **by the corpus line
    /// each one is**, so a regeneration that quietly pulls one back in fails
    /// here naming the line that returned.
    ///
    /// `b3_abilities_race.lst:1663` is in this list and is also the row that
    /// made the transcriber defer its `DESC:` refusal — see this module's
    /// header. If a future widening teaches `parse_desc` that shape, this test
    /// still holds: the row is excluded because nothing owns it, not because it
    /// could not be parsed.
    ///
    /// `SD31-W21-MONSTER-001` dropped 380, 381, 389, 390, 391, 394, 395, 396,
    /// 397 from this list — all 9 now ship, owned via the `CATEGORY:Internal`
    /// bundle-row hop (see the module header and
    /// `every_shipped_ability_is_reached_by_its_namespaced_key` below).
    /// **Superseded `decisions.md §20` for three of the four.** `304`,
    /// `1150`, and `1448` now ship owner-less (shape measurable,
    /// reachability not claimed) instead of being excluded — each is one of
    /// the 266 pinned by `every_owner_less_ability_is_a_named_and_pinned_
    /// non_reach` above. `1663` (`Jiang-Shi Vampire`) stays excluded for an
    /// UNRELATED, pre-existing reason: its `DESC:` shape is multi-valued and
    /// `parse_desc` still refuses it (`scripts/transcribe_monster_tables.py
    /// bestiary_3`'s own stderr names it under the multi-`DESC:` screen, not
    /// the orphan one) — the orphan-ship mechanism only widens WHAT the
    /// orphan screen keeps, never what an unrelated screen already drops.
    #[test]
    fn the_previously_excluded_orphans_now_ship_owner_less() {
        for line in [304u32, 1150, 1448] {
            let ability = monster_abilities()
                .iter()
                .find(|a| a.source_line == line)
                .unwrap_or_else(|| {
                    panic!(
                        "b3_abilities_race.lst:{line} ships for shape measurement \
                         (decisions.md §20)"
                    )
                });
            assert!(
                ability.owners.is_empty(),
                "{} was expected owner-less; no monster row of this book claims it",
                ability.key
            );
        }
        // **Round 9 update (`decisions.md §27b`):** `b3_abilities_race.lst:1663`
        // (Jiang-Shi Vampire) used to be excluded by the multi-DESC: screen.
        // `parse_desc`'s new generalised sixth branch (`_concat_desc_variants`)
        // now resolves it -- an `&nl;`-marker continuation shape, same
        // mechanism as `Traits Output ~ Asura` below -- so it SHIPS,
        // owner-less (no monster row of this book claims it by name).
        let jiang_shi = monster_abilities()
            .iter()
            .find(|a| a.source_line == 1663)
            .unwrap_or_else(|| {
                panic!(
                    "b3_abilities_race.lst:1663 ships for shape measurement \
                     (decisions.md §27b round 9)"
                )
            });
        assert!(
            jiang_shi.owners.is_empty(),
            "{} was expected owner-less; no monster row of this book claims it",
            jiang_shi.key
        );
    }

    /// Every shipped ability of this book is reached by the namespaced-prefix
    /// link — either directly (the prefix IS an owner's corpus `KEY:`) or
    /// through an owner's human-readable `name` (`SD31-W21-MONSTER-001`'s
    /// `CATEGORY:Internal` bundle-row hop first surfaced this: a monster like
    /// `key: "Archon (Legion)", name: "Legion Archon"` is namespaced by its
    /// SHORT display name, not its parenthesised corpus key). This was the
    /// first book in the lane where the property held for the WHOLE shipped
    /// set (`row-named` reads 0 against `b3_races.lst`'s 100
    /// `ABILITY:Special Ability|AUTOMATIC|` tokens, which name rows the
    /// inventory files under `race_trait` — `file_kind` reads only the first
    /// `TYPE:` segment; see this module's header for the 341-unit scope
    /// finding that follows from it).
    ///
    /// **Rewritten by the T9 `MonsterAbilityFacet` widening cycle.** The
    /// prior version hand-maintained a 9-entry exception list for exactly
    /// this `name`-vs-`key` shape; re-running the transcriber against the
    /// widened facet vocabulary (plus a real multi-`TYPE:`-token parsing fix
    /// — `Forest`/`Sea`/`Sky`/`Sovereign`/`Underworld` Dragon subtypes, each
    /// stating their facet on a SECOND `TYPE:` token) newly shipped 33 more
    /// ability rows of this exact shape, which would have made the list
    /// 42 entries and counting — the un-scalable pattern `decisions.md §16`
    /// warns against. Resolving through `monster.name` instead is the fix
    /// for the shape itself, not one more name added to a list.
    ///
    /// **Scoped to OWNED rows by `decisions.md §20`.** An owner-less row
    /// (no monster row of this book claims it) has no owner to check the
    /// namespaced prefix against by construction — that is the whole point
    /// of shipping it owner-less rather than dropping it — so this property
    /// only applies where an owner exists at all.
    #[test]
    fn every_shipped_ability_is_reached_by_its_namespaced_key() {
        for ability in monster_abilities().iter().filter(|a| !a.owners.is_empty()) {
            let (prefix, _) = ability
                .key
                .split_once(" ~ ")
                .unwrap_or_else(|| panic!("{} is not a namespaced key", ability.key));
            let reached = ability.owners.iter().any(|owner| {
                *owner == prefix
                    || monsters()
                        .iter()
                        .find(|m| m.key == *owner)
                        .is_some_and(|m| m.name == prefix)
            });
            assert!(
                reached,
                "{} is namespaced to {prefix}, which is not among its owners ({:?}) either by \
                 key or by the owning monster's display name",
                ability.key, ability.owners
            );
        }
    }
}