//! Starfinder 1e mapping table for the overloaded PCGen fields (SD-37 E3.3, `decisions.md §8`).
//!
//! PCGen's Starfinder data reuses Pathfinder field names with other meanings (`HP|ALTHP` is
//! Stamina, `COMBAT|AC` is two armour classes, `FACT:KeyAbilityScore` is sometimes a choice).
//! `sf-mapping-table.v1.json` says, per sheet field, which oracle tokens feed it. These tests hold
//! that table to three independent sources at once:
//!
//! 1. the SRD hand values (`artifacts/epic_0/seed-hand-values.md`, transcribed by E0.4 from bytes
//!    the engine does not read);
//! 2. real PCGen runs of named builds (`token-mapping/oracle-builds/*.oracle.txt`, written by
//!    `run_oracle_builds.sh` through `scripts/pcgen-run-character.sh`);
//! 3. the pinned oracle `.lst` tokens the evaluator reads.
//!
//! The planted mutations M1–M4 (`sf_mapping_mutations.py`) edit the table and must turn
//! `seed_fixtures_match_the_srd_hand_values_and_the_oracle` red. The tests skip, loudly, when the
//! oracle checkout is absent, like every corpus test in this crate.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codex_ingest::pcgen_import::sheet_rule::closure;
use codex_ingest::pcgen_import::sheet_rule::sf_mapping::{
    SF_MAPPING_TABLE, SfBuild, SfSheet, Stat, evaluate, load_table, parse_key_ability_fact,
};

const PACKAGE: &str = "docs/release/SD-37-starfinder-1e";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn sf_root() -> Option<PathBuf> {
    let root = closure::corpus_root().join("starfinder");
    if root.is_dir() {
        Some(root)
    } else {
        eprintln!("skipped: no {} (run scripts/fetch-pcgen-oracle.sh)", root.display());
        None
    }
}

fn oracle_builds_dir() -> PathBuf {
    repo_root().join(PACKAGE).join("artifacts/epic_3/token-mapping/oracle-builds")
}

/// `key=value` lines of one committed PCGen run (`<build>.oracle.txt`).
fn oracle_run(build: &str) -> BTreeMap<String, String> {
    let path = oracle_builds_dir().join(format!("{build}.oracle.txt"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    text.lines()
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}

fn oracle_int(run: &BTreeMap<String, String>, key: &str) -> i64 {
    let v = run.get(key).unwrap_or_else(|| panic!("oracle run has no {key}"));
    v.trim_start_matches('+').parse().unwrap_or_else(|_| panic!("{key}={v} is not an integer"))
}

/// The SRD hand values, `| SF-<seed> | <field> | <value> | ... |` rows of `seed-hand-values.md`.
fn hand_values() -> BTreeMap<(String, String), i64> {
    let path = repo_root().join(PACKAGE).join("artifacts/epic_0/seed-hand-values.md");
    let text = std::fs::read_to_string(&path).unwrap();
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        if cells.len() > 4
            && cells[1].starts_with("SF-")
            && let Ok(v) = cells[3].trim_start_matches('+').parse::<i64>()
        {
            out.entry((cells[1].to_string(), cells[2].to_string())).or_insert(v);
        }
    }
    out
}

/// The four seeds, as `artifacts/epic_0/seed-builds.md` §1–§4 specify them (final scores, after
/// race, theme and the 5th-level increase). The oracle runs print the same scores; the test checks.
fn seeds() -> Vec<(&'static str, &'static str, SfBuild)> {
    vec![
        (
            "SF-Soldier-3",
            "sf_seed_soldier_3",
            SfBuild {
                race: "Human".into(),
                class: "Soldier".into(),
                level: 3,
                scores: [16, 14, 12, 11, 10, 10],
                key_ability_choice: Some(Stat::Str),
                feats: vec!["Weapon Focus".into(), "Quick Draw".into(), "Deadly Aim".into(), "Coordinated Shot".into()],
                armor_key: Some("Defiance Series (Squad)".into()),
            },
        ),
        (
            "SF-Mystic-5",
            "sf_seed_mystic_5",
            SfBuild {
                race: "Lashunta".into(),
                class: "Mystic".into(),
                level: 5,
                scores: [10, 14, 8, 14, 19, 15],
                key_ability_choice: None,
                feats: vec!["Spell Penetration".into(), "Spell Focus".into(), "Quick Draw".into()],
                armor_key: Some("Lashunta tempweave (basic)".into()),
            },
        ),
        (
            "SF-Technomancer-5",
            "sf_seed_technomancer_5",
            SfBuild {
                race: "Android".into(),
                class: "Technomancer".into(),
                level: 5,
                scores: [10, 16, 14, 19, 13, 8],
                key_ability_choice: None,
                feats: vec![],
                armor_key: Some("D-suit I".into()),
            },
        ),
        (
            "SF-Envoy-3",
            "sf_seed_envoy_3",
            SfBuild {
                race: "Ysoki".into(),
                class: "Envoy".into(),
                level: 3,
                scores: [8, 13, 12, 12, 10, 18],
                key_ability_choice: None,
                feats: vec![],
                armor_key: Some("Carbon skin (graphite)".into()),
            },
        ),
    ]
}

const STATS: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];

