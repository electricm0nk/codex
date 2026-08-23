//! SD-27 — Aasimar's nine alternate racial traits stop being a dead affordance.
//!
//! # The defect
//!
//! All nine of Aasimar's ARG alternate racial traits were offered as checkable
//! rows in the picker and could never succeed. `create_character` refused every
//! one of them with a `race.alternate_trait.inert_flag` diagnostic, because the
//! flag each sets (`Aasimar_ReplaceSkilled`, `Aasimar_ReplaceCelestialResistance`,
//! …) suppressed nothing and granted nothing in the loaded corpus. They were the
//! app's only *unconditionally* dead affordance: not a build-dependent block, a
//! row that could never work for anybody.
//!
//! # Why the gate was missing, and where it actually lives
//!
//! `decisions.md §26` describes the swap protocol as a negated fact-check
//! carried on the standard trait's own row:
//!
//! ```text
//! Greed  KEY:Dwarf ~ Greed  !PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True
//! ```
//!
//! That is true but **not the whole protocol, and not its authoritative half**.
//! PCGen also states every gate a second time, per race, in
//! `core_essentials/races/<race>/<race>_abilities_globalvar.lst`, as a `.MOD`
//! that grants each standard trait only while its flag is unset:
//!
//! ```text
//! CATEGORY=Special Ability|Aasimar ~ Default.MOD
//!     ABILITY:Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Skilled|PREVAREQ:Aasimar_ReplaceSkilled,0
//! ```
//!
//! `PREVAREQ:<Flag>,0` — "this trait applies while `<Flag>` is 0" — is exactly
//! `!PREFACT:1,ABILITIES,<Flag>=True` said the other way round. Aasimar is the
//! one in-scope race whose `_abilities_race.lst` carries **zero** `!PREFACT`
//! tokens, so it was the one race for which the `!PREFACT`-only reader saw no
//! gates at all.
//!
//! # Why reading the second source is transcription, not invention
//!
//! Because the two sources can be checked against each other, and are —
//! [`the_two_gate_sources_agree_wherever_both_speak`] does it over every
//! in-scope race. The globalvar file is a superset that never contradicts:
//! across the 175 standard-trait rows it agrees on every row where the
//! `!PREFACT` reader found anything, and speaks for 9 rows where the `!PREFACT`
//! reader found nothing. A contradiction would fail `ingest_races` outright.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus, TraitRole};

/// The books the shipped app really loads, read out of its own
/// `RACE_CORPUS_BOOKS` declaration rather than duplicated here.
///
/// **This was a hardcoded three-book list**, which made every "over every
/// in-scope race" claim below quietly narrower than it read. See SD-29
/// `decisions.md §44.2` for what that same hardcoding cost one file over: four
/// alternates reached a player's picker and were refused by the engine, and
/// the pin whose job was to notice could not see them.
fn app_loaded_books() -> Vec<String> {
    let src = std::fs::read_to_string("apps/desktop/src-tauri/src/race_catalog.rs")
        .expect("the desktop race catalog source is readable from the repo root");
    let decl = src
        .split("pub(crate) const RACE_CORPUS_BOOKS: &[&str] =")
        .nth(1)
        .expect("RACE_CORPUS_BOOKS is declared in race_catalog.rs");
    let list = decl.split(';').next().expect("the declaration terminates");
    list.split('"').skip(1).step_by(2).map(str::to_owned).collect()
}

fn corpus() -> RaceCorpus {
    let dirs: Vec<(String, PathBuf)> = app_loaded_books()
        .into_iter()
        .map(|book| {
            let dir = PathBuf::from("data/corpus").join(&book);
            (book, dir)
        })
        .collect();
    let roots: Vec<BookCorpusRoot<'_>> = dirs
        .iter()
        .map(|(book, dir)| BookCorpusRoot { book_id: book.as_str(), dir: dir.as_path() })
        .collect();
    let corpus = load_race_corpus(&roots);
    assert!(corpus.diagnostics().is_empty(), "clean corpus load: {:?}", corpus.diagnostics());
    corpus
}

