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

/// Grounding test for subset 02's roster (see
/// `docs/release/SD-22/artifacts/beastiary1/subset_02_cycle_receipt.md`).
/// No parser widening was needed for subset 02 — every field these five
/// monsters use already falls inside `monster_stat_block`'s existing
/// recognition surface — but this test proves that against the real
/// corpus file directly, the same way subset 01's grounding test did.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_subset_02_cr_1_monster_records_from_b1_races_lst() {
    let text = real_b1_races_lst();
    let records = parse_monster_stat_block_entries("b1_races.lst", &text);

    let find = |name: &str| {
        records
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("expected a parsed record named {name:?} from the real b1_races.lst"))
    };

    let darkmantle = find("Darkmantle");
    assert_eq!(darkmantle.challenge_rating.as_f32(), 1.0);
    assert_eq!(darkmantle.size.as_deref(), Some("S"));
    assert_eq!(darkmantle.speed_ft, Some(20));
    assert_eq!(darkmantle.race_type.as_deref(), Some("Magical Beast"));
    assert_eq!(darkmantle.source_page.as_deref(), Some("p.55"));
    assert!(darkmantle.natural_attacks.iter().any(|a| a.name == "Slam" && a.damage_dice == "1d4"));

    let horse = find("Horse");
    assert_eq!(horse.challenge_rating.as_f32(), 1.0);
    assert_eq!(horse.speed_ft, Some(50));
    assert_eq!(horse.race_type.as_deref(), Some("Animal"));
    assert_eq!(horse.source_page.as_deref(), Some("p.177"));

    let hyena = find("Hyena");
    assert_eq!(hyena.challenge_rating.as_f32(), 1.0);
    assert_eq!(hyena.speed_ft, Some(50));
    assert_eq!(hyena.race_type.as_deref(), Some("Animal"));
    assert_eq!(hyena.source_page.as_deref(), Some("p.179"));

    let octopus = find("Octopus");
    assert_eq!(octopus.challenge_rating.as_f32(), 1.0);
    assert_eq!(octopus.speed_ft, Some(20), "walk speed, not the swim speed");
    assert_eq!(octopus.race_type.as_deref(), Some("Animal"));
    assert_eq!(octopus.race_subtype.as_deref(), Some("Aquatic"));
    assert!(octopus.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d3"));
    assert!(octopus.natural_attacks.iter().any(|a| a.name == "Tentacle"));

    let spider_swarm = find("Spider Swarm");
    assert_eq!(spider_swarm.challenge_rating.as_f32(), 1.0);
    assert_eq!(spider_swarm.speed_ft, Some(20), "walk speed, not the climb speed");
    assert_eq!(spider_swarm.race_type.as_deref(), Some("Vermin"));
    assert_eq!(spider_swarm.race_subtype.as_deref(), Some("Swarm"));
    assert!(spider_swarm.natural_attacks.iter().any(|a| a.name == "Swarm" && a.damage_dice == "1d6"));

    // Confirms the roster-correction note: Hobgoblin is NOT parsed as a
    // standalone monster stat-block record from this file (it exists
    // only as a `.MOD` override in `b1_races_pc.lst`, same shape as
    // subset 01's Goblin/Kobold/Orc). Rat Swarm DOES parse as a real,
    // standalone record here — but at CR 2, not CR 1, so it belongs to a
    // later CR-band subset, not subset 02.
    assert!(records.iter().all(|r| r.name != "Hobgoblin"));
    let rat_swarm = find("Rat Swarm");
    assert_eq!(rat_swarm.challenge_rating.as_f32(), 2.0, "Rat Swarm is CR 2, not CR 1");
}

