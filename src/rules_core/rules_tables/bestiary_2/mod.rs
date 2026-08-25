//! Bestiary 2 (`SOURCESHORT:B2`) — `companion`, `monster`, `monster_ability`.
//!
//! # The monster half (SD-29 Epic 5 extend, round 4)
//!
//! **314 of 316 monster rows and 401 of 466 ability rows ship.** This is the
//! largest book in the monster lane by an order of magnitude: the five books in
//! `monster_chassis::MONSTER_BOOKS` before it hold **34** monsters between them,
//! and even counting Bestiary 1's 46 SD-22 records — served by the same catalog
//! but not through this chassis — the whole prior population is 80. It is also
//! the first `roleplaying_game/` bestiary taken since the Bonus Bestiary pilot.
//!
//! ```text
//! python3 scripts/classify_monster_ability_rows.py bestiary_2
//! book         mon  abil row-named prefix ORPHAN   PI
//! bestiary_2   316   466       398      4     64    0
//! ```
//!
//! ## Zero Product Identity rows, and why that is a fact rather than a hope
//!
//! `decisions.md §50.1` found that PCGen declares per-record Product Identity
//! with `NAMEISPI:YES` and that nothing in this repo had ever read it. This
//! book carries the marker on **no** row —
//! `grep -c 'NAMEISPI:YES' b2_races.lst b2_abilities_race.lst` → `0` and `0` —
//! and the term-list screen finds nothing either. `ogl-pi-blacklist.md` §2
//! predicts exactly that shape: the Product Identity in a Pathfinder book is
//! its setting-specific proper nouns, which live in the `campaign_setting/`
//! line, while a `roleplaying_game/` bestiary's monster names are presumptively
//! Open Game Content. Round 3's book (`campaign_setting/inner_sea_world_guide`) sat on
//! the other side of that split and lost 5 of 14 monsters to it.
//!
//! The absence is held by a test against the live blacklist
//! (`no_shipped_monster_field_carries_a_product_identity_term`), not by the
//! grep above: the grep is a statement about today.
//!
//! ## The `.COPY=` rows, and the only two in the corpus
//!
//! `b2_races.lst:454` and `:594` are `<Base>.COPY=<Variant>` rows: PCGen copies
//! the base record whole and applies the few tokens the copy row carries. They
//! are the ONLY two `monster`/`monster_ability` units of `origin: copy` in the
//! entire corpus —
//! `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
//! print(sum(1 for u in d['units'] if u['kind'] in ('monster','monster_ability')
//! and u.get('origin')=='copy'))"` → `2`.
//!
//! Transcribed verbatim — which is the only thing this lane's transcriber does
//! — each produces a card carrying a challenge rating and nothing else: no
//! size, no speed, no type, no page. `gen_book_cache::verified_citation_line`
//! refuses them outright, because the row's first column reads
//! `<Base>.COPY=<Variant>` and not the record's name. That gate is what
//! surfaced this; it was working.
//!
//! Resolving the delta is not transcription. It composes values across two rows
//! while [`MonsterStatBlock`] carries ONE `source_file`/`source_line` pair, so
//! every inherited field would ship under a citation that does not contain it —
//! the stale-citation defect that gate exists to catch. A chassis that models
//! inheritance needs a second citation and a deliberate widening; two records
//! is not a reason to slip one into an ingest round. Dropping
//! `b2_races.lst:594` cascades to its one ability, which becomes the 65th
//! orphan.
//!
//! ## 65 orphan ability rows, and the three shapes they take
//!
//! An ability reaches a player only underneath its monster, so a row no monster
//! row of this book claims would load and never be shown — the
//! record-that-is-never-seen class `decisions.md §44.2` is about. The 65 are
//! left `not-ingested`, which is their honest status, and are cited by line in
//! `monster_data.rs`'s generated header. They namespace to `b2_templates.lst`
//! templates (the `Draconal` family), to monsters defined in other books, and
//! to rows this book only `.MOD`s. A template surface would close the first
//! group; nothing in this chassis can.
//!
//! ## Both chassis in one module
//!
//! This is the first book module to carry the companion chassis and the monster
//! chassis at once, and each defines a `NaturalAttack` and a `Speed`. Only the
//! companion pair is re-exported by bare name; see the `pub use` below.
//!
//! # The lane's first FAMILIAR book, and the first book in it that another lane
//! also wants
//!
//! Every companion book registered before this one contributes
//! `*_races_companion.lst` rows. B2's 16 companion units are
//! `*_races_familiar.lst` and `*_abilities_familiar_race.lst` — the same kind by
//! `v06_work_inventory::file_kind`, and the same two structural shapes, but the
//! creature rows are `TYPE:Companion.Familiar.Animal` wizard/witch familiars
//! rather than druid animal companions.
//!
//! ```text
//! python3 scripts/classify_companion_rows.py bestiary_2
//! book                              crea  abil  clas  named  prerace  prefix  ORPHAN
//! bestiary_2                          15     1     0      1        0       1       0
//! ```
//!
//! **`RuleSetId::B2` compiles this book's `companion` family and nothing else.**
//! B2 also holds 782 `monster` / `monster_ability` units, which are the
//! monster lane's (`decisions.md §46`, round-3 target list). Registering the
//! rule set moves those units from `not-started` to `not-ingested` — a status
//! relabel that states the truth more precisely, since the engine now compiles
//! part of this book — and does not claim them.
//!
//! # One ability row, and both ownership shapes disagree about how it is owned
//!
//! `Snapping Turtle ~ Shell` is claimed by `prefix` (its namespaced `KEY:`
//! resolves through the `Familiar (Snapping Turtle)` wrapper) and by `named`
//! (the creature row's `ABILITY:Special Ability|AUTOMATIC|` names it) but NOT by
//! `prerace` — the row carries no `PRERACE:` at all. It is recorded once; the
//! chassis dedupes on the key.

