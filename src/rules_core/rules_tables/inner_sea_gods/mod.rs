//! Inner Sea Gods book-level module. SD-29 Epic 5 extend, round 9 — the
//! monster / monster-ability chassis (`corpus-work-channels.md §9.2`).
//!
//! # What ships, and what the corpus holds
//!
//! **39 monsters + 154 monster abilities = 193 records**, against corpus unit
//! counts of 39 and 161. 116 shipped before `SD31-W21-MONSTER-001` (below);
//! that round's `CATEGORY:Internal` bundle-row ownership hop resolved 77 more,
//! leaving 2 genuine orphans (`Herald ~ Always Armed`/`Herald ~ Emissary`,
//! neither bundle- nor row-/prefix-reachable) and 5 Product-Identity drops
//! (unchanged by the hop — see Provenance below).
//!
//! `python3 scripts/classify_monster_ability_rows.py inner_sea_gods` reports
//! only the row-named/prefix ownership shapes and has no awareness of the
//! bundle hop below (`scripts/scan_monster_ability_bundle_rows.py` is the
//! instrument that does) — its own "orphan"/"reachable remainder" numbers are
//! therefore a pre-hop figure for this book, not this header's live count.
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
//! # The `Race Traits ~` bundle rows: RESOLVED (`SD31-W21-MONSTER-001`)
//!
//! **This section used to explain why zero of the sixteen
//! `support/isg_abilities_races_b4.lst` ability rows shipped. All sixteen now
//! ship, along with 61 more of this book's abilities reached the same way —
//! the mechanism this section names is wired, not merely recorded.**
//!
//! The corpus states an ability's owner one hop further out than a monster
//! row's own `ABILITY:Special Ability|AUTOMATIC|` token or an ability's
//! `<Monster> ~ <Ability>` namespace prefix can see:
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
//! row names the individual abilities. Neither of the transcriber's original
//! two passes reads a bundle row at all — the row-named pass reads
//! `ABILITY:Special Ability|AUTOMATIC|` tokens **on monster rows** only, and
//! the prefix pass matches an ability's namespace against a monster **key** —
//! and here the namespace is the creature's short name (`First Blade`,
//! `Skein Steward`, `Ahmuuth`) while the monster key is longer (`The First
//! Blade`, `Steward of the Skein`, `Psychopomp (Ahmuuth)`), so neither passes
//! reaches them.
//!
//! `scripts/transcribe_monster_tables.py::find_internal_bundle_ability_refs`
//! is the third pass this round added: for every monster row's
//! `ABILITY:Internal|AUTOMATIC|<bundle_key>` reference, it finds the
//! `CATEGORY:Internal` row named `bundle_key` in this book's own ability
//! files and credits the monster with every ability THAT row names. Sized
//! first by `scripts/scan_monster_ability_bundle_rows.py` (round 10,
//! `decisions.md §64.1`, 229 units across six books) before being wired —
//! this book alone accounted for 79 of the 81 orphans that scan found, and
//! the transcriber reproduces that exact number.
//!
//! Two genuine orphans remain (`Herald ~ Always Armed`/`Herald ~ Emissary`) —
//! neither names nor is named by a bundle row, so the hop correctly leaves
//! them unshipped rather than guessing an owner.
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
pub mod spell_list;

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

    /// What ships is 39 and 154, against corpus unit counts of 39 and 161.
    ///
    /// The monster count is the whole corpus set — this is the first book in
    /// the registry to lose no monster row at all to any screen: no
    /// `NAMEISPI:YES`, no `.COPY=` delta, no `.MOD` overlay, and no cascade from
    /// a Product Identity ability it names.
    ///
    /// 77 -> 154 (SD31-W21-MONSTER-001, +77): the `CATEGORY:Internal`
    /// bundle-row ownership hop (`transcribe_monster_tables.py::
    /// find_internal_bundle_ability_refs`) resolved 77 of this book's 79
    /// bundle-reachable orphans — this is the book the hop mechanism's own
    /// docstring example (`ABILITY:Internal|AUTOMATIC|Race Traits ~ First
    /// Blade`, `support/isg_races_b4.lst:6` / `support/
    /// isg_abilities_races_b4.lst:8`) is drawn from. 5 more were newly-reached
    /// abilities the pre-existing Product Identity screen correctly drops (a
    /// named-deity term in an emitted value; see the Provenance section
    /// below), and 2 remain genuinely orphaned (`Herald ~ Always Armed`/
    /// `Herald ~ Emissary`, neither bundle- nor row-/prefix-reachable).
    #[test]
    fn the_shipped_counts_are_the_reachable_ones() {
        assert_eq!(monsters().len(), 39, "every corpus monster row of this book ships");
        // 154 -> 156 (T9 `MonsterAbilityFacet` widening cycle, +2): the
        // widened facet vocabulary shipped `Mother's Maw ~ Desecrate Aura`
        // (`TYPE:Aura.Supernatural`) and `Orsheval ~ Truespeech`
        // (`TYPE:Communicate.Supernatural`) — the exact `Communicate` shape
        // the widening cycle's own dispatch brief named. 1 owned row remains
        // excluded and named on stderr (`Xocothian ~ Speed Burst`,
        // `TYPE:ModifyMovement.Extraordinary`).
        // 156 owned + 2 owner-less (`decisions.md §20`, no_record-to-zero
        // wave 2 follow-on) = 158. The owner-less count is pinned separately
        // below (`every_owner_less_ability_is_a_named_and_pinned_non_reach`).
        // 156 owned -> 158 owned (`decisions.md §24`/round 7, +2): two
        // OWNED rows whose clean name/key had an undeclared blacklist hit
        // confined to `DESC:` now ship with the description redacted
        // instead of being dropped (`Grim White Stag ~ Bugle`,
        // `Thyrlien ~ Starlight Blast` — clean names, the "2
        // description-only PI" group T9 round 6 named). 2 owner-less -> 5
        // owner-less (+3): three ability rows at
        // `isg_abilities_races.lst:43/44/45`, whose own KEY namespace
        // matched the blacklist, now ship under a Codex-generated neutral
        // key instead of being dropped; all three are orphans here (no
        // monster row of this book claims them), pinned below. 158 -> 163
        // total (+2 owned +3 owner-less).
        // 158/163 -> 159/164 (`decisions.md §27`/round 8, +1 owned): the
        // one previously-excluded `Xocothian ~ Speed Burst` row (this
        // comment's own paragraph above, `TYPE:ModifyMovement.
        // Extraordinary`, delivery-only) now ships with a PROVISIONAL
        // `SpecialQuality` facet default instead of being dropped.
        let owned = monster_abilities()
            .iter()
            .filter(|a| !a.owners.is_empty())
            .count();
        assert_eq!(owned, 159);
        assert_eq!(monster_abilities().len(), 164);
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

    /// Every OWNED ability's owner ships.
    ///
    /// **Superseded `decisions.md §20` for the owner-less half** (previously
    /// asserted every ability has a non-empty `owners`; the 2 genuinely
    /// orphaned rows named in this module's header now ship for shape
    /// measurement instead, pinned separately below).
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
    /// `Herald ~ Always Armed` and `Herald ~ Emissary` — neither bundle- nor
    /// row-/prefix-reachable — now ship with `owners: &[]` for shape
    /// measurement instead of being dropped. `list_monster_catalog` never
    /// walks these directly (only a monster's own `ability_keys`), so
    /// shipping them does not surface a stub; each key is pinned separately,
    /// by name, in `reach_gate.rs::UNREACHED_RECORD_FINDINGS` under
    /// `("inner_sea_gods", "monster_abilities")` as a proven non-reach, not a
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

        assert_eq!(
            unowned.len(),
            5,
            "the number of owner-less (unreachable-by-design) monster_ability records \
             changed — re-derive this pin from a real \
             `scripts/transcribe_monster_tables.py inner_sea_gods` run, and update the matching \
             `reach_gate.rs::UNREACHED_RECORD_FINDINGS` entry to the same key set. 2 -> 5 \
             (`decisions.md §24`/round 7, +3): three name-PI ability rows now ship under a \
             neutral key instead of being dropped -- see `the_shipped_counts_are_the_reachable_\
             ones`'s own comment."
        );
        assert_eq!(
            unowned,
            vec![
                "Codex-Named Unit (monster_ability_inner_sea_gods_isg_abilities_races_lst_43)",
                "Codex-Named Unit (monster_ability_inner_sea_gods_isg_abilities_races_lst_44)",
                "Codex-Named Unit (monster_ability_inner_sea_gods_isg_abilities_races_lst_45)",
                "Herald ~ Always Armed",
                "Herald ~ Emissary",
            ]
        );

        let mut hasher = DefaultHasher::new();
        unowned.hash(&mut hasher);
        let digest = hasher.finish();
        assert_eq!(
            digest, 0x137d_2d8a_116e_9f2b,
            "the owner-less key SET changed (same count, different members) — re-derive and \
             update `reach_gate.rs::UNREACHED_RECORD_FINDINGS` to match exactly"
        );
    }

    /// The `Race Traits ~` bundle finding, RESOLVED (`SD31-W21-MONSTER-001`):
    /// all sixteen `support/isg_abilities_races_b4.lst` ability rows now ship,
    /// owned by their real monster through the `CATEGORY:Internal` bundle-row
    /// hop the module header describes. Was `no_support_directory_ability_
    /// ships_yet`, asserting the PRE-hop emptiness; now asserts the sixteen
    /// real keys by name, so a future regression (the hop breaking, or this
    /// book's ownership shape changing upstream) is caught by name rather
    /// than by a silent count-only pin.
    #[test]
    fn the_support_directory_bundle_abilities_ship() {
        let shipped: std::collections::BTreeSet<&str> = monster_abilities()
            .iter()
            .filter(|a| a.source_file == "isg_abilities_races_b4.lst")
            .map(|a| a.key)
            .collect();
        let expected: std::collections::BTreeSet<&str> = [
            "First Blade ~ Powerful Blows (Slam)",
            "First Blade ~ Regeneration",
            "First Blade ~ Bladed Slam",
            "First Blade ~ Swarm Form",
            "First Blade ~ Lord of Battle",
            "First Blade ~ Rage Aura",
            "First Blade ~ Ironsense",
            "Skein Steward ~ Immunity to Possession",
            "Skein Steward ~ Fate Aura",
            "Skein Steward ~ Gaze",
            "Skein Steward ~ Change Shape",
            "Skein Steward ~ Constant Spells",
            "Skein Steward ~ Tugging Strands",
            "Ahmuuth ~ Animated Shield",
            "Ahmuuth ~ Death's Dagger",
            "Ahmuuth ~ Ectoplasmic Focus",
        ]
        .into_iter()
        .collect();
        assert_eq!(
            shipped, expected,
            "support/isg_abilities_races_b4.lst's shipped set no longer matches the sixteen \
             bundle-reached rows the module header names"
        );
    }
}
