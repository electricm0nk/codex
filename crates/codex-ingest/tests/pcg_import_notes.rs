//! The Notes block (SD-42 `decisions.md §7`, OPERATOR Q5): every line the import could not map
//! directly is listed once in `bio.notes`, under a sub-heading per kind, in file order.
//!
//! The synthetic `fixtures/pcg_notes.pcg` holds one line of each kind: an item with no record, a
//! `QUANTITY:0` line, a property and an eqmod kept as text (beside `DISARM`, which the flail's base
//! record holds, so it is no line, E4.21), an unknown key (twice, plus a second key),
//! a class and a race Codex lacks, a spell Codex has no record of, and a location word with no slot.
//!
//! **Does not cover:** a spell the engine's class list refuses (the same kind, written by
//! `keep_the_spells_the_engine_takes`; no synthetic line triggers it here); the app writing `bio.json`
//! (E4.1); the operator's file. A line kept as a custom feat (a feat, skill, ability, template or
//! unreadable money line Codex lacks) is its own kind since E1.MC, checked over 6 of the 7 samples.

use std::path::{Path, PathBuf};

use codex_ingest::pcg_import::catalog::Catalog;
use codex_ingest::pcg_import::map::convert;
use codex_ingest::pcg_import::wire::ImportFile;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn import(text: &str, name: &str) -> ImportFile {
    let catalog = Catalog::load(&repo_root()).expect("the checkout holds converted rules");
    convert(text, name, &catalog).expect("converts")
}

