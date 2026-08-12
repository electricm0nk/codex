//! SD-22 Epic 3 acceptance test — APG Inquisitor class chassis (criterion
//! 7-8's per-class shape for the fourth APG class in ordering: per-class
//! chassis resolution and the cross-book resolution invariant).
//!
//! Source grounding (per `decisions.md §5`, corrected 2026-07-19): the
//! real `apg_classes.lst:50` `CLASS:Inquisitor` record carries
//! `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4` (three-quarter
//! BAB), `BONUS:SAVE|BASE.Fortitude,BASE.Will|classlevel(...)/2+2` (good
//! Fortitude and Will), and `BONUS:SAVE|BASE.Reflex|classlevel(...)/3`
//! (poor Reflex), with `MAXLEVEL:20`. `real_inquisitor_class_line` below
//! re-parses that exact line (via the SD-17/SD-22
//! `lst_parser::spellcasting_class` engine, widened this cycle to
//! recognize `Inquisitor`, real-corpus-gated on `PCGEN_CORPUS_ROOT`,
//! mirroring `tests/sd22_apg_class_alchemist_resolves.rs`'s established
//! pattern) so the hand-transcribed chassis constants in
//! `rules_tables::apg::class_inquisitor` stay tied to the source record
//! rather than to memory.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::apg::{ApgClassId, class_chassis_resolve};

#[test]
fn inquisitor_level_1_chassis_resolves_via_ruleset_apg() {
    let row = class_chassis_resolve(ApgClassId::Inquisitor, 1, RuleSetId::Apg)
        .expect("Inquisitor level 1 chassis should resolve via RuleSetId::Apg");
    assert_eq!(row.base_attack_bonus, 0, "level 1 three-quarter BAB is 1*3/4 = 0");
    assert_eq!(row.fort_save, 2, "level 1 good save is 1/2+2 = 2");
    assert_eq!(row.ref_save, 0, "level 1 poor save is 1/3 = 0");
    assert_eq!(row.will_save, 2, "level 1 good save is 1/2+2 = 2");
}

#[test]
fn inquisitor_level_20_chassis_resolves_via_ruleset_apg() {
    let row = class_chassis_resolve(ApgClassId::Inquisitor, 20, RuleSetId::Apg)
        .expect("Inquisitor level 20 chassis should resolve via RuleSetId::Apg");
    assert_eq!(row.base_attack_bonus, 15, "level 20 three-quarter BAB is 20*3/4 = 15");
    assert_eq!(row.fort_save, 12, "level 20 good save is 20/2+2 = 12");
    assert_eq!(row.ref_save, 6, "level 20 poor save is 20/3 = 6");
    assert_eq!(row.will_save, 12, "level 20 good save is 20/2+2 = 12");
}

#[test]
fn inquisitor_chassis_is_none_for_level_beyond_maxlevel_20() {
    assert_eq!(
        class_chassis_resolve(ApgClassId::Inquisitor, 21, RuleSetId::Apg),
        None,
        "the real CLASS:Inquisitor record's MAXLEVEL:20 bounds the table"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §1.3):
/// an APG class chassis resolves via `RuleSetId::Apg` but must return
/// `None` for `RuleSetId::Crb` — an APG-only class table is never a
/// valid answer for a CRB query.
#[test]
fn inquisitor_chassis_returns_none_for_ruleset_crb() {
    assert_eq!(
        class_chassis_resolve(ApgClassId::Inquisitor, 1, RuleSetId::Crb),
        None,
        "APG-only class chassis must not resolve under RuleSetId::Crb"
    );
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_apg_class_alchemist_resolves.rs (the
// corpus is a separate ~700MB checkout, not part of the codex repo).
fn real_inquisitor_class_line() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst");
    let text = fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real apg_classes.lst at {}: {err}", source.display()));
    text.lines()
        .find(|line| line.starts_with("CLASS:Inquisitor") && line.contains("BASEAB"))
        .unwrap_or_else(|| panic!("expected a CLASS:Inquisitor BASEAB line in {}", source.display()))
        .to_string()
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn hand_transcribed_chassis_matches_the_real_lst_bonus_tokens() {
    let line = real_inquisitor_class_line();
    assert!(
        line.contains("BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")*3/4"),
        "real record's BASEAB token should be three-quarter BAB: {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Fortitude,BASE.Will|classlevel(\"APPLIEDAS=NONEPIC\")/2+2"),
        "real record's Fortitude/Will save token should be good (level/2+2): {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Reflex|classlevel(\"APPLIEDAS=NONEPIC\")/3"),
        "real record's Reflex save token should be poor (level/3): {line}"
    );
    assert!(line.contains("MAXLEVEL:20"), "real record should cap at level 20: {line}");
}