/// Aasimar's nine standard trait rows now carry the gate their alternates fire.
///
/// Read off the loaded corpus, so this proves the on-disk records changed, not
/// that a function returns a constant.
#[test]
fn every_aasimar_standard_trait_now_declares_the_flag_that_suppresses_it() {
    let corpus = corpus();
    let gates: BTreeMap<&str, &str> = corpus
        .traits_for("Aasimar")
        .iter()
        .filter(|record| record.role == TraitRole::Default)
        .map(|record| {
            (
                record.data.key.as_str(),
                record
                    .data
                    .suppressed_by_flag
                    .as_deref()
                    .unwrap_or_else(|| panic!("{} declares no suppression gate", record.data.key)),
            )
        })
        .collect();

    assert_eq!(
        gates,
        BTreeMap::from([
            ("Aasimar ~ Ability Scores", "Aasimar_ReplaceAbilityScores"),
            ("Aasimar ~ Celestial Resistance", "Aasimar_ReplaceCelestialResistance"),
            ("Aasimar ~ Languages", "Aasimar_ReplaceLanguages"),
            ("Aasimar ~ Size", "Aasimar_ReplaceSize"),
            ("Aasimar ~ Skilled", "Aasimar_ReplaceSkilled"),
            ("Aasimar ~ Speed", "Aasimar_ReplaceSpeed"),
            ("Aasimar ~ Spell-Like Ability", "Aasimar_ReplaceSpellLikeAbility"),
            ("Aasimar ~ Type", "Aasimar_ReplaceType"),
            ("Aasimar ~ Vision", "Aasimar_ReplaceVision"),
        ]),
        "the nine gates, exactly as aasimar_abilities_globalvar.lst declares them"
    );
}

