//! v0.6 — pins the Bestiary 1 natural attacks whose damage dice are not
//! transcribable from the monster's own `b1_races.lst` row.
//!
//! A full verification pass found Bestiary 1 error-free across all 41
//! monsters but with 12 carrying missing or partial natural attacks.
//! Those 12 rows reference their attacks through
//! `ABILITY:Internal|AUTOMATIC|<Name>` instead of an inline
//! `NATURALATTACKS:` token. The referenced rows **do** exist (in
//! `core_essentials/ce_abilities_race.lst`, not under `bestiary/`), but
//! carry no dice — they are mechanical markers whose dice PCGen supplies
//! at runtime from size tables. See
//! `rules_tables::beastiary1::natural_attack_provenance`'s module doc
//! comment for the full finding.
//!
//! These tests exist so the grounded values cannot be silently reverted
//! to an absent-token empty list, and so no attack can be added to the
//! shipped tables without a cited source.

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::beastiary1::natural_attack_provenance::{
    AttackSource, GROUNDED_NATURAL_ATTACKS, provenance_for,
};
use codex::rules_core::rules_tables::beastiary1::{MonsterId, monster_key_resolve, monster_resolve};

/// `SD-26 decisions.md §11.5`: "Allowed domains only: `d20pfsrd.com`,
/// `legacy.aonprd.com` / `aonprd.com`. No other source."
const ALLOWED_DOMAINS: &[&str] = &["aonprd.com", "d20pfsrd.com"];

/// The exact, corroborated attack lists for the 12 monsters this pass
/// grounded — the real assertion, spelled out rather than derived, so a
/// regression in either the tables or the provenance module is caught.
fn expected_grounded_lists() -> Vec<(&'static str, Vec<(&'static str, &'static str)>)> {
    vec![
        ("beastiary1:monster:ankheg", vec![("Bite", "2d6")]),
        ("beastiary1:monster:assassin_vine", vec![("Slam", "1d8")]),
        ("beastiary1:monster:boar", vec![("Gore", "1d8")]),
        // Partial, not empty: the real `NATURALATTACKS:Filament,...,*1,0`
        // token stays first and untouched; only the Claw was recovered.
        ("beastiary1:monster:cave_fisher", vec![("Filament", "0"), ("Claw", "1d4")]),
        ("beastiary1:monster:centaur", vec![("Hoof", "1d6")]),
        ("beastiary1:monster:choker", vec![("Tentacle", "1d4")]),
        ("beastiary1:monster:cockatrice", vec![("Bite", "1d4")]),
        ("beastiary1:monster:crocodile", vec![("Bite", "1d8"), ("Tail Slap", "1d12")]),
        ("beastiary1:monster:vargouille", vec![("Bite", "1d4")]),
        ("beastiary1:monster:wolf", vec![("Bite", "1d6")]),
        ("beastiary1:monster:wolverine", vec![("Bite", "1d4"), ("Claw", "1d6")]),
        ("beastiary1:monster:worg", vec![("Bite", "1d6")]),
    ]
}