/// The heading names the file and an ISO date; the rest is returned.
fn body(notes: &str, file: &str) -> String {
    let (heading, rest) = notes.split_once('\n').unwrap_or((notes, ""));
    let date = heading
        .strip_prefix(&format!("## Imported from PCGen ({file}, "))
        .and_then(|tail| tail.strip_suffix(')'))
        .unwrap_or_else(|| panic!("heading: {heading:?}"));
    let digits: Vec<bool> = date.chars().map(|c| c.is_ascii_digit()).collect();
    assert_eq!(digits.len(), 10, "{date}");
    assert!(date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-' && digits.iter().filter(|d| **d).count() == 8, "{date}");
    rest.to_owned()
}

#[test]
fn notes_block_lists_every_unmapped_line_once() {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pcg_notes.pcg")).expect("fixture");
    let file = import(&text, "pcg_notes.pcg");
    let expected = "\n### Items with no Codex record
- Glimmer Bauble: no Codex record for Glimmer Bauble

### Properties kept as text
- Heavy Flail: CRITRANGE kept as text

### Lines with 0 copies
- Torch: 0 copies in the file

### PCGen keys with no mapping
- SOMENEWKEY: kept as a custom feat
- OTHERNEWKEY: kept as a custom feat

### Classes with no Codex record
- Gunslinger Ace 2: kept as a custom feat

### Race with no Codex record
- Gnoll Wanderer: kept as a custom feat; no racial ability adjustment applied

### Spells not kept
- Starfall Lance (Paladin): no Codex record; kept as a custom spell

### Eqmods kept as text
- Heavy Flail: EQMOD GLAMERED kept as text

### Locations read as Equipped
- Tail: a PCGen location Codex has no slot for; read as Equipped
";
    assert_eq!(body(&file.bio.notes, "pcg_notes.pcg"), expected, "{}", file.bio.notes);
}

#[test]
fn notes_block_when_everything_mapped() {
    let text = "PCGVERSION:2.0\nGAMEMODE:Pathfinder_RPG\nCHARACTERNAME:Mapped Tester\nSTAT:STR|SCORE:14\nSTAT:DEX|SCORE:12\n\
STAT:CON|SCORE:14\nSTAT:INT|SCORE:10\nSTAT:WIS|SCORE:10\nSTAT:CHA|SCORE:10\nRACE:Human\nCLASS:Paladin|LEVEL:1\n\
EQUIPNAME:Longsword|OUTPUTORDER:1|COST:15|WT:4.0|QUANTITY:1.0\n";
    let file = import(text, "mapped.pcg");
    assert_eq!(body(&file.bio.notes, "mapped.pcg"), "\nEverything in the file was mapped.\n", "{}", file.bio.notes);
}

/// Since E1.6 a Starfinder file reads Starfinder records (`decisions.md §8`), so the soldier's class is
/// mapped and no longer listed. Skips loudly when the oracle is absent.
#[test]
fn notes_block_maps_sf_soldiers_class() {
    let dir = std::env::var("PCGEN_REPO_DIR")
        .ok()
        .filter(|dir| !dir.is_empty())
        .map_or_else(|| PathBuf::from(std::env::var("HOME").expect("HOME")).join("workspace/repos/pcgen"), PathBuf::from);
    let path = dir.join("code/testsuite/PCGfiles/sf_soldier.pcg");
    let Ok(bytes) = std::fs::read(&path) else {
        println!("SKIPPED: PCGen oracle absent at {} (decisions.md §22)", path.display());
        return;
    };
    let file = import(&String::from_utf8_lossy(&bytes), "sf_soldier.pcg");
    assert!(file.bio.notes.starts_with("## Imported from PCGen (sf_soldier.pcg, "), "{}", file.bio.notes);
    assert!(!file.bio.notes.contains("Classes with no Codex record") && !file.bio.notes.contains("Soldier 10"), "{}", file.bio.notes);
}

/// Q5 reads "anything the import cannot map directly": a feat, a skill, an ability, a template and a
/// money line Codex cannot read are kept as custom feats, and each is a Notes line too (E1.MC).
#[test]
fn notes_block_lists_every_line_kept_as_a_custom_feat() {
    let text = "PCGVERSION:2.0\nGAMEMODE:Pathfinder_RPG\nCHARACTERNAME:Custom Tester\nSTAT:STR|SCORE:14\nSTAT:DEX|SCORE:12\n\
STAT:CON|SCORE:14\nSTAT:INT|SCORE:10\nSTAT:WIS|SCORE:10\nSTAT:CHA|SCORE:10\nRACE:Human\nCLASS:Paladin|LEVEL:1\n\
FEAT:Moonlit Reverie\n\
SKILL:Underwater Basketweaving|OUTPUTORDER:1|CLASSBOUGHT:[CLASS:Paladin|RANKS:1.0|COST:1|CLASSSKILL:Y]\n\
ABILITY:Glimmerheart|TYPE:NORMAL|CATEGORY:Special Ability|NATURE:NORMAL|KEY:Glimmerheart\n\
TEMPLATE:Half-Shade\n\
MONEY:lots\n";
    let file = import(text, "custom_feats.pcg");
    let notes = body(&file.bio.notes, "custom_feats.pcg");
    for name in ["Moonlit Reverie", "Underwater Basketweaving", "Glimmerheart", "Half-Shade", "lots"] {
        assert!(file.custom.feats.iter().any(|feat| feat.name == name), "{name} is kept: {:?}", file.custom.feats.iter().map(|f| &f.name).collect::<Vec<_>>());
        assert!(notes.contains(&format!("- {name} (")), "{name} is a Notes line:\n{notes}");
    }
}

/// The same rule over PCGen's 7 samples: every custom feat but the Languages and Biography records
/// the import itself assembles is named in Notes. Skips loudly when the oracle is absent.
#[test]
fn notes_block_names_every_custom_feat_of_the_samples() {
    let dir = std::env::var("PCGEN_REPO_DIR")
        .ok()
        .filter(|dir| !dir.is_empty())
        .map_or_else(|| PathBuf::from(std::env::var("HOME").expect("HOME")).join("workspace/repos/pcgen"), PathBuf::from);
    let mut checked = 0;
    for sample in ["pf_Cleric", "pf_Paladin", "pf_Rogue", "pf_goldielocks", "sf_mechanic", "sf_soldier"] {
        let path = dir.join(format!("code/testsuite/PCGfiles/{sample}.pcg"));
        let Ok(bytes) = std::fs::read(&path) else {
            println!("SKIPPED: PCGen oracle absent at {} (decisions.md §22)", path.display());
            return;
        };
        let file = import(&String::from_utf8_lossy(&bytes), &format!("{sample}.pcg"));
        for feat in file.custom.feats.iter().filter(|feat| feat.name != "Languages" && feat.name != "Biography") {
            // An unknown key's lines are listed once by the key (`notes_block_lists_every_unmapped_line_once`).
            let key = feat.stats.iter().find(|stat| stat.label == "PCGen key").map(|stat| stat.value.as_str()).unwrap_or_default();
            assert!(
                file.bio.notes.contains(&format!("- {}", feat.name)) || file.bio.notes.contains(&format!("- {key}: kept as a custom feat")),
                "{sample}: {} not in Notes:\n{}",
                feat.name,
                file.bio.notes
            );
            checked += 1;
        }
    }
    println!("custom feats named in Notes: {checked}");
}