/// The affordance itself: every one of the nine really swaps something.
///
/// Asserted as a before/after pair per alternate — the standard trait is
/// present without the selection and gone with it — because "the flag is no
/// longer inert" is a weaker claim than "the player's trait list actually
/// changed", and only the second one is what a player sees.
#[test]
fn every_aasimar_alternate_really_replaces_a_standard_trait() {
    let corpus = corpus();

    let before = corpus.resolve("Aasimar", &[]).expect("Aasimar resolves");
    assert!(before.suppressions.is_empty());
    assert!(before.inert_flags.is_empty());
    let baseline: BTreeSet<&str> = before.traits.iter().map(|t| t.key.as_str()).collect();
    assert_eq!(baseline.len(), 9, "Aasimar's nine racial defaults");

    // Derived from the corpus, never hand-listed: each alternate against the
    // standard rows its own flags name.
    let alternates: Vec<String> =
        corpus.alternate_traits("Aasimar").iter().map(|record| record.data.key.clone()).collect();
    assert_eq!(
        alternates.len(),
        17,
        "ARG's nine Aasimar alternates + Inner Sea Races' two (`Aasimar ~ Crusading Magic`, \
         `Aasimar ~ Lost Promise`) + Core Essentials' six heritages (SD-29 decisions.md §49, \
         round 4). Aasimar's nine racial DEFAULTS above did not move and must not: ISR, HA and \
         CE contribute alternates only, no chassis"
    );

    let mut checked = 0usize;
    for key in &alternates {
        let after = corpus.resolve("Aasimar", &[key.as_str()]).expect("Aasimar resolves");
        assert!(after.unmatched_selections.is_empty(), "{key} is a key the resolver accepts");
        assert!(
            after.inert_flags.is_empty(),
            "{key} still fires a flag that suppresses and grants nothing: {:?}",
            after.inert_flags
        );
        assert!(!after.suppressions.is_empty(), "{key} must actually suppress something");

        let applied: BTreeSet<&str> = after.traits.iter().map(|t| t.key.as_str()).collect();
        assert!(applied.contains(key.as_str()), "{key} itself applies");
        for suppression in &after.suppressions {
            assert_eq!(suppression.set_by_trait_key, *key);
            assert!(
                baseline.contains(suppression.suppressed_trait_key.as_str()),
                "{key} suppressed {} which was not a default in the first place",
                suppression.suppressed_trait_key
            );
            assert!(
                !applied.contains(suppression.suppressed_trait_key.as_str()),
                "{key} claims to replace {} and it is still applied",
                suppression.suppressed_trait_key
            );
        }
        checked += 1;
    }
    // 9 ARG + Inner Sea Races' 2 + Core Essentials' 6 heritages. The name
    // above no longer carries the count, for `decisions.md §44.5`'s reason: a
    // new book adds Aasimar alternates without the property changing -- which
    // round 4 is the third demonstration of, and this time the six new
    // alternates each suppress THREE standard rows rather than one or two,
    // exercising the loop above harder than any previous book did.
    assert_eq!(checked, 17);

    // The worked case, spelled out: Celestial Crusader sets two flags and
    // removes both of the traits they name, and nothing else.
    let crusader = corpus.resolve("Aasimar", &["Aasimar ~ Celestial Crusader"]).expect("resolves");
    let suppressed: Vec<&str> =
        crusader.suppressions.iter().map(|s| s.suppressed_trait_key.as_str()).collect();
    assert_eq!(suppressed, vec!["Aasimar ~ Celestial Resistance", "Aasimar ~ Skilled"]);
    let applied: BTreeSet<&str> = crusader.traits.iter().map(|t| t.key.as_str()).collect();
    let lost: Vec<&&str> = baseline.difference(&applied).collect();
    assert_eq!(lost, vec![&"Aasimar ~ Celestial Resistance", &"Aasimar ~ Skilled"]);

    // `Aasimar ~ Scion of Humanity` now behaves EXACTLY as the already-shipped
    // `Orc ~ Feral` does, and this asserts the two against each other rather
    // than describing either.
    //
    // Both are the same corpus shape: an alternate that suppresses its race's
    // `Languages` row and names its replacement with a direct
    // `ABILITY:<Race> Racial Trait|AUTOMATIC|<key>` token on its own row —
    // a *third* grant shape, distinct from the positive `PREFACT` gate that
    // brings in `Saltbeard ~ Dwarf ~ Greed`.
    //
    // **When this file was written, `race_resolver` read the positive `PREFACT`
    // shape and not this one, so both replacement rows classified
    // `TraitRole::Unclassified` and never arrived. That gap is now closed** —
    // `race_resolver::link_automatic_grants` resolves the grant and
    // `tests/sd27_ability_automatic_granted_race_traits.rs` pins both rows in
    // both directions. The loop below is unchanged and still passes, because
    // what it asserts is the *symmetry* of the two races, not the state of the
    // replacement row.
    for (race, alternate, standard, replacement) in [
        ("Aasimar", "Aasimar ~ Scion of Humanity", "Aasimar ~ Languages", "Scion of Humanity ~ Languages"),
        ("Orc", "Orc ~ Feral", "Orc ~ Languages", "Feral ~ Languages"),
    ] {
        let resolved = corpus.resolve(race, &[alternate]).expect("resolves");
        let keys: BTreeSet<&str> = resolved.traits.iter().map(|t| t.key.as_str()).collect();
        assert!(resolved.inert_flags.is_empty(), "{alternate} fires no inert flag: {:?}", resolved.inert_flags);
        assert!(keys.contains(alternate), "{alternate} itself applies");
        assert!(!keys.contains(standard), "{alternate} replaces {standard}");
        // Was `assert!(!keys.contains(replacement), ...)` with a note to flip
        // it once the direct-`ABILITY` grant edge was implemented. It is
        // implemented; this is the flip.
        assert!(
            keys.contains(replacement),
            "{alternate} suppresses {standard}, so {replacement} must arrive in its place"
        );
    }
}