#[test]
fn the_twelve_grounded_monsters_ship_exactly_their_corroborated_attack_lists() {
    for (key, expected) in expected_grounded_lists() {
        let block = monster_key_resolve(key, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{key} must resolve for RuleSetId::Bestiary1"));
        let actual: Vec<(&str, &str)> = block
            .natural_attacks
            .iter()
            .map(|a| (a.name.as_str(), a.damage_dice.as_str()))
            .collect();
        assert_eq!(
            actual, expected,
            "{key}: shipped natural attacks must match the corroborated list. If this failed \
             because the list is empty, do NOT 'fix' it by deleting the expectation -- read \
             rules_tables::beastiary1::natural_attack_provenance first; the absent corpus token \
             is not evidence the monster has no attack."
        );
    }
}

#[test]
fn every_grounded_attack_has_a_provenance_row_and_every_provenance_row_is_shipped() {
    // Direction 1: every provenance row is really shipped, with the
    // exact dice it claims.
    for g in GROUNDED_NATURAL_ATTACKS {
        let block = monster_key_resolve(g.monster_key, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{}: provenance row names an unresolvable monster", g.monster_key));
        let shipped = block
            .natural_attacks
            .iter()
            .find(|a| a.name == g.attack_name)
            .unwrap_or_else(|| panic!("{}: provenance claims a {:?} attack that is not shipped", g.monster_key, g.attack_name));
        assert_eq!(
            shipped.damage_dice, g.damage_dice,
            "{}: {:?} dice disagree between the shipped table and its provenance row",
            g.monster_key, g.attack_name
        );
    }

    // Direction 2: no monster silently gains an attack that has neither
    // a real inline `NATURALATTACKS:` token nor a provenance row. The 12
    // grounded monsters are the only ones this table covers, so for each
    // of them every shipped attack must be accounted for -- either by a
    // provenance row, or by being Cave Fisher's real corpus Filament.
    for (key, _) in expected_grounded_lists() {
        let block = monster_key_resolve(key, RuleSetId::Bestiary1).unwrap();
        let documented: Vec<&str> = provenance_for(key).iter().map(|g| g.attack_name).collect();
        for attack in &block.natural_attacks {
            let is_real_corpus_token = key == "beastiary1:monster:cave_fisher" && attack.name == "Filament";
            assert!(
                is_real_corpus_token || documented.contains(&attack.name.as_str()),
                "{key}: shipped attack {:?} has no provenance row. Every attack not transcribed \
                 from the monster's own row must be documented in natural_attack_provenance.",
                attack.name
            );
        }
    }
}

#[test]
fn every_web_grounded_value_cites_at_least_two_independent_allowed_domain_sources() {
    // The 2-of-3 corroboration bar, enforced structurally rather than by
    // trusting a reviewer to notice a single-sourced value.
    for g in GROUNDED_NATURAL_ATTACKS {
        let AttackSource::WebSecondSource { urls, fetched_at, identity_match_basis } = g.source else {
            continue;
        };
        assert!(
            urls.len() >= 2,
            "{} {:?}: only {} source(s) cited -- the grounding bar is at least two independent agreeing sources",
            g.monster_key,
            g.attack_name,
            urls.len()
        );
        let mut hosts: Vec<&str> = Vec::new();
        for url in urls {
            let domain = ALLOWED_DOMAINS
                .iter()
                .find(|d| url.contains(**d))
                .unwrap_or_else(|| panic!("{} {:?}: url {url:?} is outside §11.5's allowed-domain list", g.monster_key, g.attack_name));
            assert!(
                !hosts.contains(domain),
                "{} {:?}: two citations resolve to the same domain {domain:?} -- that is one source, not two",
                g.monster_key,
                g.attack_name
            );
            hosts.push(domain);
        }
        assert!(
            !identity_match_basis.is_empty(),
            "{} {:?}: web-sourced values must record how the creature identity was confirmed (§11.5)",
            g.monster_key,
            g.attack_name
        );
        // ISO-8601 date shape, e.g. 2026-07-29.
        assert!(
            fetched_at.len() == 10 && fetched_at.split('-').count() == 3,
            "{} {:?}: fetched_at {fetched_at:?} is not an ISO-8601 date",
            g.monster_key,
            g.attack_name
        );
    }
}

#[test]
fn crocodile_tail_slap_is_recovered_from_a_real_corpus_token_not_the_web() {
    // The one genuine cross-file corpus recovery in this pass. Crocodile
    // reaches its attacks via `Racial Traits ~ Crocodile`, whose row
    // carries a real inline `NATURALATTACKS:Tail Slap,...,*1,1d12`.
    // Unlike the generic `Bite`/`Claw` markers, this one has dice.
    let row = GROUNDED_NATURAL_ATTACKS
        .iter()
        .find(|g| g.monster_key == "beastiary1:monster:crocodile" && g.attack_name == "Tail Slap")
        .expect("Crocodile Tail Slap provenance row must exist");
    match row.source {
        AttackSource::LstToken { path, line, record_key } => {
            assert_eq!(path, "pathfinder/paizo/roleplaying_game/bestiary/b1_abilities_race.lst");
            assert_eq!(line, 248);
            assert_eq!(record_key, "Crocodile ~ Tail Slap");
        }
        AttackSource::WebSecondSource { .. } => {
            panic!("Crocodile Tail Slap has a real corpus token (1d12) -- it must cite as lst_token, not web_second_source")
        }
    }
    assert_eq!(row.damage_dice, "1d12");
}

#[test]
fn vargouille_kiss_and_shriek_are_not_recorded_as_natural_attacks() {
    // Both appear only under Special Attacks, with no damage dice in any
    // source. Recording them with invented dice would be exactly the
    // fabrication AGENTS.md rules out.
    let block = monster_resolve(MonsterId::Vargouille, RuleSetId::Bestiary1).unwrap();
    for forbidden in ["Kiss", "Shriek"] {
        assert!(
            !block.natural_attacks.iter().any(|a| a.name == forbidden),
            "Vargouille {forbidden:?} is a Special Attack with no damage dice -- it must not be a natural attack"
        );
    }
    assert_eq!(block.natural_attacks.len(), 1, "Vargouille has exactly one real natural attack: the bite");
}

#[test]
fn the_five_weapon_using_monsters_keep_their_correctly_empty_attack_lists() {
    // These five have empty lists *correctly* -- confirmed in print as
    // weapon users. This test exists so a future "close the remaining
    // empty lists" sweep cannot quietly invent natural attacks for them.
    for id in [MonsterId::Bugbear, MonsterId::DarkCreeper, MonsterId::Derro, MonsterId::Dryad, MonsterId::Gnoll] {
        let block = monster_resolve(id, RuleSetId::Bestiary1).unwrap();
        assert!(
            block.natural_attacks.is_empty(),
            "{id:?} is a weapon user with no natural attacks in the published Bestiary 1 -- its empty list is correct, not a gap"
        );
    }
}

#[test]
fn no_monster_outside_the_grounded_twelve_gained_or_lost_attacks() {
    // Guards the blast radius: this pass touched exactly 12 monsters.
    // Every other monster's attack count must be whatever its own real
    // `NATURALATTACKS:` token already produced.
    let grounded: Vec<&str> = expected_grounded_lists().iter().map(|(k, _)| *k).collect();
    let mut untouched_with_attacks = 0;
    for &id in MonsterId::ALL {
        let block = monster_resolve(id, RuleSetId::Bestiary1).unwrap();
        let key = format!(
            "beastiary1:monster:{}",
            block.name.to_lowercase().replace(' ', "_")
        );
        if grounded.contains(&key.as_str()) {
            continue;
        }
        assert!(
            provenance_for(&key).is_empty(),
            "{key}: has provenance rows but is not one of the 12 monsters this pass grounded"
        );
        if !block.natural_attacks.is_empty() {
            untouched_with_attacks += 1;
        }
    }
    // 46 total - 12 grounded - 6 correctly-empty weapon users = 28
    // untouched monsters, all of which carry real inline corpus tokens.
    //
    // SD28-E16 subset 09 added 5 (Lion, Ogre, Pegasus, Rust Monster,
    // Shadow), none of them in the grounded twelve. Ogre carries no
    // NATURALATTACKS: token at all (fights with weapons -- same shape as
    // Dryad, one of the original 5 correctly-empty weapon users), moving
    // that count 5 -> 6. Lion, Pegasus, Rust Monster, and Shadow each
    // carry their own real inline NATURALATTACKS: token(s) (verified in
    // `monster_subset_09.rs`'s own doc comments against the real
    // `b1_races.lst` rows), moving this count 24 -> 28.
    assert_eq!(
        untouched_with_attacks, 28,
        "the 28 monsters with real inline NATURALATTACKS: tokens must be unchanged by this pass"
    );
}

#[test]
fn published_melee_text_actually_contains_the_dice_it_was_read_from() {
    // Cheap transcription check: the recorded dice must literally appear
    // in the quoted published line. Catches a fat-fingered "1d8" against
    // a quote that reads "1d6".
    for g in GROUNDED_NATURAL_ATTACKS {
        if g.damage_dice == "0" {
            continue;
        }
        assert!(
            g.published_melee_text.contains(g.damage_dice),
            "{} {:?}: recorded dice {:?} do not appear in the quoted published text {:?}",
            g.monster_key,
            g.attack_name,
            g.damage_dice,
            g.published_melee_text
        );
    }
}
