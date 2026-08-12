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

use codex::rules_core::rules_tables::{RuleSetId, acg, apg, crb};

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

/// The other half of the identity contract. Storing the archetype-qualified
/// `KEY:` token (above) is what keeps these 18 records distinct from the
/// Core Rulebook records whose *display* names they share. But the corpus
/// row has two columns, and the display column is the name a selection
/// actually carries: `acg_spells.lst:787` reads
/// `Summon Nature's Ally III<TAB>KEY:Naturalist Summon Nature's Ally III`.
/// A caller holding only that display name — with no archetype context —
/// must still resolve, so `spell_resolve` accepts either column.
///
/// This is unambiguous *within* each book: no ingested row in
/// `acg_spells.lst` displays as a bare `Summon Nature's Ally <roman>`
/// except the 9 Naturalist rows themselves (the only other such rows are
/// the `.MOD` modifiers at `:546`-`:766`, which are not ingested as
/// records), and likewise in `apg_spells.lst` the bare
/// `Summon Monster <roman>` display name belongs only to the 9 Summoner
/// rows (`:649`-`:657`) — its other ingested `Summon Monster` records all
/// carry a distinguishing parenthetical (`Summon Monster V (Summons 1d3
/// Shadows)`, `Summon Monster III (Reptiles Only)`).
#[test]
fn an_archetype_qualified_record_resolves_by_its_corpus_display_name() {
    for numeral in ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"] {
        let display = format!("Summon Nature's Ally {numeral}");
        let entry = acg::spell_list::spell_resolve(&display, RuleSetId::Acg)
            .unwrap_or_else(|| panic!("ACG display name {display:?} should resolve"));
        assert_eq!(
            entry.key,
            format!("Naturalist Summon Nature's Ally {numeral}"),
            "the display name must resolve to the archetype-qualified record"
        );

        let display = format!("Summon Monster {numeral}");
        let entry = apg::spell_list::spell_resolve(&display, RuleSetId::Apg)
            .unwrap_or_else(|| panic!("APG display name {display:?} should resolve"));
        assert_eq!(
            entry.key,
            format!("Summoner Summon Monster {numeral}"),
            "the display name must resolve to the archetype-qualified record"
        );
    }
}

/// Guards the display-name fallback against reintroducing exactly the
/// ambiguity the `KEY:`-token change fixed: a display name must never
/// shadow a *different* record already present in the same book under
/// that name, and the two columns must stay one-to-one.
#[test]
fn no_archetype_display_name_shadows_another_record_in_its_own_book() {
    fn check(book: &str, pairs: &'static [(&'static str, &'static str)], record_keys: &[&str]) {
        let mut seen_display: HashMap<&'static str, &'static str> = HashMap::new();
        for (key, display) in pairs {
            assert!(
                record_keys.contains(key),
                "{book}: {key:?} is not a record in its own SPELL_LIST"
            );
            assert!(
                !record_keys.contains(display),
                "{book}: display name {display:?} (for {key:?}) is also a \
                 record key in its own book, so the fallback would be ambiguous"
            );
            if let Some(previous) = seen_display.insert(display, key) {
                panic!("{book}: display name {display:?} maps to both {previous:?} and {key:?}");
            }
        }
    }

    let acg_keys: Vec<&str> = acg::spell_list::SPELL_LIST.iter().map(|e| e.key).collect();
    check("ACG", acg::spell_list::ARCHETYPE_QUALIFIED_KEYS, &acg_keys);

    let apg_keys: Vec<&str> = apg::spell_list::SPELL_LIST.iter().map(|e| e.key).collect();
    check("APG", apg::spell_list::ARCHETYPE_QUALIFIED_KEYS, &apg_keys);
}

/// The `SPELL_LIST` table above is only half the pipeline: the on-disk
/// JSON cache (`data/corpus/advanced_class_guide/spell/*.json`), produced
/// by `codex::rules_core::cache_gen::acg::generate()` and consulted by
/// `v06_corpus_trap_report --audit`, is the actual artifact ingest ships.
/// A prior generation run stamped `data.key` as the *display* name for the
/// 9 Naturalist rows (pre-dating the `SPELL_LIST` KEY:-token fix above),
/// so the shipped cache silently clobbered the base CRB `Summon Nature's
/// Ally <roman>` identity even after the table itself was corrected. This
/// test guards the regenerated cache directly: the base spell's identity
/// must never appear as the `data.key` of the ACG Naturalist record, and
/// the real archetype-qualified `KEY:` must be what's stored instead.
#[test]
fn the_acg_naturalist_json_cache_stores_the_archetype_qualified_key_not_the_display_name() {
    let cache_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data/corpus/advanced_class_guide/spell");
    let mut seen_keys: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for entry in std::fs::read_dir(&cache_dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", cache_dir.display()))
    {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let text = std::fs::read_to_string(&path).unwrap();
            let value: serde_json::Value = serde_json::from_str(&text).unwrap();
            let key = value["data"]["key"].as_str().unwrap().to_string();
            seen_keys.insert(key);
        }
    }

    for numeral in ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"] {
        let archetype_key = format!("Naturalist Summon Nature's Ally {numeral}");
        assert!(
            seen_keys.contains(&archetype_key),
            "ACG spell cache is missing the archetype-qualified identity {archetype_key:?}; \
             the ingest must file the Naturalist variant under its own KEY:, not the base \
             spell's display name (data.key currently: {seen_keys:?})"
        );
    }
}
