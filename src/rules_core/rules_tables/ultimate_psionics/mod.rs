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
//! **21 monsters + 15 monster abilities = 36 records**, against corpus unit
//! counts of 21 and 79. 34 shipped before `SD31-W21-MONSTER-001` (below); that
//! round's `CATEGORY:Internal` bundle-row ownership hop resolved the 2
//! bundle-owned abilities the "66 orphans" section already named.
//! `python3 scripts/classify_monster_ability_rows.py ultimate_psionics` still
//! reports the pre-hop shape (`ultimate_psionics  21  79  3  10  66  0  0`) —
//! it has no awareness of the bundle hop, same caveat `inner_sea_gods`'s own
//! module doc now carries.
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
//! # The 66 orphans, and the 2 the bundle hop resolves (`SD31-W21-MONSTER-001`)
//!
//! 64 of the 66 remain genuinely orphaned: the Astral Construct menu
//! (`Astral_*`) and three namespaced rows (`Energy Touch ~ …`,
//! `Astral Warrior ~ …`, `Horror ~ …`) whose namespace is not a monster key of
//! this book. They stay `not-ingested`, which is their honest status.
//!
//! **Two were not orphans in the corpus** — `Naturally Psionic` and
//! `Psionic Aptitude`, and both now ship. Ten of this book's monster rows
//! carry `ABILITY:Internal|AUTOMATIC|Racial Traits ~ <Race>`, and those
//! `CATEGORY:Internal` bundle rows name both abilities — exactly the
//! ownership class `decisions.md §62.4` recorded for `inner_sea_gods` and
//! asked a successor to scan for; round 10's scan found it corpus-wide (**229
//! units across six books**, `decisions.md §64.1`), and
//! `scripts/transcribe_monster_tables.py::find_internal_bundle_ability_refs`
//! (`SD31-W21-MONSTER-001`) is that hop, wired: for every monster row's
//! `ABILITY:Internal|AUTOMATIC|<bundle_key>` reference it finds the
//! `CATEGORY:Internal` row named `bundle_key` and credits the monster with
//! every ability that row names.

