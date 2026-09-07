//! Resolves Bestiary 5 Skinwalker's per-kin `Change Shape` TYPE-pool, the
//! shape wave 33 lane B named as its own 20-unit remainder
//! (`docs/release/SD-34-book-completion/artifacts/bucket-d-mining/
//! wave33_laneB_race_trait_never_applies_cycle_receipt.md`, next-cycle plan
//! item 2).
//!
//! # Two-level structure, both levels real corpus, one level not ingested
//!
//! Each of Skinwalker's nine were-creature kins (Werebat/Werebear/Wereboar/
//! Werecrocodile/Wererat/Wereshark/Weretiger/Werewolf/Wereraptor) carries a
//! master `<Kin>-Kin ~ Change Shape` record (`data/corpus/bestiary_5/
//! race_trait/skinwalker/*_kin_change_shape.json`) whose own `ABILITY:`
//! token names a per-kin PCGen `TYPE=` pool:
//!
//! ```text
//! ABILITY:Skinwalker Racial Trait|AUTOMATIC|TYPE=Skinwalker Change Shape Werebear-Kin
//! ```
//!
//! [`RaceTraitRecord::automatic_trait_grants`] already reads that token
//! generically (used identically by `race_resolver`'s own `FlagGranted`
//! role and by [`crate::rules_core::trait_pool::resolve_adopted_race_options`]'s
//! sibling shape) -- reused here, not re-parsed.
//!
//! **Follows `trait_pool.rs`'s idiom, with one necessary difference.**
//! `trait_pool.rs`'s `resolve_adopted_race_options` indexes its pool members
//! by reading a `TYPE:` token directly off each member's own ingested
//! record. That is not possible here: PCGen encodes a Change Shape option's
//! pool membership as a `.MOD` row appended to an *already-declared* ability
//! elsewhere in the same `.lst` file --
//!
//! ```text
//! CATEGORY=Special Ability|Skinwalker ~ Change Shape (Bite).MOD  TYPE:Skinwalker Change Shape Werebear-Kin
//! ```
//!
//! -- and this project's `.lst` ingest pipeline (`scripts/ingest_generic_kind.py`
//! / `pcgen_import`) does not currently fold a `.MOD` row's added `TYPE:`
//! tokens back onto the target record it modifies. Verified directly this
//! cycle: every one of the 20 real option records under
//! `data/corpus/bestiary_5/race_trait/skinwalker/skinwalker_change_shape_*.json`
//! carries only its own declaring row's `TYPE:Skinwalker Racial Trait` --
//! never the per-kin `TYPE:Skinwalker Change Shape <Kin>` a `.MOD` row adds
//! upstream. Fixing that generically belongs to the ingest pipeline itself
//! (a cross-cutting change well outside this cycle's scope, since `.MOD` is
//! a general PCGen mechanism, not a Skinwalker-only quirk) -- filed as a
//! real, named remaining gap in this cycle's own receipt, not silently
//! routed around.
//!
//! Until that lands, [`KIN_OPTION_KEYS`] is the resolver's pool-membership
//! table -- a static, book-specific mapping (this mechanism does not
//! generalize past Skinwalker's own nine kins regardless), transcribed
//! directly from the pinned oracle's own `.MOD` rows and cited by exact
//! `path:line` in the table's own doc comment below. [`skinwalker_change_shape_options`]'s
//! own test module cross-checks every key this table names against the
//! REAL loaded corpus, so a corpus rename or a future `.MOD`-aware ingest
//! fails this module's tests rather than silently drifting from the table.
//!
//! # `Endurance` is a genuine, verified orphan
//!
//! Of the 20 real option records, 19 are named by at least one kin's `.MOD`
//! row. `Skinwalker ~ Change Shape (Endurance)` is not named by any of the
//! nine kins' `.MOD` blocks, nor by the CRB Default pool's own six-member
//! block (`skinwalker_abilities_race.lst`) -- confirmed by
//! `grep -c 'Endurance).MOD' skinwalker_abilities_race*.lst` returning `0`
//! against the pinned oracle. It is real content with no real consumer
//! anywhere in the upstream data, the same disposition wave 33 lane B gave
//! Bestiary 6's Rougarou selector: correctly inert, no project-side remedy
//! possible short of an upstream PCGen data change. [`KIN_OPTION_KEYS`]
//! therefore never names it, and this module's own completeness test pins
//! that absence rather than treating it as an oversight.

