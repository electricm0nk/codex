//! SD-27 mandatory guard (`decisions.md §23`, carried forward by `§25.5`):
//! pin Core Rulebook's 7 shipped races against the new corpus-driven path.
//!
//! # What this file is for
//!
//! CRB's 7 races ship today as a hardcoded 7-variant `RaceId` enum plus a
//! 49-row `RACE_TRAITS` table in
//! `src/rules_core/rules_tables/crb/race_tables.rs`. `decisions.md §23`
//! commits to pinning those races' resolved traits *before and after* the
//! corpus-driven swap "so any drift is a caught failure rather than a silent
//! regression." This file is that pin. Both paths coexist for exactly this
//! reason; `race_tables.rs` is not deleted in this cycle.
//!
//! # The two paths are not the same shape, and this file says so explicitly
//!
//! The hardcoded table is a *curated dimension* table — one row per named
//! trait *dimension* per race (Ability Modifiers / Size / Speed / Senses,
//! plus each race's named special traits), 49 rows total. The corpus is
//! PCGen's own complete racial-trait roster, 67 rows for the same 7 races.
//! So a naive row-count equality would be meaningless. This file instead
//! pins the four claims the two paths *both* make, dimension by dimension:
//!
//! 1. size            — `race_size()` vs the chassis `FACT:BaseSize` token
//! 2. base walk speed — the `Speed` row's value vs the resolved walk speed
//! 3. senses          — the `Senses` row vs the resolved vision trait
//! 4. named traits    — every named special trait in the table must exist in
//!    the corpus-resolved default set, with the same numeric magnitude
//!
//! and then pins, as an explicit named divergence, the one dimension where
//! the two paths genuinely disagree (ability modifiers — see the bottom of
//! this file). That divergence is recorded, not papered over: the test
//! asserts *both* sides' actual values, so a change to either fails here.

use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus};
use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::rules_tables::crb::race_tables::{race_id_from_token, race_size, race_traits, RaceId};
use codex::rules_core::size::SizeCategory;
use std::path::Path;

/// The `race:<slug>` character-input token for each shipped `RaceId`, and
/// the corpus race key it must resolve to. The token column is exactly
/// what `race_id_from_token` already accepts — asserted below, so this
/// table cannot drift from the shipped one.
const RACE_ROW: &[(&str, &str)] = &[
    ("race:human", "Human"),
    ("race:dwarf", "Dwarf"),
    ("race:elf", "Elf"),
    ("race:gnome", "Gnome"),
    ("race:half-elf", "Half-Elf"),
    ("race:half-orc", "Half-Orc"),
    ("race:halfling", "Halfling"),
];

fn crb_corpus() -> RaceCorpus {
    let roots = [BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }];
    let corpus = load_race_corpus(&roots);
    assert!(
        corpus.diagnostics().is_empty(),
        "CRB race corpus must load clean; diagnostics: {:?}",
        corpus.diagnostics()
    );
    corpus
}

/// Every shipped `RaceId` has a corpus chassis, and nothing else does.
/// This is the roster half of the pin: if the corpus ever gains or loses a
/// CRB race, the hardcoded 7-variant enum and the corpus disagree and this
/// fails.
#[test]
fn the_corpus_carries_exactly_the_seven_shipped_crb_races() {
    let corpus = crb_corpus();
    let mut corpus_keys: Vec<&str> = corpus.race_keys();
    corpus_keys.sort_unstable();
    let mut expected: Vec<&str> = RACE_ROW.iter().map(|(_, key)| *key).collect();
    expected.sort_unstable();
    assert_eq!(corpus_keys, expected, "corpus CRB race roster vs the shipped RaceId enum");
    assert_eq!(RaceId::ALL.len(), RACE_ROW.len());
    for (token, key) in RACE_ROW {
        assert!(race_id_from_token(token).is_some(), "{token} must still resolve in the shipped table");
        assert!(corpus.chassis(key).is_some(), "{key} must have a corpus chassis record");
    }
}