fn check_scores(build_name: &str, build: &SfBuild, run: &BTreeMap<String, String>, bad: &mut Vec<String>) {
    for (i, s) in STATS.iter().enumerate() {
        let oracle = oracle_int(run, &format!("stat.{s}"));
        if oracle != build.scores[i] {
            bad.push(format!("{build_name}: fixture {s} {} != oracle run {oracle}", build.scores[i]));
        }
    }
}

fn compare(build_name: &str, field: &str, ours: i64, expected: i64, source: &str, bad: &mut Vec<String>) {
    if ours != expected {
        bad.push(format!("{build_name} {field}: mapping gives {ours}, {source} says {expected}"));
    }
}

#[test]
fn seed_fixtures_match_the_srd_hand_values_and_the_oracle() {
    let Some(root) = sf_root() else { return };
    let table = load_table(&repo_root().join(SF_MAPPING_TABLE)).expect("SF mapping table loads");
    let hand = hand_values();
    let mut bad = Vec::new();
    for (seed, build_name, build) in seeds() {
        let run = oracle_run(build_name);
        check_scores(seed, &build, &run, &mut bad);
        let sheet: SfSheet = match evaluate(&table, &root, &build) {
            Ok(s) => s,
            Err(e) => {
                bad.push(format!("{seed}: refused: {e}"));
                continue;
            }
        };
        for (field, ours, oracle_key) in [
            ("HP", sheet.hit_points, "hp"),
            ("Stamina", sheet.stamina, "althp"),
            ("Resolve", sheet.resolve, "var.resolve"),
            ("EAC", sheet.eac, "ac.eac"),
            ("KAC", sheet.kac, "ac.kac"),
        ] {
            let srd = *hand.get(&(seed.to_string(), field.to_string())).unwrap_or_else(|| panic!("no hand value {seed} {field}"));
            compare(seed, field, ours, srd, "the SRD hand value", &mut bad);
            compare(seed, field, ours, oracle_int(&run, oracle_key), "the PCGen run", &mut bad);
        }
        let key_mod = build.modifier(sheet.key_ability);
        compare(seed, "key ability modifier", key_mod, oracle_int(&run, "var.key_ability_bonus"), "the PCGen run", &mut bad);
        if !sheet.unclaimed.is_empty() {
            bad.push(format!("{seed}: oracle tokens no row claims: {:?}", sheet.unclaimed));
        }
    }
    assert!(bad.is_empty(), "SF mapping disagrees with the seed fixtures:\n{}", bad.join("\n"));
}