/// No orphan replace-flag remains for any race whose gate the corpus can state.
///
/// The full-corpus version of the finding this cycle closed: a flag an
/// alternate sets that nothing declares. One survivor is expected and named —
/// `Duergar_ReplaceSLAInvisibility`, whose gate the corpus *does* declare but
/// which `RaceTraitCacheData::suppressed_by_flag` (single-valued) cannot hold
/// because its row's gate names three flags. That is a schema limit, not a
/// missing file, it is reported by `race_trait_picker::multi_flag_gate_findings`,
/// and the alternate that sets it is not dead: the flag grants
/// `Duergar ~ Spell-Like Ability ~ Enlarge Person`, so the selection succeeds.
#[test]
fn the_only_remaining_orphan_flag_is_the_one_a_single_valued_field_cannot_hold() {
    let corpus = corpus();
    let all: Vec<_> = corpus.race_keys().into_iter().flat_map(|race| corpus.traits_for(race)).collect();
    let claimed: BTreeSet<&str> = all
        .iter()
        .filter_map(|record| record.data.suppressed_by_flag.as_deref().or(record.requires_flag.as_deref()))
        .collect();

    let mut orphans: BTreeSet<&str> = BTreeSet::new();
    for record in &all {
        for flag in &record.data.sets_replace_flags {
            if !claimed.contains(flag.as_str()) {
                orphans.insert(flag.as_str());
            }
        }
    }
    // Two flags, one cause — the SAME truncated multi-flag gate seen from its
    // two ends. `Duergar ~ Spell-Like Ability ~ Invisibility`'s row names three
    // flags and the single-valued `suppressed_by_flag` keeps only the first, so
    // Monster Codex's `Duergar ~ Ironskinned` fires
    // `Duergar_ReplaceSLAEnlargePerson` into the same blind spot that
    // `Duergar ~ Blood Enmity` already fired `Duergar_ReplaceSLAInvisibility`
    // into. Neither is a dead affordance; both are proved to grant below.
    // SD-29 `decisions.md §44.2`.
    assert_eq!(
        orphans.into_iter().collect::<Vec<_>>(),
        vec!["Duergar_ReplaceSLAEnlargePerson", "Duergar_ReplaceSLAInvisibility"],
        "the Aasimar five are closed; a new orphan is a new defect"
    );

    // ...and neither is a dead affordance, because both grant.
    let ironskinned = corpus.resolve("Duergar", &["Duergar ~ Ironskinned"]).expect("Duergar resolves");
    assert!(ironskinned.inert_flags.is_empty(), "{:?}", ironskinned.inert_flags);
    let blood_enmity = corpus.resolve("Duergar", &["Duergar ~ Blood Enmity"]).expect("Duergar resolves");
    assert!(blood_enmity.inert_flags.is_empty(), "{:?}", blood_enmity.inert_flags);
    assert!(blood_enmity
        .traits
        .iter()
        .any(|t| t.key == "Duergar ~ Spell-Like Ability ~ Enlarge Person"));
}