/// Grounding test for subset 03's roster (see
/// `docs/release/SD-22/artifacts/beastiary1/subset_03_cycle_receipt.md`).
/// Subset 03 moves to CR 2 (CR 1 is exhausted after subsets 01+02 leave
/// only Squid/Troglodyte, too few for a five-monster subset). No parser
/// widening was needed — every field these five monsters use already
/// falls inside `monster_stat_block`'s existing recognition surface —
/// but this test proves that against the real corpus file directly, the
/// same way subset 01's and subset 02's grounding tests did.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_subset_03_cr_2_monster_records_from_b1_races_lst() {
    let text = real_b1_races_lst();
    let records = parse_monster_stat_block_entries("b1_races.lst", &text);

    let find = |name: &str| {
        records
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("expected a parsed record named {name:?} from the real b1_races.lst"))
    };

    let bat_swarm = find("Bat Swarm");
    assert_eq!(bat_swarm.challenge_rating.as_f32(), 2.0);
    assert_eq!(bat_swarm.size.as_deref(), Some("D"));
    assert_eq!(bat_swarm.speed_ft, Some(5), "walk speed, not the fly speed");
    assert_eq!(bat_swarm.race_type.as_deref(), Some("Animal"));
    assert_eq!(bat_swarm.race_subtype.as_deref(), Some("Swarm"));
    assert_eq!(bat_swarm.source_page.as_deref(), Some("p.30"));
    assert!(bat_swarm.natural_attacks.iter().any(|a| a.name == "Swarm" && a.damage_dice == "1d6"));

    let boar = find("Boar");
    assert_eq!(boar.challenge_rating.as_f32(), 2.0);
    assert_eq!(boar.speed_ft, Some(40));
    assert_eq!(boar.race_type.as_deref(), Some("Animal"));
    assert_eq!(boar.source_page.as_deref(), Some("p.36"));
    assert!(boar.natural_attacks.is_empty(), "Gore is an ABILITY:Internal cross-reference, not a NATURALATTACKS: token");

    let boggard = find("Boggard");
    assert_eq!(boggard.challenge_rating.as_f32(), 2.0);
    assert_eq!(boggard.speed_ft, Some(20), "walk speed, not the swim speed");
    assert_eq!(boggard.race_type.as_deref(), Some("Humanoid"));
    assert_eq!(boggard.race_subtype.as_deref(), Some("Boggard"));
    assert_eq!(boggard.source_page.as_deref(), Some("p.37"));
    assert!(boggard.natural_attacks.iter().any(|a| a.name == "Tongue"));

    let bugbear = find("Bugbear");
    assert_eq!(bugbear.challenge_rating.as_f32(), 2.0);
    assert_eq!(bugbear.speed_ft, Some(30));
    assert_eq!(bugbear.race_type.as_deref(), Some("Humanoid"));
    assert_eq!(bugbear.race_subtype.as_deref(), Some("Goblinoid"));
    assert_eq!(bugbear.source_page.as_deref(), Some("p.38"));
    assert!(bugbear.natural_attacks.is_empty());

    let cave_fisher = find("Cave Fisher");
    assert_eq!(cave_fisher.challenge_rating.as_f32(), 2.0);
    assert_eq!(cave_fisher.speed_ft, Some(20), "walk speed, not the climb speed");
    assert_eq!(cave_fisher.race_type.as_deref(), Some("Vermin"));
    assert_eq!(cave_fisher.source_page.as_deref(), Some("p.41"));
    assert!(cave_fisher.natural_attacks.iter().any(|a| a.name == "Filament"));

    // Confirms the CR-band-move note: Squid and Troglodyte ARE real,
    // standalone, unused CR:1 records (the two that made CR 1
    // "insufficient, not absent" for a five-monster subset), but this
    // cycle intentionally did not use them — they remain available for a
    // future small/mixed subset.
    let squid = find("Squid");
    assert_eq!(squid.challenge_rating.as_f32(), 1.0, "Squid is CR 1 and was left unused this cycle");
    let troglodyte = find("Troglodyte");
    assert_eq!(troglodyte.challenge_rating.as_f32(), 1.0, "Troglodyte is CR 1 and was left unused this cycle");
}

