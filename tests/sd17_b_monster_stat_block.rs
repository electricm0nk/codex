//! SD-22 Epic 5 parser-gap acceptance test — bare tab-delimited monster
//! stat-block rows in `b1_races.lst`.
//!
//! `src/pcgen_import/lst_parser/race_ability.rs`'s `parse_lst_entry` only
//! recognizes `RACE:`/`RACES:` pointer lines and `ABILITY:` declarations;
//! the real Bestiary 1 monster records in `b1_races.lst` are bare
//! tab-delimited rows with the monster name as the unprefixed first
//! field (confirmed: `grep -c "RACE:" b1_races.lst` -> 0) — that parser
//! extracts zero records from the file. This test proves the new sibling
//! parser, `pcgen_import::lst_parser::monster_stat_block`, closes that
//! gap against the real corpus file, mirroring the established
//! `PCGEN_CORPUS_ROOT`-gated real-corpus pattern in
//! `tests/sd17_b_spellcasting_class.rs` /
//! `tests/sd22_apg_class_alchemist_resolves.rs`.
//!
//! Real-corpus grounding for the five monsters asserted below (all
//! verified directly against
//! `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`, not from
//! memory): Ghoul (line 200), Gnoll (line 212), Goblin Dog (line 213),
//! Lizardfolk (line 276), Wolf (line 414) — see
//! `docs/release/SD-22/artifacts/beastiary1/subset_01_cycle_receipt.md`
//! for the roster-correction note (the operator-pinned "Goblin, Kobold,
//! Orc, Skeleton, Zombie" sample list in `corpus-source-inventory.md`
//! §3.1 does not correspond to real standalone CR-1 monster stat-block
//! rows in this corpus file — Goblin/Kobold/Orc exist only as `.MOD`
//! overrides onto their *playable-race* records, and Skeleton (Human) /
//! Zombie (Human) are CR 1/3 and CR 1/2 respectively, not CR 1).

use std::fs;
use std::path::PathBuf;

use codex::pcgen_import::lst_parser::monster_stat_block::parse_monster_stat_block_entries;

fn real_b1_races_lst() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source = corpus_root.join("pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst");
    fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real b1_races.lst at {}: {err}", source.display()))
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_cr_1_monster_records_from_b1_races_lst() {
    let text = real_b1_races_lst();
    let records = parse_monster_stat_block_entries("b1_races.lst", &text);

    let find = |name: &str| {
        records
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("expected a parsed record named {name:?} from the real b1_races.lst"))
    };

    let ghoul = find("Ghoul");
    assert_eq!(ghoul.challenge_rating.as_f32(), 1.0);
    assert_eq!(ghoul.size.as_deref(), Some("M"));
    assert_eq!(ghoul.speed_ft, Some(30));
    assert_eq!(ghoul.race_type.as_deref(), Some("Undead"));
    assert_eq!(ghoul.source_page.as_deref(), Some("p.146"));
    assert!(ghoul.natural_attacks.iter().any(|a| a.name == "Claw" && a.damage_dice == "1d6"));
    assert!(ghoul.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d6"));

    let gnoll = find("Gnoll");
    assert_eq!(gnoll.challenge_rating.as_f32(), 1.0);
    assert_eq!(gnoll.race_type.as_deref(), Some("Humanoid"));
    assert_eq!(gnoll.race_subtype.as_deref(), Some("Gnoll"));
    assert_eq!(gnoll.source_page.as_deref(), Some("p.155"));

    let goblin_dog = find("Goblin Dog");
    assert_eq!(goblin_dog.challenge_rating.as_f32(), 1.0);
    assert_eq!(goblin_dog.speed_ft, Some(50));
    assert_eq!(goblin_dog.race_type.as_deref(), Some("Animal"));
    assert!(
        goblin_dog
            .natural_attacks
            .iter()
            .any(|a| a.name == "Bite" && a.damage_dice == "1d6")
    );

    let lizardfolk = find("Lizardfolk");
    assert_eq!(lizardfolk.challenge_rating.as_f32(), 1.0);
    assert_eq!(lizardfolk.speed_ft, Some(30), "walk speed, not the swim speed");
    assert_eq!(lizardfolk.race_type.as_deref(), Some("Humanoid"));
    assert_eq!(lizardfolk.race_subtype.as_deref(), Some("Reptilian"));
    assert!(lizardfolk.natural_attacks.iter().any(|a| a.name == "Claw" && a.damage_dice == "1d4"));
    assert!(lizardfolk.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d4"));

    let wolf = find("Wolf");
    assert_eq!(wolf.challenge_rating.as_f32(), 1.0);
    assert_eq!(wolf.speed_ft, Some(50));
    assert_eq!(wolf.race_type.as_deref(), Some("Animal"));
    assert_eq!(wolf.source_page.as_deref(), Some("p.278"));

    // Confirms the roster-correction note: Goblin, Kobold, and Orc are
    // NOT parsed as standalone monster stat-block records from this file
    // (they exist only as `.MOD` overrides in `b1_races_pc.lst`, a
    // different file, onto their playable-race base, which this parser
    // deliberately does not treat as a fresh record).
    assert!(records.iter().all(|r| r.name != "Goblin"));
    assert!(records.iter().all(|r| r.name != "Kobold"));
    assert!(records.iter().all(|r| r.name != "Orc"));
}
