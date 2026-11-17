//! The Notes block (SD-42 `decisions.md §7`, OPERATOR Q5): every line the import could not map
//! directly is listed once in `bio.notes`, under a sub-heading per kind, in file order.
//!
//! The synthetic `fixtures/pcg_notes.pcg` holds one line of each kind: an item with no record, a
//! `QUANTITY:0` line, a property and an eqmod kept as text, an unknown key (twice, plus a second key),
//! a class and a race Codex lacks, a spell Codex has no record of, and a location word with no slot.
//!
//! **Does not cover:** a spell the engine's class list refuses (the same kind, written by
//! `keep_the_spells_the_engine_takes`; no synthetic line triggers it here); a feat, skill or ability
//! Codex lacks (kept as custom feats, not a kind the card names); the app writing `bio.json` (E4.1);
//! the operator's file and the 7 samples other than `sf_soldier`.

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
- Heavy Flail: EQMOD DISARM kept as text

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