mod companion_data;
mod monster_data;

pub use super::companion_chassis::{
    CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord,
    NaturalAttack, Speed, StatAdjustment,
};

// The monster chassis' own `NaturalAttack` and `Speed` are deliberately NOT
// re-exported here: this is the first book module to carry BOTH chassis, and
// each defines a type of each name. `monster_data.rs` names them through
// `monster_chassis` directly, so nothing needs the shortcut, and re-exporting
// one of the two pairs under a bare name is exactly how a later reader ends up
// building a monster's speed out of a companion's struct.
pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
};

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

    /// From `docs/work-inventory.json`'s own units for this book: 16 companion
    /// units, 15 creature rows and 1 ability row.
    #[test]
    fn the_book_defines_fifteen_familiars_and_one_ability() {
        assert_eq!(companions().len(), 15);
        assert_eq!(companion_abilities().len(), 1);
    }

    /// Every creature row in this book is a FAMILIAR, not an animal companion —
    /// the property that makes this the first book of its shape in the lane, and
    /// the one a `*_races_companion.lst`-shaped reader would have quietly got
    /// wrong.
    #[test]
    fn every_creature_row_is_a_familiar() {
        for companion in companions() {
            assert!(
                companion.key.starts_with("Familiar ("),
                "{} is not a familiar",
                companion.key
            );
            assert!(
                companion.type_segments.contains(&"Familiar"),
                "{} does not state Familiar in its TYPE:",
                companion.key
            );
        }
    }

    /// Verbatim spot-check against `b2_abilities_familiar_race.lst:6` and the
    /// creature row that names it. The link closes in both directions on the
    /// book's only ability.
    #[test]
    fn the_snapping_turtles_shell_matches_its_corpus_row_and_its_owner() {
        let shell = &companion_abilities()[0];
        assert_eq!(shell.key, "Snapping Turtle ~ Shell");
        assert_eq!(shell.name, "Shell");
        assert_eq!(shell.facet, Some(CompanionAbilityFacet::SpecialQuality));
        assert_eq!(shell.delivery, Some(CompanionAbilityDelivery::Extraordinary));
        assert_eq!(shell.source_page, Some("p.273"));
        assert_eq!(shell.owners, &["Familiar (Snapping Turtle)"]);

        let turtle = companions()
            .iter()
            .find(|c| c.key == "Familiar (Snapping Turtle)")
            .expect("the Snapping Turtle is in this book");
        assert!(turtle.ability_keys.contains(&"Snapping Turtle ~ Shell"));
    }

    /// The other 14 familiars carry no ability of this book, and their
    /// `external_ability_refs` say so rather than the table silently holding an
    /// empty list on both sides.
    #[test]
    fn the_other_fourteen_familiars_name_only_abilities_this_book_does_not_define() {
        let with_ability: Vec<_> = companions()
            .iter()
            .filter(|c| !c.ability_keys.is_empty())
            .map(|c| c.key)
            .collect();
        assert_eq!(with_ability, vec!["Familiar (Snapping Turtle)"]);
    }

    // ---- SD-29 Epic 5 extend, round 4: the `monster` / `monster_ability`
    // half of this book. See the module doc comment's "The monster half"
    // section for the derivation.

    /// The corpus unit counts are `docs/work-inventory.json`'s own, never a
    /// line count over the `.lst`:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='bestiary_2'
    /// and u['kind']=='monster'))"` → 316, `monster_ability` → 466.
    ///
    /// The trap report reads 322 DECLARES on `b2_races.lst` — six more than the
    /// inventory's 316. The difference is `.COPY=` rows the inventory's own trap
    /// filters drop; the two that survive as units are the two this table
    /// withholds, for the reason the module doc gives.
    ///
    /// **314 and (corrected `SD31-E6-F9-005`, was 401) 493.** This book
    /// carries no Product Identity in either signal, so nothing is withheld
    /// for that; the two withheld monster rows are `.COPY=` deltas. Of the
    /// 401 owned-but-unshipped ability rows the transcriber used to leave on
    /// the table, 92 were reachable and cleanly parseable, silently blocked
    /// only because `transcribe()` `raise SystemExit`'d the instant it found
    /// the 2 OTHER unmodelled-`DESC:`-shape rows anywhere in the book
    /// (`OPEN-ISSUES.md` row 157/206). The fix drops just those 2 (named in
    /// this book's own module doc comment above) instead of crashing the
    /// whole run, and the other 92 now ship for real: 401 + 92 = 493.
    #[test]
    fn the_book_ships_every_stat_block_and_every_owned_ability() {
        assert_eq!(monsters().len(), 314);
        // 493 -> 511 (SD31-W21-MONSTER-001, +18): the `CATEGORY:Internal`
        // bundle-row ownership hop resolved 18 previously-orphaned ability
        // rows this book's monsters name only indirectly.
        // 511 -> 571 (T9 `MonsterAbilityFacet` widening cycle, +60): the
        // widened facet vocabulary (`Weakness`/`Defensive`/`Aura`/`Sense`/
        // `Communicate`) shipped 60 more owned, reachable ability rows that
        // previously carried a `TYPE:` shape the chassis did not model. 9
        // owned rows remain excluded and named on stderr — 3 with no
        // `TYPE:` token at all, 2 bare-delivery-only (`Extraordinary` with
        // no facet segment), and 4 needing a per-record read
        // (`ModifyHP`/`Spelllike`-typo/`SpecialAttck`-typo/an
        // un-investigated `Bunyip ~ Blood Rage`) — see
        // `scripts/transcribe_monster_tables.py bestiary_2`'s own stderr for
        // the live list.
        // 571 owned + 85 owner-less (`decisions.md §20`, no_record-to-zero
        // wave 2 follow-on) = 656. The owner-less count is pinned separately
        // below (`every_owner_less_ability_is_a_named_and_pinned_non_reach`),
        // so this assertion is now over the OWNED subset only.
        // 571/656 -> 572/657 (`decisions.md §22`/round 6, +1 owned): the
        // `SpecialAttck` typo-fold resolved `Tick Swarm ~ Cling`'s facet for
        // the first time -- this file's own pins were missed when round 6
        // bumped the identical delta in `apps/desktop/src-tauri/src/
        // reach_gate.rs`; re-derived here, not caused by this cycle's own
        // diff (`git diff --stat` for `bestiary_2/monster_data.rs` shows
        // zero deletions, only the 3 trailing `codex_generated_name`/
        // `rename_*` fields appended per record).
        // 572/657 -> 580/665 (`decisions.md §27`/round 8, +8 owned): the
        // `TYPE:`-facet-vocabulary-gap group closes via the provisional
        // `SpecialQuality` default -- all 8 are namespaced `<Monster> ~
        // <Ability>` keys whose owner resolves through the existing prefix
        // pass (`Aurumvorax ~ Rake`, `Bunyip ~ Blood Rage`, `Carnivorous
        // Blob ~ Split`, `Denizen of Leng ~ Planar Fast Healing`, `Howler ~
        // Abyssal Strike`, `Lamia Matriarch ~ Spells`, `Mothman ~ Agent of
        // Fate`, `Yrthak ~ Sonic Lance`), so all 8 land in `owned`, none in
        // the owner-less pin below.
        // 580/665 -> 582/667 (`decisions.md §27b` round 9, +2 owned): the
        // multi-DESC: `PREVAREQ`/`PREVARGT`-gated parse-refusal group closes
        // via `parse_desc`'s new generalised sixth branch -- `Telepathy ~
        // Miles` and `Voidworm ~ Change Shape` (`ce_abilities_race.lst:1955`/
        // `:2043`, round 6's own named 2-row `bestiary_2` share) both
        // resolve a real owner through the existing prefix pass, so both
        // land in `owned`, none in the owner-less pin below.
        let owned = monster_abilities()
            .iter()
            .filter(|a| !a.owners.is_empty())
            .count();
        assert_eq!(owned, 582);
        assert_eq!(monster_abilities().len(), 667);
    }

    /// **Superseded `decisions.md §20` (no_record-to-zero wave 2 follow-on).**
    /// An owner-less ability row no longer forbids shipping: an un-ingested
    /// row's shape cannot be measured, so the 85 rows no monster row of this
    /// book claims now SHIP with `owners: &[]`, and this test's job changes
    /// from "forbid an empty owner list" to "pin the EXACT set of records
    /// that carry one" — a silent new arrival OR a silent disappearance both
    /// fail here, by name, the same discipline
    /// `rules_tables::bestiary::tests::every_owner_less_ability_is_a_named_and_pinned_non_reach`
    /// established. `list_monster_catalog` never walks these directly (only
    /// a monster's own `ability_keys`), so shipping them does not surface a
    /// stub; each key is pinned separately, by name, in `reach_gate.rs::
    /// UNREACHED_RECORD_FINDINGS` under `("bestiary_2", "monster_abilities")`
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
            85,
            "the number of owner-less (unreachable-by-design) monster_ability records \
             changed — re-derive this pin from a real \
             `scripts/transcribe_monster_tables.py bestiary_2` run, and update the matching \
             `reach_gate.rs::UNREACHED_RECORD_FINDINGS` entry to the same key set"
        );

        let mut hasher = DefaultHasher::new();
        unowned.hash(&mut hasher);
        let digest = hasher.finish();
        assert_eq!(
            digest, 0xfb07_0eb2_4302_5d02,
            "the owner-less key SET changed (same count, different members) — re-derive and \
             update `reach_gate.rs::UNREACHED_RECORD_FINDINGS` to match exactly"
        );
    }

    /// Every owner named by a shipped ability is itself a shipped monster.
    ///
    /// Not implied by the test above: an ability naming a monster this book
    /// does not ship satisfies "owners is non-empty" and still points at a
    /// creature the catalog cannot render.
    #[test]
    fn every_owner_named_by_a_shipped_monster_ability_is_a_shipped_monster() {
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

    /// This book declares no Product Identity, and the assertion is made
    /// against the LIVE list rather than against a grep that returned 0 today.
    ///
    /// `grep -c 'NAMEISPI:YES' b2_races.lst b2_abilities_race.lst` → `0` and
    /// `0`, so the upstream marker `decisions.md §50.1` found is absent here;
    /// this covers the other signal, and fails if a per-book override ever adds
    /// a term one of these 715 records matches.
    ///
    /// **Two predicates, and the difference is not cosmetic.** Identity fields
    /// (`key`, `name`) are screened case-INSENSITIVELY, because a deity name
    /// reaching a key in any casing is a real hit. Rules TEXT is screened with
    /// `pi_screening`'s own case-sensitive `contains`, the predicate
    /// `gen_book_cache::monster_record_pi_hits` applies as a hard stop, because
    /// that is the authoritative screen and prose is where a loosened one
    /// starts eating Open Game Content.
    ///
    /// This is `decisions.md §50.4`'s over-exclusion lesson, measured. A first
    /// draft screened description prose case-insensitively too and reported
    /// **13** hits in this book — every one a false positive, a short blacklist
    /// term sitting inside an ordinary English word that happens to contain it
    /// (12 of the 13 on one such term, 1 on another).
    ///
    /// **The two terms are named in `decisions.md §52.2`, not here, and that is
    /// itself the finding.** `pi_table_sweep` and `scripts/verify.sh`'s
    /// `pi-sweep` reject a Product Identity term anywhere under `rules_tables/`
    /// and neither reads intent: a comment recording a FALSE positive
    /// instantiates the name exactly as a comment recording a real removal
    /// does. `decisions.md §50` learned that for removals; this round re-learned
    /// it for false alarms, from a red gate.
    ///
    /// Case-sensitively, and over identity fields in any casing, this book is
    /// clean.
    #[test]
    fn no_shipped_monster_field_carries_a_product_identity_term() {
        let terms = crate::rules_core::pi_screening::PI_BLACKLIST_TERMS;
        let identity: Vec<String> = monsters()
            .iter()
            .map(|m| format!("{} {}", m.key, m.name).to_ascii_lowercase())
            .chain(
                monster_abilities()
                    .iter()
                    .map(|a| format!("{} {}", a.key, a.name).to_ascii_lowercase()),
            )
            .collect();
        for haystack in &identity {
            for term in terms {
                assert!(
                    !haystack.contains(&term.to_ascii_lowercase()),
                    "a shipped Bestiary 2 key or name matches a Product Identity term; \
                     the record must not be ingested at all"
                );
            }
        }
        for ability in monster_abilities() {
            let text = ability.description.unwrap_or("");
            for term in terms {
                assert!(
                    !text.contains(term),
                    "{}'s rules text carries the Product Identity term {term:?}",
                    ability.key
                );
            }
        }
    }

    /// Verbatim spot-check of both halves of one link against the corpus rows
    /// they were read from — `b2_races.lst:14` and `b2_abilities_race.lst:6`.
    ///
    /// Every asserted value is a substring of its row: the transcriber computes
    /// nothing, and this is the check that says so for a record a reader can
    /// open the `.lst` and confirm.
    #[test]
    fn the_achaierai_matches_its_corpus_row_and_its_one_ability() {
        let monster = monsters()
            .iter()
            .find(|m| m.key == "Achaierai")
            .expect("b2_races.lst:14");
        assert_eq!(monster.size, Some("L"));
        assert_eq!(monster.race_type, Some("Outsider"));
        assert_eq!(monster.challenge_rating, Some("5"));
        assert_eq!(monster.source_page, Some("p.7"));
        assert_eq!(monster.source_file, "b2_races.lst");
        assert_eq!(monster.source_line, 14);
        assert_eq!(monster.speeds.len(), 1);
        assert_eq!(monster.speeds[0].mode, "Walk");
        assert_eq!(monster.speeds[0].feet, 50);
        assert_eq!(monster.ability_keys, &["Achaierai ~ Black Cloud"]);

        let ability = monster_abilities()
            .iter()
            .find(|a| a.key == "Achaierai ~ Black Cloud")
            .expect("b2_abilities_race.lst:6");
        assert_eq!(ability.name, "Black Cloud");
        assert_eq!(ability.facet, MonsterAbilityFacet::SpecialAttack);
        assert_eq!(ability.delivery, Some(MonsterAbilityDelivery::Supernatural));
        assert_eq!(ability.source_page, Some("p.7"));
        assert_eq!(ability.owners, &["Achaierai"]);
        assert_eq!(ability.source_line, 6);
        // The `%1`/`%2` in the corpus text name this row's own two `DESC:`
        // variables; the description is carried verbatim, placeholders and all,
        // because substituting them would be computing a value the row states
        // as a formula.
        assert!(
            ability
                .description
                .expect("the row carries a DESC:")
                .contains("DC %1 Fortitude save"),
            "the description is the row's own text, not a rendered one"
        );
        assert_eq!(
            ability.description_variables,
            &["BlackCloudDC", "BlackCloudDC"]
        );
    }

    /// **Superseded `decisions.md §20`.** The 805-line row (cascaded from the
    /// dropped `.COPY=` variant at `b2_races.lst:594`) now ships as an
    /// owner-less record instead of being excluded — it is one of the 85
    /// pinned by `every_owner_less_ability_is_a_named_and_pinned_non_reach`
    /// above. Confirmed present, not merely absent-from-exclusion.
    #[test]
    fn the_copy_cascaded_orphan_ships_owner_less() {
        let ability = monster_abilities()
            .iter()
            .find(|a| a.source_line == 805)
            .expect("b2_abilities_race.lst:805 ships for shape measurement (decisions.md §20)");
        assert!(
            ability.owners.is_empty(),
            "{} was expected owner-less (its only owner is the un-transcribed `.COPY=` row at \
             b2_races.lst:594)",
            ability.key
        );
    }
}