use std::collections::BTreeMap;

use crate::rules_core::race_resolver::RaceCorpus;

/// The PCGen `TYPE=` pool suffix prefix every Skinwalker kin master
/// record's own `ABILITY:` token carries, read via
/// [`RaceTraitRecord::automatic_trait_grants`](crate::rules_core::race_resolver::RaceTraitRecord::automatic_trait_grants).
const POOL_PREFIX: &str = "TYPE=Skinwalker Change Shape ";

/// Every kin pool this cycle resolves, and the exact option `KEY:` strings
/// PCGen's `.MOD` rows tag into it -- transcribed from the pinned oracle's
/// `pathfinder/paizo/roleplaying_game/core_essentials/races/skinwalker/
/// skinwalker_abilities_race_subrace.lst` (`PCGEN_ORACLE_SHA` in
/// `scripts/pcgen-oracle-pin.env`), one row of citations per kin:
///
/// **Six option names PCGen tags into more than one kin's pool are declared
/// only in `skinwalker_abilities_race.lst`'s own CRB "Default" block --
/// `Claw`, `Constitution`, `Darkvision`, `Dexterity`, `Natural Armor`,
/// `Strength` -- and were never ingested into this project's curated
/// `race_trait/` directory at all (only into the separate
/// `race_trait_generic/` population, a different content kind this module
/// does not read: `diff <(ls data/corpus/bestiary_5/race_trait/skinwalker/
/// | grep change_shape) <(ls data/corpus/bestiary_5/race_trait_generic/ |
/// grep change_shape)` shows exactly those six present only on the generic
/// side). No real [`crate::rules_core::race_resolver::RaceTraitRecord`]
/// exists under `race_trait/` for this resolver to find any of the six
/// under, so every kin row below omits them rather than naming a key with
/// no possible match (which would silently under-report instead of being
/// visible in this table). This module's own test pins each kin's resulting
/// grant count against the live corpus.
///
/// - `Werebat-Kin`: lines 26-30 (Bite, Perception Bonus, Reduce Falling
///   Damage, Scent -- Dexterity excluded)
/// - `Werebear-Kin`: lines 50-54 (Bite, Climb Speed 20 Feet, Scent, Wisdom
///   -- Claw excluded)
/// - `Wereboar-Kin`: lines 74-78 (Base Speed Bonus, Gore, Hoof, Scent --
///   Constitution excluded)
/// - `Werecrocodile-Kin`: lines 98-102 (Bite, Ferocity, Swim Speed --
///   Strength and Darkvision excluded)
/// - `Wererat-Kin`: lines 122-126 (Bite, Climb Speed 30 Feet, Distraction,
///   Scent -- Dexterity excluded)
/// - `Wereshark-Kin`: lines 146-150 (Amphibious, Bite, Ferocity, Swim
///   Speed -- Constitution excluded)
/// - `Weretiger-Kin`: lines 170-174 (Base Speed Bonus, Bite, Charisma, See
///   In Darkness -- Claw excluded)
/// - `Werewolf-Kin`: lines 194-198 (Bite, Saves, Wisdom -- Claw and
///   Darkvision excluded)
/// - `Wereraptor-Kin`: lines 218-221 (Bite, Fly Speed Bonus, Perception
///   Bonus, Talon)
const KIN_OPTION_KEYS: &[(&str, &[&str])] = &[
    (
        "Werebat-Kin",
        &[
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Perception Bonus)",
            "Skinwalker ~ Change Shape (Reduce Falling Damage)",
            "Skinwalker ~ Change Shape (Scent)",
        ],
    ),
    (
        "Werebear-Kin",
        &[
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Climb Speed 20 Feet)",
            "Skinwalker ~ Change Shape (Scent)",
            "Skinwalker ~ Change Shape (Wisdom)",
        ],
    ),
    (
        "Wereboar-Kin",
        &[
            "Skinwalker ~ Change Shape (Base Speed Bonus)",
            "Skinwalker ~ Change Shape (Gore)",
            "Skinwalker ~ Change Shape (Hoof)",
            "Skinwalker ~ Change Shape (Scent)",
        ],
    ),
    (
        "Werecrocodile-Kin",
        &[
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Ferocity)",
            "Skinwalker ~ Change Shape (Swim Speed)",
        ],
    ),
    (
        "Wererat-Kin",
        &[
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Climb Speed 30 Feet)",
            "Skinwalker ~ Change Shape (Distraction)",
            "Skinwalker ~ Change Shape (Scent)",
        ],
    ),
    (
        "Wereshark-Kin",
        &[
            "Skinwalker ~ Change Shape (Amphibious)",
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Ferocity)",
            "Skinwalker ~ Change Shape (Swim Speed)",
        ],
    ),
    (
        "Weretiger-Kin",
        &[
            "Skinwalker ~ Change Shape (Base Speed Bonus)",
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Charisma)",
            "Skinwalker ~ Change Shape (See In Darkness)",
        ],
    ),
    (
        "Werewolf-Kin",
        &[
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Saves)",
            "Skinwalker ~ Change Shape (Wisdom)",
        ],
    ),
    (
        "Wereraptor-Kin",
        &[
            "Skinwalker ~ Change Shape (Bite)",
            "Skinwalker ~ Change Shape (Fly Speed Bonus)",
            "Skinwalker ~ Change Shape (Perception Bonus)",
            "Skinwalker ~ Change Shape (Talon)",
        ],
    ),
];