/// PCGen's own two Starfinder test characters (`code/testsuite/PCGfiles/sf_soldier.pcg`, an
/// android soldier 10 with Extra Resolve; `sf_mechanic.pcg`, a ysoki mechanic 20 with Toughness and
/// Extra Resolve). No SRD hand values exist for them; they reach terms no seed has (Toughness,
/// Extra Resolve, a Dex choice for the soldier's key ability) and levels 10 and 20.
#[test]
fn pcgen_test_characters_match_the_oracle() {
    let Some(root) = sf_root() else { return };
    let table = load_table(&repo_root().join(SF_MAPPING_TABLE)).expect("SF mapping table loads");
    let pcg_dir = closure::corpus_root().join("../code/testsuite/PCGfiles");
    let mut bad = Vec::new();
    for (build_name, race, class, level, choice, armor) in [
        ("sf_soldier", "Android", "Soldier", 10, Some(Stat::Dex), Some("Aegis Series (Squad)")),
        ("sf_mechanic", "Ysoki", "Mechanic", 20, None, None),
    ] {
        let run = oracle_run(build_name);
        let pcg = std::fs::read_to_string(pcg_dir.join(format!("{build_name}.pcg"))).expect("PCGen test character");
        // every feat the character carries, so the unclaimed-token scan sees all of them
        let feats: Vec<String> = pcg
            .lines()
            .filter(|l| l.contains("|CATEGORY:FEAT|KEY:"))
            .filter_map(|l| l.split("|CATEGORY:FEAT|KEY:").nth(1))
            .map(|rest| rest.split('|').next().unwrap().to_string())
            .collect();
        let mut scores = [0i64; 6];
        for (i, s) in STATS.iter().enumerate() {
            scores[i] = oracle_int(&run, &format!("stat.{s}"));
        }
        let build = SfBuild {
            race: race.into(),
            class: class.into(),
            level,
            scores,
            key_ability_choice: choice,
            feats,
            armor_key: armor.map(Into::into),
        };
        let sheet = evaluate(&table, &root, &build).unwrap_or_else(|e| panic!("{build_name}: {e}"));
        compare(build_name, "HP", sheet.hit_points, oracle_int(&run, "hp"), "the PCGen run", &mut bad);
        compare(build_name, "Stamina", sheet.stamina, oracle_int(&run, "althp"), "the PCGen run", &mut bad);
        compare(build_name, "Resolve", sheet.resolve, oracle_int(&run, "var.resolve"), "the PCGen run", &mut bad);
        compare(build_name, "key ability modifier", build.modifier(sheet.key_ability), oracle_int(&run, "var.key_ability_bonus"), "the PCGen run", &mut bad);
        if armor.is_some() {
            compare(build_name, "EAC", sheet.eac, oracle_int(&run, "ac.eac"), "the PCGen run", &mut bad);
            compare(build_name, "KAC", sheet.kac, oracle_int(&run, "ac.kac"), "the PCGen run", &mut bad);
        }
        if !sheet.unclaimed.is_empty() {
            bad.push(format!("{build_name}: oracle tokens no row claims: {:?}", sheet.unclaimed));
        }
    }
    assert!(bad.is_empty(), "SF mapping disagrees with PCGen's test characters:\n{}", bad.join("\n"));
}