/// Grounding test for subset 04's roster (see
/// `docs/release/SD-22/artifacts/beastiary1/subset_04_cycle_receipt.md`).
/// Subset 04 continues CR 2, alphabetically after subset 03's "Cave
/// Fisher". No parser widening was needed — every field these five
/// monsters use already falls inside `monster_stat_block`'s existing
/// recognition surface, including Morlock's pipe-separated two-attack
/// `NATURALATTACKS:` token — but this test proves that against the real
/// corpus file directly, the same way subsets 01-03's grounding tests
/// did.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_subset_04_cr_2_monster_records_from_b1_races_lst() {
    let text = real_b1_races_lst();
    let records = parse_monster_stat_block_entries("b1_races.lst", &text);

    let find = |name: &str| {
        records
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("expected a parsed record named {name:?} from the real b1_races.lst"))
    };

    let choker = find("Choker");
    assert_eq!(choker.challenge_rating.as_f32(), 2.0);
    assert_eq!(choker.size.as_deref(), Some("S"));
    assert_eq!(choker.speed_ft, Some(20), "walk speed, not the climb speed");
    assert_eq!(choker.race_type.as_deref(), Some("Aberration"));
    assert_eq!(choker.source_page.as_deref(), Some("p.45"));
    assert!(choker.natural_attacks.is_empty(), "Tentacle is an ABILITY:Internal cross-reference, not a NATURALATTACKS: token");

    let crocodile = find("Crocodile");
    assert_eq!(crocodile.challenge_rating.as_f32(), 2.0);
    assert_eq!(crocodile.size.as_deref(), Some("L"));
    assert_eq!(crocodile.speed_ft, Some(20), "walk speed, not the swim speed");
    assert_eq!(crocodile.race_type.as_deref(), Some("Animal"));
    assert_eq!(crocodile.source_page.as_deref(), Some("p.51"));
    assert!(crocodile.natural_attacks.is_empty());

    let dark_creeper = find("Dark Creeper");
    assert_eq!(dark_creeper.challenge_rating.as_f32(), 2.0);
    assert_eq!(dark_creeper.speed_ft, Some(30));
    assert_eq!(dark_creeper.race_type.as_deref(), Some("Humanoid"));
    assert_eq!(dark_creeper.race_subtype.as_deref(), Some("Dark Folk"));
    assert_eq!(dark_creeper.source_page.as_deref(), Some("p.53"));
    assert!(dark_creeper.natural_attacks.is_empty());

    let iron_cobra = find("Iron Cobra");
    assert_eq!(iron_cobra.challenge_rating.as_f32(), 2.0);
    assert_eq!(iron_cobra.speed_ft, Some(40));
    assert_eq!(iron_cobra.race_type.as_deref(), Some("Construct"));
    assert_eq!(iron_cobra.source_page.as_deref(), Some("p.182"));
    assert!(iron_cobra.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d6"));

    let morlock = find("Morlock");
    assert_eq!(morlock.challenge_rating.as_f32(), 2.0);
    assert_eq!(morlock.speed_ft, Some(40), "walk speed, not the climb speed");
    assert_eq!(morlock.race_type.as_deref(), Some("Monstrous Humanoid"));
    assert_eq!(morlock.source_page.as_deref(), Some("p.209"));
    assert_eq!(morlock.natural_attacks.len(), 2, "pipe-separated two-attack NATURALATTACKS: token");
    assert!(morlock.natural_attacks.iter().any(|a| a.name == "Bite (Primary)" && a.damage_dice == "1d4"));
    assert!(morlock.natural_attacks.iter().any(|a| a.name == "Bite (With Weapon Attack)" && a.damage_dice == "1d4"));
}

