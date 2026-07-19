//! SD-22 Epic 4 acceptance test — ACG Arcanist class chassis (criteria
//! 10-12: `rules_tables/acg/mod.rs` populated with the `RuleSetId::Acg`
//! variant registration, per-class chassis resolution, and the
//! cross-book resolution invariant). Arcanist is the first real ACG
//! class ingested (`corpus-source-inventory.md §2.1`'s row 1,
//! "Alchemist (ACG-side)", names a class with no real `CLASS:` record
//! anywhere in `acg_classes.lst` — confirmed by direct grep of the real
//! corpus; see `progress.md`'s `## Open blockers` entry for this cycle.
//! Arcanist is the first row with a real record).
//!
//! Source grounding (per `decisions.md §5`): the real
//! `acg_classes.lst:11` `CLASS:Arcanist` record carries
//! `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")/2` (poor/half
//! BAB — same shape as APG's Witch), `BONUS:SAVE|BASE.Will|classlevel(...)/2+2`
//! (good Will), and `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/3`
//! (poor Fortitude and Reflex), with `MAXLEVEL:20`.
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` below
//! re-parses that exact line (real-corpus-gated on `PCGEN_CORPUS_ROOT`)
//! so the hand-transcribed chassis constants in
//! `rules_tables::acg::class_arcanist` stay tied to the source record
//! rather than to memory.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::{AcgClassId, class_chassis_resolve};

#[test]
fn arcanist_level_1_chassis_resolves_via_ruleset_acg() {
    let row = class_chassis_resolve(AcgClassId::Arcanist, 1, RuleSetId::Acg)
        .expect("Arcanist level 1 chassis should resolve via RuleSetId::Acg");
    assert_eq!(row.base_attack_bonus, 0, "level 1 poor/half BAB is 1/2 = 0");
    assert_eq!(row.fort_save, 0, "level 1 poor save is 1/3 = 0");
    assert_eq!(row.ref_save, 0, "level 1 poor save is 1/3 = 0");
    assert_eq!(row.will_save, 2, "level 1 good save is 1/2+2 = 2");
}

#[test]
fn arcanist_level_20_chassis_resolves_via_ruleset_acg() {
    let row = class_chassis_resolve(AcgClassId::Arcanist, 20, RuleSetId::Acg)
        .expect("Arcanist level 20 chassis should resolve via RuleSetId::Acg");
    assert_eq!(row.base_attack_bonus, 10, "level 20 poor/half BAB is 20/2 = 10");
    assert_eq!(row.fort_save, 6, "level 20 poor save is 20/3 = 6");
    assert_eq!(row.ref_save, 6, "level 20 poor save is 20/3 = 6");
    assert_eq!(row.will_save, 12, "level 20 good save is 20/2+2 = 12");
}

#[test]
fn arcanist_chassis_is_none_for_level_beyond_maxlevel_20() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Arcanist, 21, RuleSetId::Acg),
        None,
        "the real CLASS:Arcanist record's MAXLEVEL:20 bounds the table"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §2.3):
/// an ACG class chassis resolves via `RuleSetId::Acg` but must return
/// `None` for `RuleSetId::Apg` and `RuleSetId::Crb` — an ACG-only class
/// table is never a valid answer for a different book's query.
#[test]
fn arcanist_chassis_returns_none_for_ruleset_crb() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Arcanist, 1, RuleSetId::Crb),
        None,
        "ACG-only class chassis must not resolve under RuleSetId::Crb"
    );
}

#[test]
fn arcanist_chassis_returns_none_for_ruleset_apg() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Arcanist, 1, RuleSetId::Apg),
        None,
        "ACG-only class chassis must not resolve under RuleSetId::Apg"
    );
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_apg_class_alchemist_resolves.rs (the
// corpus is a separate ~700MB checkout, not part of the codex repo).
fn real_arcanist_class_line() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst");
    let text = fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real acg_classes.lst at {}: {err}", source.display()));
    text.lines()
        .find(|line| line.starts_with("CLASS:Arcanist") && line.contains("BASEAB"))
        .unwrap_or_else(|| panic!("expected a CLASS:Arcanist BASEAB line in {}", source.display()))
        .to_string()
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn hand_transcribed_chassis_matches_the_real_lst_bonus_tokens() {
    let line = real_arcanist_class_line();
    assert!(
        line.contains("BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")/2|"),
        "real record's BASEAB token should be poor/half BAB: {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Will|classlevel(\"APPLIEDAS=NONEPIC\")/2+2"),
        "real record's Will save token should be good (level/2+2): {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Fortitude,BASE.Reflex|classlevel(\"APPLIEDAS=NONEPIC\")/3"),
        "real record's Fort/Reflex save token should be poor (level/3): {line}"
    );
    assert!(line.contains("MAXLEVEL:20"), "real record should cap at level 20: {line}");
}