/// `decisions.md §8`: every row cites the SRD rule and an oracle observation; a recorded
/// observation is the value the committed PCGen run printed, never a retyped number.
#[test]
fn every_row_cites_the_srd_and_an_oracle_observation_that_matches_the_run() {
    let table = load_table(&repo_root().join(SF_MAPPING_TABLE)).expect("SF mapping table loads");
    let mut bad = Vec::new();
    for row in &table.rows {
        if row.srd.is_empty() || row.srd.iter().any(|c| !c.url.starts_with("https://www.aonsrd.com/") || c.section.trim().is_empty()) {
            bad.push(format!("{}: SRD citation missing or without a section", row.id));
        }
        if row.oracle.is_empty() {
            bad.push(format!("{}: no oracle observation", row.id));
        }
        for obs in &row.oracle {
            let run = oracle_run(&obs.build);
            match run.get(&obs.key) {
                Some(v) if v.trim_start_matches('+') == obs.value.trim_start_matches('+') => {}
                other => bad.push(format!("{} {} {}: table says {}, run says {:?}", row.id, obs.build, obs.key, obs.value, other)),
            }
        }
        if row.pf_reading.trim().is_empty() {
            bad.push(format!("{}: no statement of what the PF reading would print", row.id));
        }
    }
    for want in ["hit_points", "stamina", "eac", "kac", "key_ability", "resolve"] {
        if !table.rows.iter().any(|r| r.id == want) {
            bad.push(format!("no row for {want}"));
        }
    }
    assert!(!table.reused_pf_fields.is_empty(), "the receipt-facing list of PF mappings reused unchanged is empty");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The overload is real in the oracle: PCGen's `AC.Total` (what the Pathfinder reading prints)
/// equals neither EAC nor KAC on any of the six runs.
#[test]
fn the_pathfinder_ac_total_is_neither_eac_nor_kac() {
    for build in ["sf_seed_soldier_3", "sf_seed_mystic_5", "sf_seed_technomancer_5", "sf_seed_envoy_3", "sf_soldier", "sf_mechanic"] {
        let run = oracle_run(build);
        let (total, eac, kac) = (oracle_int(&run, "ac.total"), oracle_int(&run, "ac.eac"), oracle_int(&run, "ac.kac"));
        assert!(total != eac && total != kac, "{build}: AC.Total {total} vs EAC {eac} / KAC {kac}");
        // EAC = TOTAL minus the KAC types and vice versa (game mode ACTYPE rows)
        assert_eq!(total - oracle_int(&run, "ac.kac_armor"), eac, "{build}");
        assert_eq!(total - oracle_int(&run, "ac.eac_armor"), kac, "{build}");
    }
}

#[test]
fn key_ability_fact_forms_parse_fixed_and_choice() {
    assert_eq!(parse_key_ability_fact("CHA").unwrap(), vec![Stat::Cha]);
    assert_eq!(parse_key_ability_fact("Str or Dex").unwrap(), vec![Stat::Str, Stat::Dex]);
    assert_eq!(parse_key_ability_fact("INT or WIS").unwrap(), vec![Stat::Int, Stat::Wis]);
    assert!(parse_key_ability_fact("Strength").is_err());
    assert!(parse_key_ability_fact("").is_err());
}

/// Every `FACT:KeyAbilityScore` value in all SF data trees parses (fixed or choice form).
#[test]
fn every_key_ability_fact_in_the_oracle_parses() {
    let Some(root) = sf_root() else { return };
    let mut seen = 0;
    let mut stack = vec![root];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "lst") {
                let text = String::from_utf8_lossy(&std::fs::read(&p).unwrap()).into_owned();
                for line in text.lines().filter(|l| !l.starts_with('#')) {
                    for f in line.split('\t').filter_map(|f| f.strip_prefix("FACT:KeyAbilityScore|")) {
                        seen += 1;
                        parse_key_ability_fact(f).unwrap_or_else(|e| panic!("{}: {f}: {e}", p.display()));
                    }
                }
            }
        }
    }
    // the 10 player classes; the receipt carries the independent awk count
    assert_eq!(seen, 10, "FACT:KeyAbilityScore values across the SF trees");
}

/// A choice-form class without a choice is a named refusal, never a guess.
#[test]
fn a_choice_key_ability_without_a_choice_is_refused() {
    let Some(root) = sf_root() else { return };
    let table = load_table(&repo_root().join(SF_MAPPING_TABLE)).expect("SF mapping table loads");
    let (_, _, mut build) = seeds().remove(0);
    build.key_ability_choice = None;
    let err = evaluate(&table, &root, &build).expect_err("Soldier without a key-ability choice");
    assert!(err.contains("Str or Dex"), "{err}");
    build.key_ability_choice = Some(Stat::Wis);
    let err = evaluate(&table, &root, &build).expect_err("Soldier choosing Wis");
    assert!(err.contains("Str or Dex"), "{err}");
}