/// Dimension 1: size. `race_size()` (hardcoded) vs the chassis
/// `FACT:BaseSize|<code>` token read through the resolver.
#[test]
fn resolved_size_matches_the_hardcoded_race_size_for_all_seven() {
    let corpus = crb_corpus();
    for (token, key) in RACE_ROW {
        let race_id = race_id_from_token(token).expect("shipped token");
        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        assert_eq!(
            resolved.size,
            Some(race_size(race_id)),
            "{key}: corpus size {:?} vs hardcoded race_size {:?}",
            resolved.size,
            race_size(race_id)
        );
    }
}

/// Dimension 2: base land speed. The hardcoded `Speed` row's `value` vs
/// the resolver's effective walk speed (chassis `MOVE:Walk`, overridden by
/// the resolved `Racial Speed` trait's own `MOVE:Walk` where one exists).
#[test]
fn resolved_walk_speed_matches_the_hardcoded_speed_row_for_all_seven() {
    let corpus = crb_corpus();
    for (token, key) in RACE_ROW {
        let race_id = race_id_from_token(token).expect("shipped token");
        let hardcoded = race_traits()
            .iter()
            .find(|t| t.race_id == race_id && t.trait_name == "Speed")
            .unwrap_or_else(|| panic!("{key} must have a hardcoded Speed row"));
        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        assert_eq!(
            resolved.walk_speed_ft,
            Some(i32::from(hardcoded.value)),
            "{key}: corpus walk speed vs hardcoded Speed row",
        );
    }
}

/// Dimension 3: senses. The hardcoded `Senses` row's `value` is `60` for
/// the two darkvision races and `0` otherwise, with the sense named in its
/// prose. The corpus states the same thing as a `VISION:` token on a
/// resolved trait — or, for Human and Halfling, by carrying no vision
/// trait at all.
#[test]
fn resolved_vision_matches_the_hardcoded_senses_row_for_all_seven() {
    let corpus = crb_corpus();
    // (race key, the VISION: payload the corpus must carry, hardcoded value)
    let expected: &[(&str, Option<&str>, i16)] = &[
        ("Dwarf", Some("Darkvision (60)"), 60),
        ("Half-Orc", Some("Darkvision (60)"), 60),
        ("Elf", Some("Low-Light Vision"), 0),
        ("Gnome", Some("Low-Light Vision"), 0),
        ("Half-Elf", Some("Low-Light Vision"), 0),
        ("Human", None, 0),
        ("Halfling", None, 0),
    ];
    for (key, vision, hardcoded_value) in expected {
        let token = RACE_ROW.iter().find(|(_, k)| k == key).expect("row").0;
        let race_id = race_id_from_token(token).expect("shipped token");
        let hardcoded = race_traits()
            .iter()
            .find(|t| t.race_id == race_id && t.trait_name == "Senses")
            .unwrap_or_else(|| panic!("{key} must have a hardcoded Senses row"));
        assert_eq!(hardcoded.value, *hardcoded_value, "{key}: hardcoded Senses row value");

        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        let corpus_vision: Vec<&str> = resolved
            .traits
            .iter()
            .flat_map(|t| t.raw_tokens.iter())
            .filter(|t| t.key == "VISION")
            .map(|t| t.value.as_str())
            .collect();
        match vision {
            Some(v) => assert_eq!(corpus_vision, vec![*v], "{key}: corpus VISION token"),
            None => assert!(
                corpus_vision.is_empty(),
                "{key}: hardcoded table says no special senses, corpus carries {corpus_vision:?}"
            ),
        }
    }
}