/// Grounding test for subset 05's roster (see
/// `docs/release/SD-22/artifacts/beastiary1/subset_05_cycle_receipt.md`).
/// Subset 05 continues CR 2, alphabetically after subset 04's "Morlock".
/// No parser widening was needed. This subset introduces two new shapes
/// not previously exercised: Shark's row has no `Walk` pair in its
/// `MOVE:` token at all (`speed_ft` parses to `None`), and Sahuagin/Skum
/// each carry two separate `NATURALATTACKS:` tab fields that accumulate
/// into one combined list.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_subset_05_cr_2_monster_records_from_b1_races_lst() {
    let text = real_b1_races_lst();
    let records = parse_monster_stat_block_entries("b1_races.lst", &text);

    let find = |name: &str| {
        records
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("expected a parsed record named {name:?} from the real b1_races.lst"))
    };

    let rat_swarm = find("Rat Swarm");
    assert_eq!(rat_swarm.challenge_rating.as_f32(), 2.0);
    assert_eq!(rat_swarm.size.as_deref(), Some("T"));
    assert_eq!(rat_swarm.speed_ft, Some(15), "walk speed, not the climb/swim speed");
    assert_eq!(rat_swarm.race_type.as_deref(), Some("Animal"));
    assert_eq!(rat_swarm.race_subtype.as_deref(), Some("Swarm"));
    assert_eq!(rat_swarm.source_page.as_deref(), Some("p.232"));
    assert!(rat_swarm.natural_attacks.iter().any(|a| a.name == "Swarm" && a.damage_dice == "1d6"));

    let sahuagin = find("Sahuagin");
    assert_eq!(sahuagin.challenge_rating.as_f32(), 2.0);
    assert_eq!(sahuagin.speed_ft, Some(30), "walk speed, not the swim speed");
    assert_eq!(sahuagin.race_type.as_deref(), Some("Monstrous Humanoid"));
    assert_eq!(sahuagin.race_subtype.as_deref(), Some("Aquatic"));
    assert_eq!(sahuagin.source_page.as_deref(), Some("p.239"));
    assert_eq!(
        sahuagin.natural_attacks.len(),
        3,
        "two NATURALATTACKS: tab fields (Claws, and a pipe-separated Bite pair) accumulate to 3 entries"
    );
    assert!(sahuagin.natural_attacks.iter().any(|a| a.name == "Claws" && a.damage_dice == "1d4"));
    assert!(sahuagin.natural_attacks.iter().any(|a| a.name == "Bite (w/o weapon)" && a.damage_dice == "1d4"));
    assert!(sahuagin.natural_attacks.iter().any(|a| a.name == "Bite (with weapon)" && a.damage_dice == "1d4"));

    let shark = find("Shark");
    assert_eq!(shark.challenge_rating.as_f32(), 2.0);
    assert_eq!(shark.speed_ft, None, "no Walk pair on the real row -- MOVE:Swim,60 only");
    assert_eq!(shark.race_type.as_deref(), Some("Animal"));
    assert_eq!(shark.race_subtype.as_deref(), Some("Aquatic"));
    assert_eq!(shark.source_page.as_deref(), Some("p.247"));
    assert!(shark.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d8"));

    let shocker_lizard = find("Shocker Lizard");
    assert_eq!(shocker_lizard.challenge_rating.as_f32(), 2.0);
    assert_eq!(shocker_lizard.speed_ft, Some(40), "walk speed, not the climb/swim speed");
    assert_eq!(shocker_lizard.race_type.as_deref(), Some("Magical Beast"));
    assert_eq!(shocker_lizard.race_subtype, None);
    assert_eq!(shocker_lizard.source_page.as_deref(), Some("p.248"));
    assert!(shocker_lizard.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d4"));

    let skum = find("Skum");
    assert_eq!(skum.challenge_rating.as_f32(), 2.0);
    assert_eq!(skum.speed_ft, Some(20), "walk speed, not the swim speed");
    assert_eq!(skum.race_type.as_deref(), Some("Monstrous Humanoid"));
    assert_eq!(skum.race_subtype.as_deref(), Some("Aquatic"));
    assert_eq!(skum.source_page.as_deref(), Some("p.253"));
    assert_eq!(
        skum.natural_attacks.len(),
        4,
        "two NATURALATTACKS: tab fields, each a pipe-separated pair, accumulate to 4 entries"
    );
    assert!(skum.natural_attacks.iter().any(|a| a.name == "Bite (w/o weapon)" && a.damage_dice == "1d6"));
    assert!(skum.natural_attacks.iter().any(|a| a.name == "Bite (w/weapon)" && a.damage_dice == "1d6"));
    assert!(skum.natural_attacks.iter().any(|a| a.name == "Claw (w/o weapon)" && a.damage_dice == "1d4"));
    assert!(skum.natural_attacks.iter().any(|a| a.name == "Claw (w/weapon)" && a.damage_dice == "1d4"));
}

/// Grounding test for subset 06's roster (see
/// `docs/release/SD-22/artifacts/beastiary1/subset_06_cycle_receipt.md`).
/// Subset 06 is a band-exhaustion cleanup subset: the last two unused
/// non-parenthetical CR-1 names (Squid, Troglodyte) plus the last four
/// unused non-parenthetical CR-2 names (Vargouille, Wolverine, Worg,
/// Yellow Musk Creeper). No parser widening was needed. Squid and
/// Vargouille both exercise the "no Walk pair in MOVE:" shape subset
/// 05's Shark already proved; Troglodyte exercises the "two
/// NATURALATTACKS: tab fields accumulate" shape subset 05's
/// Sahuagin/Skum already proved; Vargouille/Wolverine/Worg exercise the
/// "no NATURALATTACKS: token at all" shape subset 04's
/// Choker/Crocodile/Dark Creeper already proved.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_subset_06_cr_1_and_cr_2_monster_records_from_b1_races_lst() {
    let text = real_b1_races_lst();
    let records = parse_monster_stat_block_entries("b1_races.lst", &text);

    let find = |name: &str| {
        records
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("expected a parsed record named {name:?} from the real b1_races.lst"))
    };

    let squid = find("Squid");
    assert_eq!(squid.challenge_rating.as_f32(), 1.0);
    assert_eq!(squid.size.as_deref(), Some("M"));
    assert_eq!(squid.speed_ft, None, "no Walk pair on the real row -- MOVE:Swim,60,Jet,240 only");
    assert_eq!(squid.race_type.as_deref(), Some("Animal"));
    assert_eq!(squid.race_subtype.as_deref(), Some("Aquatic"));
    assert_eq!(squid.source_page.as_deref(), Some("p.259"));
    assert!(squid.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d3"));
    assert!(squid.natural_attacks.iter().any(|a| a.name == "Tentacles" && a.damage_dice == "1d4"));

    let troglodyte = find("Troglodyte");
    assert_eq!(troglodyte.challenge_rating.as_f32(), 1.0);
    assert_eq!(troglodyte.speed_ft, Some(30));
    assert_eq!(troglodyte.race_type.as_deref(), Some("Humanoid"));
    assert_eq!(troglodyte.race_subtype.as_deref(), Some("Reptilian"));
    assert_eq!(troglodyte.source_page.as_deref(), Some("p.267"));
    assert_eq!(
        troglodyte.natural_attacks.len(),
        4,
        "two NATURALATTACKS: tab fields, each a pipe-separated pair, accumulate to 4 entries"
    );
    assert!(troglodyte.natural_attacks.iter().any(|a| a.name == "Claw" && a.damage_dice == "1d4"));
    assert!(
        troglodyte
            .natural_attacks
            .iter()
            .any(|a| a.name == "Claw (with weapon attack)" && a.damage_dice == "1d4")
    );
    assert!(troglodyte.natural_attacks.iter().any(|a| a.name == "Bite" && a.damage_dice == "1d4"));
    assert!(
        troglodyte
            .natural_attacks
            .iter()
            .any(|a| a.name == "Bite (with weapon attack)" && a.damage_dice == "1d4")
    );

    let vargouille = find("Vargouille");
    assert_eq!(vargouille.challenge_rating.as_f32(), 2.0);
    assert_eq!(vargouille.speed_ft, None, "no Walk pair on the real row -- MOVE:Fly,30 only");
    assert_eq!(vargouille.race_type.as_deref(), Some("Outsider"));
    assert_eq!(vargouille.race_subtype.as_deref(), Some("Evil|Extraplanar"));
    assert_eq!(vargouille.source_page.as_deref(), Some("p.272"));
    assert!(vargouille.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let wolverine = find("Wolverine");
    assert_eq!(wolverine.challenge_rating.as_f32(), 2.0);
    assert_eq!(wolverine.speed_ft, Some(30), "walk speed, not the burrow/climb speed");
    assert_eq!(wolverine.race_type.as_deref(), Some("Animal"));
    assert_eq!(wolverine.race_subtype, None);
    assert_eq!(wolverine.source_page.as_deref(), Some("p.279"));
    assert!(wolverine.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let worg = find("Worg");
    assert_eq!(worg.challenge_rating.as_f32(), 2.0);
    assert_eq!(worg.speed_ft, Some(50));
    assert_eq!(worg.race_type.as_deref(), Some("Magical Beast"));
    assert_eq!(worg.race_subtype, None);
    assert_eq!(worg.source_page.as_deref(), Some("p.280"));
    assert!(worg.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let yellow_musk_creeper = find("Yellow Musk Creeper");
    assert_eq!(yellow_musk_creeper.challenge_rating.as_f32(), 2.0);
    assert_eq!(yellow_musk_creeper.speed_ft, Some(5));
    assert_eq!(yellow_musk_creeper.race_type.as_deref(), Some("Plant"));
    assert_eq!(yellow_musk_creeper.race_subtype, None);
    assert_eq!(yellow_musk_creeper.source_page.as_deref(), Some("p.285"));
    assert!(yellow_musk_creeper.natural_attacks.iter().any(|a| a.name == "Tendril" && a.damage_dice == "1d4"));
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_subset_07_cr_3_monster_records_from_b1_races_lst() {
    let text = real_b1_races_lst();
    let records = parse_monster_stat_block_entries("b1_races.lst", &text);

    let find = |name: &str| {
        records
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("expected a parsed record named {name:?} from the real b1_races.lst"))
    };

    let ankheg = find("Ankheg");
    assert_eq!(ankheg.challenge_rating.as_f32(), 3.0);
    assert_eq!(ankheg.size.as_deref(), Some("L"));
    assert_eq!(ankheg.speed_ft, Some(30), "MOVE:Walk,30,Burrow,20 -- walk speed, not burrow");
    assert_eq!(ankheg.race_type.as_deref(), Some("Magical Beast"));
    assert_eq!(ankheg.race_subtype, None);
    assert_eq!(ankheg.source_page.as_deref(), Some("p.15"));
    assert!(ankheg.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let assassin_vine = find("Assassin Vine");
    assert_eq!(assassin_vine.challenge_rating.as_f32(), 3.0);
    assert_eq!(assassin_vine.size.as_deref(), Some("L"));
    assert_eq!(assassin_vine.speed_ft, Some(5));
    assert_eq!(assassin_vine.race_type.as_deref(), Some("Plant"));
    assert_eq!(assassin_vine.race_subtype, None);
    assert_eq!(assassin_vine.source_page.as_deref(), Some("p.22"));
    assert!(assassin_vine.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let centaur = find("Centaur");
    assert_eq!(centaur.challenge_rating.as_f32(), 3.0);
    assert_eq!(centaur.size.as_deref(), Some("L"));
    assert_eq!(centaur.speed_ft, Some(50));
    assert_eq!(centaur.race_type.as_deref(), Some("Monstrous Humanoid"));
    assert_eq!(centaur.race_subtype, None);
    assert_eq!(centaur.source_page.as_deref(), Some("p.42"));
    assert!(centaur.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let cockatrice = find("Cockatrice");
    assert_eq!(cockatrice.challenge_rating.as_f32(), 3.0);
    assert_eq!(cockatrice.size.as_deref(), Some("S"));
    assert_eq!(cockatrice.speed_ft, Some(20), "MOVE:Walk,20,Fly,60 -- walk speed, not fly");
    assert_eq!(cockatrice.race_type.as_deref(), Some("Magical Beast"));
    assert_eq!(cockatrice.race_subtype, None);
    assert_eq!(cockatrice.source_page.as_deref(), Some("p.48"));
    assert!(cockatrice.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row");

    let derro = find("Derro");
    assert_eq!(derro.challenge_rating.as_f32(), 3.0);
    assert_eq!(derro.size.as_deref(), Some("S"));
    assert_eq!(derro.speed_ft, Some(20));
    assert_eq!(derro.race_type.as_deref(), Some("Humanoid"));
    assert_eq!(derro.race_subtype.as_deref(), Some("Derro"));
    assert_eq!(derro.source_page.as_deref(), Some("p.70"));
    assert!(derro.natural_attacks.is_empty(), "no NATURALATTACKS: token on the real row -- fights with weapons");
}
