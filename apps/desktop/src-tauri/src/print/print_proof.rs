//! `print_proof` (SD-40 E1.2): the look-at-your-own-output harness. With `PRINT_PROOF_OUT=<dir>`
//! set, the two Pathfinder starter seeds (Aldric, Elowen), each with a Notes text stored in the
//! proof's own copy, print through the real print path on both papers with each of the eight section
//! sets (SD-40 E4.1, `decisions.md §23.4`, §24.9: the golden four, none = 1+2+3, spellbook = 1+2+3+4,
//! math = 1+2+3+7, all = 1-7; then the Spellbook, Bio, Math and Notes each alone) and each PDF is
//! written to `<dir>/<seed>-<paper>-<set>.pdf`; every path is printed. 2 x 2 x 8 = 32.
//! Then Aldric, with seven languages stored through the bio save path (E2.16), prints the first-time
//! set on both papers as `aldric-languages7-<paper>-none.pdf`: 32 + 2 = 34.
//! With the variable unset the test writes nothing and passes.
//!
//! Run (from `apps/desktop/src-tauri`):
//!
//! ```text
//! mkdir -p "$DIR" && PRINT_PROOF_OUT="$DIR" cargo test --locked -j 8 print_proof -- --test-threads=8 --nocapture
//! bash scripts/print/rasterize.sh "$DIR" "$DIR/png"
//! ```

use std::path::PathBuf;

use super::{render_sheet_pdf_at_root, CampaignNote, Paper, PrintRequest, PrintSections};
use crate::character_hub::{load_character_bio_at_root, save_character_bio_at_root, seed_default_characters_at, CharacterBioDto};

/// Aldric's seven languages (`decisions.md §24.3.3`): stored in the proof's temp copy of Aldric to
/// show the Languages cell holds seven names; the shipped seed stores none.
pub(crate) const ALDRIC_SEVEN_LANGUAGES: [&str; 7] = ["Common", "Dwarven", "Elven", "Giant", "Gnome", "Goblin", "Orc"];

const SEEDS: [(&str, &str); 2] = [("aldric", "00000000-0000-0000-0000-000000000001"), ("elowen", "00000000-0000-0000-0000-000000000002")];

/// The first-time set (`decisions.md §23.4`): Primary, Abilities and Feats, Inventory.
const FIRST_TIME: PrintSections = PrintSections { main: true, feats: true, inventory: true, spellbook: false, bio: false, progression: false, math: false, notes: false };
const NONE: PrintSections = PrintSections { main: false, feats: false, inventory: false, ..FIRST_TIME };

/// (name, sections): the golden four, as `print_golden` builds them, then four sections alone.
const SECTION_SETS: [(&str, PrintSections); 8] = [
    ("none", FIRST_TIME),
    ("spellbook", PrintSections { spellbook: true, ..FIRST_TIME }),
    ("math", PrintSections { math: true, ..FIRST_TIME }),
    ("all", PrintSections { spellbook: true, bio: true, progression: true, math: true, ..FIRST_TIME }),
    ("spellbookOnly", PrintSections { spellbook: true, ..NONE }),
    ("bioOnly", PrintSections { bio: true, ..NONE }),
    ("mathOnly", PrintSections { math: true, ..NONE }),
    ("notesOnly", PrintSections { notes: true, ..NONE }),
];

/// The Notes tab text the proof stores in its copy of each seed (the shipped seeds store none).
const PROOF_NOTES: &str = "Remember to take Whirlwind as the next feat.\nThe DM ruled a wand of cure light wounds recharges for 500 gp and half a day of crafting.";

#[test]
fn print_proof() {
    let Some(out) = std::env::var_os("PRINT_PROOF_OUT").map(PathBuf::from) else {
        return;
    };
    std::fs::create_dir_all(&out).expect("the proof directory is creatable");
    let dir = std::env::temp_dir().join(format!("codex-print-proof-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp app-data dir is creatable");
    seed_default_characters_at(&dir, "0.0.0-test").expect("starter seeding succeeds");
    let mut written = 0;
    for (seed, id) in SEEDS {
        let root = dir.join("characters").join(id);
        let bio = CharacterBioDto { notes: PROOF_NOTES.to_owned(), ..load_character_bio_at_root(&root).expect("the seed's bio loads") };
        save_character_bio_at_root(&root, &bio).expect("the proof's Notes text saves");
        for (paper, paper_name) in [(Paper::A4, "a4"), (Paper::Letter, "letter")] {
            for (options, sections) in SECTION_SETS {
                let campaigns = if sections.bio {
                    vec![CampaignNote { name: "The Golden Road".to_owned(), description: "Proof campaign note.".to_owned() }]
                } else {
                    Vec::new()
                };
                let request = PrintRequest { paper, sections: Some(sections), campaigns };
                let case = format!("{seed}-{paper_name}-{options}");
                let rendered = render_sheet_pdf_at_root(&root, &request).unwrap_or_else(|err| panic!("{case}: renders: {err}"));
                let path = out.join(format!("{case}.pdf"));
                std::fs::write(&path, &rendered.pdf).unwrap_or_else(|err| panic!("{case}: writes {}: {err}", path.display()));
                println!("{}", path.display());
                written += 1;
            }
        }
    }
    let aldric = dir.join("characters").join(SEEDS[0].1);
    let bio = CharacterBioDto {
        languages: ALDRIC_SEVEN_LANGUAGES.map(str::to_owned).to_vec(),
        ..load_character_bio_at_root(&aldric).expect("Aldric's bio loads")
    };
    save_character_bio_at_root(&aldric, &bio).expect("the seven languages save");
    for (paper, paper_name) in [(Paper::A4, "a4"), (Paper::Letter, "letter")] {
        let request = PrintRequest { paper, sections: Some(FIRST_TIME), ..PrintRequest::default() };
        let case = format!("aldric-languages7-{paper_name}-none");
        let rendered = render_sheet_pdf_at_root(&aldric, &request).unwrap_or_else(|err| panic!("{case}: renders: {err}"));
        let path = out.join(format!("{case}.pdf"));
        std::fs::write(&path, &rendered.pdf).unwrap_or_else(|err| panic!("{case}: writes {}: {err}", path.display()));
        println!("{}", path.display());
        written += 1;
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(written, 34, "2 seeds x 2 papers x 8 section sets, then seven-language Aldric x 2 papers");
}