/// Dimension 4, the substantive one: every *named special trait* the
/// hardcoded table grounds must be present in the corpus-resolved default
/// set for that race, carrying the same numeric magnitude.
///
/// "Named special trait" = every row that is not one of the four generic
/// dimension rows (`Ability Modifiers`/`Ability Bonus`, `Size`, `Speed`,
/// `Senses`), which the three tests above already cover.
///
/// One name differs between the two paths and is aliased explicitly rather
/// than fuzzy-matched: the table calls Human's skill trait
/// `Extra Skill Ranks`; PCGen calls it `Skilled`. Same trait, same
/// `BONUS:SKILLPOINTS|NUMBER|1`.
#[test]
fn every_hardcoded_named_trait_exists_in_the_corpus_with_the_same_magnitude() {
    let corpus = crb_corpus();
    let generic = ["Ability Modifiers", "Ability Bonus", "Size", "Speed", "Senses"];
    fn alias(name: &str) -> &str {
        match name {
            "Extra Skill Ranks" => "Skilled",
            other => other,
        }
    }

    let mut checked = 0usize;
    for (token, key) in RACE_ROW {
        let race_id = race_id_from_token(token).expect("shipped token");
        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        for entry in race_traits().iter().filter(|t| t.race_id == race_id) {
            if generic.contains(&entry.trait_name) {
                continue;
            }
            let wanted = alias(entry.trait_name);
            let found = resolved
                .traits
                .iter()
                .find(|t| t.name == wanted)
                .unwrap_or_else(|| {
                    panic!(
                        "{key}: hardcoded trait {:?} (corpus name {wanted:?}) is absent from the \
                         corpus-resolved default set {:?}",
                        entry.trait_name,
                        resolved.traits.iter().map(|t| t.name.as_str()).collect::<Vec<_>>()
                    )
                });
            checked += 1;
            // `Bonus Feat` is a player choice, carried as value 0 in the
            // hardcoded table by design (see race_tables.rs's header). Its
            // corpus magnitude is the size of the granted pool, 1 — a
            // different quantity, so it is deliberately not compared.
            if entry.trait_name == "Bonus Feat" {
                assert_eq!(entry.value, 0, "Bonus Feat is a recognition-only row in the hardcoded table");
                continue;
            }
            let magnitudes = found.declared_bonus_magnitudes();
            assert!(
                magnitudes.contains(&i32::from(entry.value)),
                "{key} / {}: hardcoded value {} is not among the corpus BONUS magnitudes {:?}",
                entry.trait_name,
                entry.value,
                magnitudes
            );
        }
    }
    // Derived, not asserted from memory: 49 hardcoded rows minus the
    // 7 Size + 7 Speed + 7 Senses + 7 ability rows the other tests cover.
    assert_eq!(checked, 49 - 28, "every non-generic hardcoded row must have been checked");
}