pub mod aegis_features;
pub mod archetype_tables;
pub mod cryptic_features;
pub mod dread_features;
pub mod equipment_tables;
pub mod feat_tables;
pub mod marksman_features;
pub mod psion_features;
pub mod psychic_warrior_features;
pub mod soulknife_features;
pub mod tactician_features;
pub mod vitalist_features;
pub mod wilder_features;

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

    /// What ships is 21 and 127, against corpus unit counts of 21 and 176
    /// (`docs/work-inventory.json`'s current `monster_ability` count for this
    /// book, up from the 79 the original round-10 transcription ran
    /// against).
    ///
    /// Every corpus monster row of this book ships — no `NAMEISPI:YES`, no
    /// `.COPY=` delta and no `.MOD` overlay reaches the monster side here.
    ///
    /// 13 -> 15 (SD31-W21-MONSTER-001, +2): the `CATEGORY:Internal` bundle-row
    /// ownership hop resolved 2 previously-orphaned ability rows.
    ///
    /// 15 -> 127 (SD-32 card 11, T9 onboarding, `decisions.md §19` sign-off /
    /// `§17` generic-pass discipline, +112): re-running `transcribe_monster_
    /// tables.py ultimate_psionics` against the current corpus/inventory
    /// found 112 rows reachable through the namespaced-prefix shape
    /// (`Astral Warrior ~ Link` etc. -- a monster row's own name as the
    /// ability row's key prefix) that the round-10 transcription's own
    /// snapshot had not yet resolved. 64 `Astral_`-namespaced rows remain
    /// genuine orphans (no monster row of this book owns a bundle named
    /// `Astral`) and are correctly still excluded -- named explicitly by the
    /// transcriber's own stderr, not silently dropped. Re-derive: `python3
    /// scripts/transcribe_monster_tables.py ultimate_psionics && cargo run
    /// --locked --release --bin gen_book_cache -- ultimate_psionics`.
    #[test]
    fn the_shipped_counts_are_the_reachable_ones() {
        assert_eq!(monsters().len(), 21, "every corpus monster row of this book ships");
        // 127 owned + 64 owner-less (`decisions.md §20`, no_record-to-zero
        // wave 2 follow-on) = 191. The 64 `Astral_`-namespaced rows this
        // module's own doc comment already names as genuine orphans now SHIP
        // instead of being excluded. The owner-less count is pinned
        // separately below
        // (`every_owner_less_ability_is_a_named_and_pinned_non_reach`).
        let owned = monster_abilities()
            .iter()
            .filter(|a| !a.owners.is_empty())
            .count();
        assert_eq!(owned, 127);
        assert_eq!(monster_abilities().len(), 191);
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

    /// Every OWNED ability's owner ships.
    ///
    /// **Superseded `decisions.md §20` for the owner-less half** (previously
    /// asserted every ability has a non-empty `owners`; the 64 genuinely
    /// orphaned `Astral_`-namespaced rows now ship for shape measurement
    /// instead, pinned separately below).
    #[test]
    fn every_ability_has_a_shipped_owner() {
        let monster_keys: HashSet<&str> = monsters().iter().map(|m| m.key).collect();
        for ability in monster_abilities() {
            for owner in ability.owners {
                assert!(
                    monster_keys.contains(owner),
                    "{} is owned by {owner}, which this book does not ship",
                    ability.key
                );
            }
        }
    }

    /// **Superseded `decisions.md §20` (no_record-to-zero wave 2 follow-on).**
    /// The 64 `Astral_`-namespaced rows no monster row of this book claims
    /// now SHIP with `owners: &[]`, and this test pins the EXACT set of
    /// records that carry one. `list_monster_catalog` never walks these
    /// directly (only a monster's own `ability_keys`), so shipping them does
    /// not surface a stub; each key is pinned separately, by name, in
    /// `reach_gate.rs::UNREACHED_RECORD_FINDINGS` under
    /// `("ultimate_psionics", "monster_abilities")` as a proven non-reach,
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
            64,
            "the number of owner-less (unreachable-by-design) monster_ability records \
             changed — re-derive this pin from a real \
             `scripts/transcribe_monster_tables.py ultimate_psionics` run, and update the \
             matching `reach_gate.rs::UNREACHED_RECORD_FINDINGS` entry to the same key set"
        );

        let mut hasher = DefaultHasher::new();
        unowned.hash(&mut hasher);
        let digest = hasher.finish();
        assert_eq!(
            digest, 0x20e3_944a_3d90_ea44,
            "the owner-less key SET changed (same count, different members) — re-derive and \
             update `reach_gate.rs::UNREACHED_RECORD_FINDINGS` to match exactly"
        );
    }

    /// The `Racial Traits ~` bundle finding, RESOLVED (`SD31-W21-MONSTER-001`):
    /// `Naturally Psionic` and `Psionic Aptitude` now ship, owned by all ten
    /// races the `CATEGORY:Internal` bundle row names. Was `no_internal_
    /// bundle_ability_ships_yet`, asserting the pre-hop emptiness; now asserts
    /// both keys ship AND carry the full ten-race owner set, so a future
    /// regression (a race silently dropped from the bundle row, or the hop
    /// breaking outright) is caught either way.
    #[test]
    fn the_bundle_owned_abilities_ship_with_every_named_race() {
        let expected_owners: &[&str] = &[
            "Blue", "Dromite", "Duergar ~ Psionic", "Elan", "Forgeborn", "Half-Giant", "Maenad",
            "Noral", "Ophiduan", "Xeph",
        ];
        for key in ["Naturally Psionic", "Psionic Aptitude"] {
            let ability = monster_abilities()
                .iter()
                .find(|a| a.key == key)
                .unwrap_or_else(|| panic!("{key} must ship, owned via the `Racial Traits ~` bundle row"));
            let mut owners = ability.owners.to_vec();
            owners.sort_unstable();
            let mut expected = expected_owners.to_vec();
            expected.sort_unstable();
            assert_eq!(owners, expected, "{key}'s owner set no longer matches the bundle row");
        }
    }
}
