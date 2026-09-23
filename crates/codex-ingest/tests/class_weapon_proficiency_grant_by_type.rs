//! SD-36 Epic F1c-1 (defect D1, `epic-f-class-completion.md` §3 F1, review findings 1 and 15):
//! a "grant every ability of type X" row -- `ABILITY:<category>|AUTOMATIC|TYPE=<tag>` -- converts
//! to grant edges onto every converted record of that category whose own `TYPE` facet carries
//! the tag(s), instead of being dropped into `data/sheet_rules/_defects/grant-by-type.json`.
//!
//! The weapon case is the one the F1 reader exposed: the pinned oracle's class-feature rows name
//! the tier they grant through `ABILITY:Internal|AUTOMATIC|TYPE=WeaponProfMartial` (or `...Simple`),
//! and the oracle's own `Weapon Prof ~ Auto` / `~ Simple` / `~ Martial` Internal abilities
//! (`cr_abilities_class.lst:2799-2801`) carry those tags and the `AUTO:WEAPONPROF|TYPE=<tier>`
//! grant. The converter never maps a selector spelling to a tier itself; the tier arrives through
//! the target record the oracle names, the same record every by-name grant already reaches.
//!
//! These tests read the generated package through the F1 reader
//! (`codex::rules_core::pilot_compute::class_proficiency_sheet_rules`).

use codex::rules_core::pilot_compute::class_proficiency_sheet_rules::{class_weapon_proficiency_view, ProficiencyAnswer};
use codex::rules_core::rules_tables::crb::weapon_tables::WeaponProficiency;

fn tiers_at_level_1(class: &str) -> Vec<WeaponProficiency> {
    match class_weapon_proficiency_view(class, 1) {
        ProficiencyAnswer::Known(view) => view.tiers.clone(),
        ProficiencyAnswer::Unknown { reason } => panic!("{class} L1 must be Known, reader said Unknown: {reason}"),
    }
}

fn assert_tiers(class: &str, expected: &[WeaponProficiency]) {
    let tiers = tiers_at_level_1(class);
    for t in expected {
        assert!(tiers.contains(t), "{class} L1 must hold the {t:?} tier; reader tiers {tiers:?}");
    }
}

/// Antipaladin (`apg_abilities_class.lst`, `ABILITY:Internal|AUTOMATIC|TYPE=WeaponProfMartial`)
/// and Magus (`um_abilities_class.lst`, the same selector) were Blocked solely on the dropped
/// grant. Per the oracle, `TYPE=WeaponProfMartial` reaches `Weapon Prof ~ Simple` (tagged
/// `WeaponProfSimple.WeaponProfMartial`) and `Weapon Prof ~ Martial`: both tiers.
#[test]
fn antipaladin_and_magus_gain_the_martial_tier() {
    for class in ["antipaladin", "magus"] {
        assert_tiers(class, &[WeaponProficiency::Simple, WeaponProficiency::Martial]);
    }
}

/// The three classes that read Known but a tier short: alchemist and inquisitor
/// (`TYPE=WeaponProfSimple`) lacked Simple, gunslinger (`TYPE=WeaponProfMartial` on
/// `gunslinger_proficiencies`) lacked Martial.
#[test]
fn alchemist_inquisitor_gunslinger_gain_their_missing_tier() {
    assert_tiers("alchemist", &[WeaponProficiency::Simple]);
    assert_tiers("inquisitor", &[WeaponProficiency::Simple]);
    assert_tiers("gunslinger", &[WeaponProficiency::Simple, WeaponProficiency::Martial]);
}

/// The classes whose whole weapon closure was the dropped grant read Unknown (an empty walk);
/// with the grant converted they answer Known with the tier the oracle names. Three classes whose
/// proficiency records now carry the converted grant are NOT in this list, each for a named
/// mechanism this step does not touch:
/// - psychic and spiritualist: each class-feature record is also shut by the hoisted
///   `Psychic_CF_Knacks` / `Spiritualist_CF_Knacks` condition (defect D2, F1c-2);
/// - summoner: `summoner_weapon_and_armor_proficiency` now grants `Weapon Prof ~ Simple`, but it
///   is reached only through `summoner_standard_class` (the `Summoner Class Selection` pick,
///   `apg_abilities_class.lst:741`), whose `applies` reads `Standard Summoner Allowed == 1` -- a
///   variable whose only contributor is `advanced_players_guide:class_feature:default` (rows
///   `apg_abilities_class.lst:715,717`), a record outside the summoner's class closure, so the
///   level-1 walk never admits it.
#[test]
fn cavalier_oracle_and_the_occult_classes_answer_known() {
    let cases: &[(&str, &[WeaponProficiency])] = &[
        ("cavalier", &[WeaponProficiency::Simple, WeaponProficiency::Martial]),
        ("oracle", &[WeaponProficiency::Simple]),
        ("kineticist", &[WeaponProficiency::Simple]),
        ("occultist", &[WeaponProficiency::Simple, WeaponProficiency::Martial]),
        ("vigilante", &[WeaponProficiency::Simple, WeaponProficiency::Martial]),
    ];
    for (class, tiers) in cases {
        assert_tiers(class, tiers);
    }
}

/// The named-defect contract, checked against the package itself: every row left in
/// `_defects/grant-by-type.json` names a category/tag pair NO converted record carries (so the
/// converter dropped nothing it could have resolved), and no `TYPE=WeaponProf*` row is left at
/// all -- before F1c-1 there were 47 (28 Martial, 17 Simple, 2 Auto) of 613 rows.
#[test]
fn every_grant_by_type_row_left_is_unexpandable() {
    let path = codex_ingest::repo_root().join("data/sheet_rules/_defects/grant-by-type.json");
    let lines: Vec<String> = serde_json::from_str(&std::fs::read_to_string(&path).expect("defect file readable")).expect("defect file parses");
    let package = codex::rules_core::sheet_rule_package::package().as_ref().expect("package loads");
    let mut expandable = Vec::new();
    let weapon_rows: Vec<&String> = lines.iter().filter(|l| l.contains("TYPE=WeaponProf")).collect();
    assert!(weapon_rows.is_empty(), "{} weapon grant-by-type rows remain: {weapon_rows:?}", weapon_rows.len());
    for line in &lines {
        let Some((_, rest)) = line.split_once(": ") else { continue };
        let Some((category, selector)) = rest.split_once('|') else { continue };
        let tags: Vec<String> = selector.trim_start_matches("TYPE=").split('.').map(|t| t.to_ascii_lowercase()).collect();
        let pool = codex_ingest::pcgen_import::sheet_rule::ctx::slug(category);
        let hit = package
            .rules
            .values()
            .filter(|r| r.pool == pool)
            .find(|r| tags.iter().all(|t| r.tags.iter().any(|x| x.eq_ignore_ascii_case(t))));
        if let Some(r) = hit {
            expandable.push(format!("{line} -> {}", r.id));
        }
    }
    assert!(expandable.is_empty(), "{} grant-by-type rows name a selector a converted record carries:\n{}", expandable.len(), expandable.join("\n"));
}
