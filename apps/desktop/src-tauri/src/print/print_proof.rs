//! `print_proof` (SD-40 E1.2): the look-at-your-own-output harness. With `PRINT_PROOF_OUT=<dir>`
//! set, the two Pathfinder starter seeds (Aldric, Elowen) print through the real print path on
//! both papers with each of the four option sets (none, Spellbook, Show the math, all three) and
//! each PDF is written to `<dir>/<seed>-<paper>-<options>.pdf`; every path is printed. 2 x 2 x 4 = 16.
//! Then Aldric, with seven languages stored through the bio save path (E2.16), prints with no option
//! on both papers as `aldric-languages7-<paper>-none.pdf`: 16 + 2 = 18.
//! With the variable unset the test writes nothing and passes.
//!
//! Run (from `apps/desktop/src-tauri`):
//!
//! ```text
//! mkdir -p "$DIR" && PRINT_PROOF_OUT="$DIR" cargo test --locked -j 8 print_proof -- --test-threads=8 --nocapture
//! bash scripts/print/rasterize.sh "$DIR" "$DIR/png"
//! ```

use std::path::PathBuf;

use super::{render_sheet_pdf_at_root, CampaignNote, Paper, PrintRequest};
use crate::character_hub::{load_character_bio_at_root, save_character_bio_at_root, seed_default_characters_at, CharacterBioDto};

/// Aldric's seven languages (`decisions.md §24.3.3`): stored in the proof's temp copy of Aldric to
/// show the Languages cell holds seven names; the shipped seed stores none.
pub(crate) const ALDRIC_SEVEN_LANGUAGES: [&str; 7] = ["Common", "Dwarven", "Elven", "Giant", "Gnome", "Goblin", "Orc"];

const SEEDS: [(&str, &str); 2] = [("aldric", "00000000-0000-0000-0000-000000000001"), ("elowen", "00000000-0000-0000-0000-000000000002")];

/// (name, spellbook, math, bio): the four option sets, as `print_golden` builds them.
const OPTION_SETS: [(&str, bool, bool, bool); 4] =
    [("none", false, false, false), ("spellbook", true, false, false), ("math", false, true, false), ("all", true, true, true)];

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
        for (paper, paper_name) in [(Paper::A4, "a4"), (Paper::Letter, "letter")] {
            for (options, spellbook, math, bio) in OPTION_SETS {
                let campaigns = if bio {
                    vec![CampaignNote { name: "The Golden Road".to_owned(), description: "Proof campaign note.".to_owned() }]
                } else {
                    Vec::new()
                };
                let request = PrintRequest { paper, spellbook, math, bio, campaigns };
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
        let request = PrintRequest { paper, spellbook: false, math: false, bio: false, campaigns: Vec::new() };
        let case = format!("aldric-languages7-{paper_name}-none");
        let rendered = render_sheet_pdf_at_root(&aldric, &request).unwrap_or_else(|err| panic!("{case}: renders: {err}"));
        let path = out.join(format!("{case}.pdf"));
        std::fs::write(&path, &rendered.pdf).unwrap_or_else(|err| panic!("{case}: writes {}: {err}", path.display()));
        println!("{}", path.display());
        written += 1;
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(written, 18, "2 seeds x 2 papers x 4 option sets, then seven-language Aldric x 2 papers");
}