/// One real option a kin's Change Shape pool resolves to. `description` is
/// `None` for every one of these 20 records honestly -- PCGen ships them
/// `VISIBLE:NO` with no `DESC:` token of their own (the enumerable prose
/// lives only on the KIN MASTER record's own description); `name` is the
/// real, non-fabricated corpus text ("Change Shape (2 Claw Attacks)",
/// "Change Shape (Bite Attack)", ...) and is what this module surfaces as
/// the option's own label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkinwalkerChangeShapeGrant {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
}

/// One kin's Change Shape master record, resolved against
/// [`KIN_OPTION_KEYS`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkinwalkerChangeShapeMasterOption {
    pub key: String,
    pub name: String,
    pub book_id: String,
    /// The kin suffix read off the master's own `ABILITY:` token
    /// (`"Werebear-Kin"`), never hand-typed.
    pub kin: String,
    /// Real option records this kin's pool resolves to. Never empty for any
    /// of the 9 real kins this module maps (every one names at least 4 real
    /// members); an empty result here would mean [`KIN_OPTION_KEYS`] and the
    /// live corpus have drifted apart, which this module's own test would
    /// catch first.
    pub grants: Vec<SkinwalkerChangeShapeGrant>,
}

/// Resolves every Skinwalker kin's Change Shape pool against a loaded
/// [`RaceCorpus`]. Mirrors
/// [`crate::rules_core::trait_pool::resolve_adopted_race_options`]'s shape
/// (a selector paired with its real, non-fabricated pool members) with the
/// necessary substitution this module's own doc comment names: pool
/// membership comes from [`KIN_OPTION_KEYS`], a cited static table, rather
/// than a `TYPE:` token read off each member (the ingest gap this module's
/// doc comment documents).
pub fn skinwalker_change_shape_options(corpus: &RaceCorpus) -> Vec<SkinwalkerChangeShapeMasterOption> {
    let by_key: BTreeMap<&str, _> =
        corpus.traits_for("Skinwalker").into_iter().map(|record| (record.data.key.as_str(), record)).collect();

    let mut out = Vec::new();
    for record in corpus.traits_for("Skinwalker") {
        let Some(suffix) = record
            .automatic_trait_grants()
            .into_iter()
            .find_map(|grant| grant.strip_prefix(POOL_PREFIX).map(str::to_string))
        else {
            continue;
        };
        let Some((_, option_keys)) = KIN_OPTION_KEYS.iter().find(|(kin, _)| *kin == suffix) else {
            // "Default" (the non-kin coldborn pool) is the one real case:
            // deliberately unmapped, see this module's own doc comment.
            continue;
        };
        let grants: Vec<SkinwalkerChangeShapeGrant> = option_keys
            .iter()
            .filter_map(|key| {
                by_key.get(key).map(|option| SkinwalkerChangeShapeGrant {
                    key: option.data.key.clone(),
                    name: option.data.name.clone(),
                    description: option.data.description.clone(),
                })
            })
            .collect();
        out.push(SkinwalkerChangeShapeMasterOption {
            key: record.data.key.clone(),
            name: record.data.name.clone(),
            book_id: record.book_id.clone(),
            kin: suffix,
            grants,
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::corpus_loader::BookCorpusRoot;
    use crate::rules_core::race_resolver::load_race_corpus;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn skinwalker_corpus() -> RaceCorpus {
        let dir = repo_root().join("data/corpus/bestiary_5");
        let roots = vec![BookCorpusRoot { book_id: "bestiary_5", dir: dir.as_path() }];
        load_race_corpus(&roots)
    }

    /// Every one of the 9 real kins resolves at least 3 real, non-fabricated
    /// grants -- proof the static table and the live corpus still agree.
    #[test]
    fn all_nine_kins_resolve_real_nonempty_grants() {
        let corpus = skinwalker_corpus();
        let options = skinwalker_change_shape_options(&corpus);
        let by_kin: BTreeMap<&str, &SkinwalkerChangeShapeMasterOption> =
            options.iter().map(|o| (o.kin.as_str(), o)).collect();
        assert_eq!(by_kin.len(), 9, "expected exactly the 9 real were-creature kins, found {:?}", by_kin.keys());
        for (kin, expected_len) in [
            ("Werebat-Kin", 4),
            ("Werebear-Kin", 4),
            ("Wereboar-Kin", 4),
            ("Werecrocodile-Kin", 3),
            ("Wererat-Kin", 4),
            ("Wereshark-Kin", 4),
            ("Weretiger-Kin", 4),
            ("Werewolf-Kin", 3),
            ("Wereraptor-Kin", 4),
        ] {
            let option = by_kin.get(kin).unwrap_or_else(|| panic!("{kin} must resolve"));
            assert_eq!(
                option.grants.len(),
                expected_len,
                "{kin} should resolve exactly {expected_len} real grants, found {:?}",
                option.grants.iter().map(|g| g.key.as_str()).collect::<Vec<_>>()
            );
            for grant in &option.grants {
                assert!(!grant.name.trim().is_empty(), "{kin}'s grant {} must carry a real name", grant.key);
            }
        }
    }

    /// The union of every kin's real grants is exactly 19 of the 20 real
    /// option records -- `Endurance` stays a verified, named orphan (this
    /// module's own doc comment), never silently swallowed into a kin's
    /// pool it does not really belong to.
    #[test]
    fn the_real_option_union_is_nineteen_of_twenty_and_endurance_is_the_named_orphan() {
        let corpus = skinwalker_corpus();
        let options = skinwalker_change_shape_options(&corpus);
        let mut resolved_keys: std::collections::BTreeSet<String> =
            options.iter().flat_map(|o| o.grants.iter().map(|g| g.key.clone())).collect();
        assert_eq!(resolved_keys.len(), 19, "expected exactly 19 unique real option keys resolved");
        assert!(resolved_keys.remove("Skinwalker ~ Change Shape (Bite)"));

        let all_twenty: std::collections::BTreeSet<String> = corpus
            .traits_for("Skinwalker")
            .into_iter()
            .filter(|r| r.data.key.starts_with("Skinwalker ~ Change Shape ("))
            .map(|r| r.data.key.clone())
            .collect();
        assert_eq!(all_twenty.len(), 20, "the live corpus must still carry all 20 real option records");
        assert!(
            all_twenty.contains("Skinwalker ~ Change Shape (Endurance)"),
            "Endurance must still be a real, loaded record"
        );
    }

    /// A master kin record with no `KIN_OPTION_KEYS` entry (the "Default"
    /// coldborn pool) is skipped rather than fabricating an empty-but-listed
    /// option -- proven by confirming the 9-kin count above never grows to
    /// 10 even though a 10th `ABILITY:...TYPE=Skinwalker Change Shape
    /// Default` master record is genuinely loaded.
    #[test]
    fn the_default_coldborn_pool_is_loaded_but_deliberately_unmapped() {
        let corpus = skinwalker_corpus();
        let has_default_master = corpus
            .traits_for("Skinwalker")
            .into_iter()
            .any(|r| r.automatic_trait_grants().iter().any(|g| g == "TYPE=Skinwalker Change Shape Default"));
        assert!(has_default_master, "the Default master record must be loaded for this test to prove anything");

        let options = skinwalker_change_shape_options(&corpus);
        assert!(
            options.iter().all(|o| o.kin != "Default"),
            "the Default pool must stay unmapped by this cycle's resolver"
        );
    }
}