/// **Nothing anywhere is a dead affordance any more.**
///
/// Every alternate the picker offers, for every race, resolved against a clean
/// selection: none may fire an inert flag, which is the single condition
/// `character_hub::resolve_alternate_trait_choices` refuses a save on. This is
/// the assertion that would have caught the original defect, and it is stated
/// over all 153 rather than over the nine that happened to be broken.
#[test]
fn no_offered_alternate_racial_trait_can_ever_be_refused_for_an_inert_flag() {
    let corpus = corpus();
    let mut checked = 0usize;
    for race_key in corpus.race_keys() {
        for record in corpus.alternate_traits(race_key) {
            let key = record.data.key.clone();
            let resolved = corpus.resolve(race_key, &[key.as_str()]).expect("race resolves");
            assert!(
                resolved.inert_flags.is_empty(),
                "{key} is offered in the picker and `create_character` would refuse it: {:?}",
                resolved.inert_flags
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked, 370,
        "ARG's 153 + Monster Codex's 8 (4 original + SD-32 card-11 T2b's 4 Ratfolk \
         alternates, 2026-08-23) + APG's 1 + Inner Sea Races' 76 (67 pre-existing + 9 from \
         a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22) + Horror \
         Adventures' 41 \
         + Core Essentials' 16 heritages, selectable alternates (SD-29 decisions.md §44, §45, \
         §47, §49) + SD-31 Epic 1-F2's 48 Bestiary 2 batch (ARG's 42 + Inner Sea Races' 6, \
         2026-08-15) + SD-31-E6-F4-003's 19 (2026-08-16, ARG's own 6-race chassis batch) + \
         SD31-E6-F4-006's 8 (2026-08-17, ARG's own follow-on 4-race chassis batch). The \
         158 this pin held until 2026-08-12 was round 2's miss, not a smaller \
         corpus"
    );
}

/// The two gate sources agree wherever both speak, over every in-scope race.
///
/// This is the evidence that reading `_abilities_globalvar.lst` is faithful
/// transcription. It re-derives both readings straight off the loaded corpus
/// records rather than re-parsing the upstream LST: `suppressed_by_flag` is
/// what the ingest tool concluded, and the verbatim `!PREFACT` token preserved
/// in `raw_tokens` is what the row itself said. Where the row said something,
/// the conclusion must match it — so a globalvar read can never overwrite a
/// gate the trait row itself declares.
#[test]
fn the_two_gate_sources_agree_wherever_both_speak() {
    let corpus = corpus();
    let mut from_row = 0usize;
    let mut from_globalvar = 0usize;

    for race_key in corpus.race_keys() {
        for record in corpus.traits_for(race_key) {
            if record.role == TraitRole::Alternate {
                continue;
            }
            let Some(gate) = record.data.suppressed_by_flag.as_deref() else { continue };
            let row_token = record.data.raw_tokens.iter().find(|token| token.key == "!PREFACT");
            match row_token {
                Some(token) => {
                    assert!(
                        token.value.contains(gate),
                        "{}: concluded gate {gate} is not in its own row's !PREFACT ({})",
                        record.data.key,
                        token.value
                    );
                    from_row += 1;
                }
                None => from_globalvar += 1,
            }
        }
    }

    assert_eq!(
        from_row,
        366,
        "rows whose own !PREFACT declares the gate (166 -> 223: SD-31 Epic 1-F2's 57 new \
         standard rows, 2026-08-15, all of which declare their own !PREFACT; 223 -> 232: the \
         Skinwalker follow-on batch's 9 standard rows, all of which also declare their own \
         !PREFACT; 232 -> 290: SD-31-E6-F4-002's Advanced Race Guide batch of 58 standard \
         rows, 2026-08-16, same shape, all self-gated; 290 -> 328: SD31-E6-F4-004's \
         Advanced Race Guide follow-on batch of 38 standard rows, 2026-08-17, same shape, \
         all self-gated; 328 -> 346: SD31-E6-F4-007's Advanced Race Guide follow-on batch of \
         18 standard rows (Changeling, Samsaran), 2026-08-17, closing arg_races.lst's full \
         37-row playable-race roster, same shape, all self-gated -- including Samsaran's \
         `Shards of the Past`, whose own row carries `!PREFACT:1,ABILITIES,Samsaran_\
         ReplaceShardsOfThePast=True` even though its SECOND gate statement in the globalvar \
         file is a `BONUS:ABILITYPOOL` grant rather than an `ABILITY:` one -- see \
         `ingest_races.rs`'s `globalvar_prevareq_flags`; 346 -> 354: SD-31 wave-24's \
         Rougarou (Bestiary 6, 2026-08-20), 8 standard rows, same shape, all self-gated; \
         354 -> 366: SD-32 card-11 T2b lane's Dhampir (Bestiary 2, 2026-08-23), 12 standard \
         rows, same shape, all self-gated)"
    );
    assert_eq!(from_globalvar, 9, "Aasimar's nine, whose only declaration is the globalvar file");
}
