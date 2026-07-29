//! Cross-book spell identity: a spell `key` must name exactly one real
//! corpus record across every ingested book.
//!
//! Why this matters, derived from the raw corpus (not from any doc):
//! PCGen spell rows are identified by their `KEY:` token when they carry
//! one, and by their display name when they do not. In
//! `advanced_players_guide/apg_spells.lst` and
//! `advanced_class_guide/acg_spells.lst` exactly 18 rows carry a `KEY:`
//! token, and in every one of those the KEY is archetype-qualified and
//! differs from the display name:
//!
//! ```text
//! apg_spells.lst:649  Summon Monster I       KEY:Summoner Summon Monster I
//! acg_spells.lst:785  Summon Nature's Ally I KEY:Naturalist Summon Nature's Ally I
//! ```
//!
//! These are genuinely different records from the Core Rulebook spells
//! whose display names they share — the Summoner/Naturalist rows carry
//! their own `DURATION:` formula keyed to
//! `ConjurationSummonersCharmBonus` / `ConjurationNaturalistsCharmBonus`,
//! an archetype-specific effect the CRB record does not have.
//!
//! Both ingests stored the *display name* in `key` and discarded the
//! `KEY:` token, so all 18 collided with the CRB record of the same name.
//! Since `spell_resolver::spell_id_resolve` resolves a selection by name
//! and `SPELL_LIST` lookups match `entry.key == spell_id`, two different
//! records sharing one identity is not cosmetic: it makes the resolution
//! ambiguous, and any catalog that serves more than one book at once
//! shows the same name twice with different text.

use std::collections::HashMap;

use codex::rules_core::rules_tables::{acg, apg, crb};

/// Every `(key, book)` pair across the three ingested spell tables.
fn all_keys() -> Vec<(&'static str, &'static str)> {
    let mut keys: Vec<(&'static str, &'static str)> = Vec::new();
    keys.extend(crb::spell_list::SPELL_LIST.iter().map(|e| (e.key, "CRB")));
    keys.extend(apg::spell_list::SPELL_LIST.iter().map(|e| (e.key, "APG")));
    keys.extend(acg::spell_list::SPELL_LIST.iter().map(|e| (e.key, "ACG")));
    keys
}

#[test]
fn no_spell_key_is_shared_by_two_ingested_books() {
    let mut by_key: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    for (key, book) in all_keys() {
        by_key.entry(key).or_default().push(book);
    }

    let mut collisions: Vec<(&str, Vec<&str>)> = by_key
        .into_iter()
        .filter(|(_, books)| books.len() > 1)
        .collect();
    collisions.sort();

    assert!(
        collisions.is_empty(),
        "these spell keys name more than one record across books, so a \
         selection carrying one of them cannot be resolved unambiguously: {collisions:?}"
    );
}

#[test]
fn the_apg_summoner_summon_monster_records_keep_their_corpus_key_token() {
    // Verbatim from apg_spells.lst lines 649-657's own `KEY:` tokens.
    for numeral in ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"] {
        let expected = format!("Summoner Summon Monster {numeral}");
        assert!(
            apg::spell_list::SPELL_LIST
                .iter()
                .any(|entry| entry.key == expected),
            "APG spell list is missing the corpus KEY {expected:?}"
        );
    }
}

#[test]
fn the_acg_naturalist_summon_natures_ally_records_keep_their_corpus_key_token() {
    // Verbatim from acg_spells.lst lines 785-793's own `KEY:` tokens.
    for numeral in ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"] {
        let expected = format!("Naturalist Summon Nature's Ally {numeral}");
        assert!(
            acg::spell_list::SPELL_LIST
                .iter()
                .any(|entry| entry.key == expected),
            "ACG spell list is missing the corpus KEY {expected:?}"
        );
    }
}

#[test]
fn the_three_ingested_books_carry_their_full_corpus_record_counts() {
    // Counts re-derived from the raw .lst files, excluding `#`-disabled
    // rows, `.MOD` modifier rows and the `SOURCELONG:` header line.
    assert_eq!(crb::spell_list::SPELL_LIST.len(), 652, "CRB cr_spells.lst");
    assert_eq!(apg::spell_list::SPELL_LIST.len(), 297, "APG apg_spells.lst");
    assert_eq!(acg::spell_list::SPELL_LIST.len(), 144, "ACG acg_spells.lst");
}