/// Dimension 5: ability modifiers. **This was the one real drift, and it
/// is now closed — so this pins agreement, not divergence.**
///
/// ## What was wrong, and which side was right
///
/// The hardcoded table's ability-modifier prose omitted the *secondary*
/// `+2` that four of the seven races actually get, because PCGen states
/// both in one token (`BONUS:STAT|CON,WIS|2`) and the original
/// transcription read only the first stat of the pair. Elf's row went
/// further and explicitly declared the missing `+2 Intelligence` an
/// out-of-scope "alternate variant", which is wrong about the rule rather
/// than merely incomplete.
///
/// The corpus was right on all four. Established against the real PCGen
/// checkout, not against this repo's own ingest (which would be circular)
/// — `core_essentials/races/<race>/<race>_abilities_race.lst:18`:
///
/// ```text
/// Dwarf     +2 Constitution, +2 Wisdom, -2 Charisma      BONUS:STAT|CON,WIS|2  BONUS:STAT|CHA|-2
/// Elf       +2 Dexterity, +2 Intelligence, -2 Constitution BONUS:STAT|DEX,INT|2 BONUS:STAT|CON|-2
/// Gnome     +2 Constitution, +2 Charisma, -2 Strength    BONUS:STAT|CON,CHA|2  BONUS:STAT|STR|-2
/// Halfling  +2 Dexterity, +2 Charisma, -2 Strength       BONUS:STAT|DEX,CHA|2  BONUS:STAT|STR|-2
/// ```
///
/// `pilot_compute.rs`'s own per-race seams (`DWARF_WIS_ADJUSTMENT`,
/// `ELF_INT_ADJUSTMENT`, `GNOME_CHA_ADJUSTMENT`,
/// `HALFLING_CHA_ADJUSTMENT`) had already been corrected to the same
/// three-stat reading, so `race_tables.rs` was the last stale statement of
/// this fact in the engine.
///
/// ## What this test now does
///
/// It does **not** compare the table against a literal expected string —
/// that would only re-encode a transcription, which is the exact failure
/// mode being fixed. It derives the grant clause from the corpus'
/// machine-readable `BONUS:STAT` chains and requires three independent
/// statements of the same fact to agree:
///
/// 1. the chains (`BONUS:STAT|CON,WIS|2` + `BONUS:STAT|CHA|-2`),
/// 2. PCGen's own display name for the row (`+2 Constitution, +2 Wisdom,
///    -2 Charisma`), and
/// 3. `race_tables.rs`'s prose grant clause.
///
/// A future edit to any one of the three that disagrees with the other two
/// fails here.
#[test]
fn ability_modifier_rows_agree_with_the_corpus_for_every_fixed_modifier_race() {
    let corpus = crb_corpus();
    // The four races whose ability adjustment is fixed rather than a player
    // choice. Named explicitly so the set cannot silently shrink; the
    // other three are asserted to be the player-choice shape below.
    let fixed_modifier_races = ["Dwarf", "Elf", "Gnome", "Halfling"];

    for key in fixed_modifier_races {
        let token = RACE_ROW.iter().find(|(_, k)| *k == key).expect("row").0;
        let race_id = race_id_from_token(token).expect("shipped token");
        let hardcoded = race_traits()
            .iter()
            .find(|t| t.race_id == race_id && t.trait_name == "Ability Modifiers")
            .unwrap_or_else(|| panic!("{key} must have an Ability Modifiers row"));

        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        let ability = resolved
            .traits
            .iter()
            .find(|t| t.type_tokens.iter().any(|tt| tt == "Racial Ability Scores"))
            .unwrap_or_else(|| panic!("{key} must have a corpus ability-score trait"));

        // (1) Derive the grant from the machine-readable chains alone.
        let mut derived: Vec<String> = Vec::new();
        for chain in &ability.raw_bonus_chains {
            if chain.qualifiers.first().map(String::as_str) != Some("STAT") {
                continue;
            }
            let stats = chain.qualifiers.get(1).expect("a STAT chain names its stats");
            let magnitude: i32 = chain
                .qualifiers
                .get(2)
                .expect("a STAT chain carries a magnitude")
                .parse()
                .expect("the magnitude is numeric");
            for stat in stats.split(',') {
                derived.push(format!("{magnitude:+} {}", long_stat_name(stat)));
            }
        }
        assert!(
            derived.len() >= 3,
            "{key}: PF1 grants three ability adjustments; the corpus chains derived {derived:?}"
        );
        let derived = derived.join(", ");

        // (2) PCGen's own display name for the same row.
        assert_eq!(
            ability.name, derived,
            "{key}: the corpus trait name and its own BONUS:STAT chains disagree"
        );

        // (3) The shipped table's grant clause — the text before the
        //     parenthesised citation.
        let grant_clause = hardcoded.detail.split(" (").next().unwrap_or(hardcoded.detail);
        assert_eq!(
            grant_clause, derived,
            "{key}: race_tables.rs's Ability Modifiers row disagrees with the corpus. Fix the \
             row, never this assertion — the row is a transcription and this is what checks it"
        );
    }

    // The other three races take a player-chosen `+2 to One Ability Score`
    // and were never part of this drift — asserted so the fixed-modifier
    // set stays exactly four races.
    for key in ["Human", "Half-Elf", "Half-Orc"] {
        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        assert!(
            resolved.traits.iter().any(|t| t.name == "+2 to One Ability Score"),
            "{key} takes a player-chosen ability bonus in the corpus too"
        );
        let token = RACE_ROW.iter().find(|(_, k)| *k == key).expect("row").0;
        let race_id = race_id_from_token(token).expect("shipped token");
        assert!(
            race_traits()
                .iter()
                .any(|t| t.race_id == race_id && t.trait_name == "Ability Bonus"),
            "{key}'s shipped row is the player-choice `Ability Bonus` shape, not a fixed pair"
        );
    }
}

fn long_stat_name(code: &str) -> &'static str {
    match code.trim() {
        "STR" => "Strength",
        "DEX" => "Dexterity",
        "CON" => "Constitution",
        "INT" => "Intelligence",
        "WIS" => "Wisdom",
        "CHA" => "Charisma",
        other => panic!("unknown ability code {other}"),
    }
}

/// The corpus carries strictly more than the hardcoded table does — the
/// traits PCGen grounds that the 49-row curated table never had. Named
/// explicitly so "the corpus has more rows" is a stated, reviewed fact
/// rather than an unexamined count difference.
#[test]
fn the_corpus_adds_named_traits_the_hardcoded_table_never_carried() {
    let corpus = crb_corpus();
    let added: &[(&str, &[&str])] = &[
        ("Dwarf", &["Hatred", "Languages", "Weapon Familiarity"]),
        ("Elf", &["Languages", "Weapon Familiarity"]),
        ("Gnome", &["Gnome Magic", "Obsessive", "Languages", "Weapon Familiarity"]),
        ("Half-Elf", &["Adaptability", "Elf Blood", "Multitalented", "Languages"]),
        ("Half-Orc", &["Orc Blood", "Orc Ferocity", "Weapon Familiarity", "Languages"]),
        ("Halfling", &["Languages", "Weapon Familiarity"]),
        ("Human", &["Languages"]),
    ];
    for (key, names) in added {
        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        for name in *names {
            assert!(
                resolved.traits.iter().any(|t| &t.name == name),
                "{key}: corpus must carry {name:?}"
            );
        }
    }
    // Derived count, both sides.
    let corpus_total: usize =
        RACE_ROW.iter().map(|(_, k)| corpus.resolve(k, &[]).expect("resolves").traits.len()).sum();
    assert_eq!(race_traits().len(), 49, "the shipped hardcoded table's row count");
    assert_eq!(corpus_total, 67, "the corpus' CRB default-trait count");
}

/// Sanity: the resolver used by every assertion above is genuinely reading
/// on-disk JSON, not a fixture. If `data/corpus/core_rulebook/race/` were
/// empty this whole file would vacuously pass, so this asserts the load is
/// non-empty and that a real source path came back with it.
#[test]
fn the_pin_reads_real_on_disk_corpus_records() {
    let corpus = crb_corpus();
    let dwarf = corpus.chassis("Dwarf").expect("Dwarf chassis");
    assert!(
        dwarf.source_path.ends_with("core_essentials/races/dwarf/dwarf_races.lst"),
        "real LST provenance, got {:?}",
        dwarf.source_path
    );
    assert_eq!(dwarf.book_id, "core_rulebook", "attributed to the true source book, never core_essentials");
    let greed = corpus.traits_for("Dwarf").into_iter().find(|t| t.data.name == "Greed").expect("Greed");
    assert_eq!(greed.data.source_page.as_deref(), Some("p.21"));
}

/// `SizeCategory` is re-exported through the resolver's own resolution, so
/// a caller never has to re-parse `FACT:BaseSize`. Pins the two Small
/// races, which is the fact carrying capacity depends on.
#[test]
fn exactly_gnome_and_halfling_resolve_small_from_the_corpus() {
    let corpus = crb_corpus();
    let small: Vec<&str> = RACE_ROW
        .iter()
        .map(|(_, k)| *k)
        .filter(|k| corpus.resolve(k, &[]).expect("resolves").size == Some(SizeCategory::Small))
        .collect();
    assert_eq!(small, vec!["Gnome", "Halfling"]);
}
