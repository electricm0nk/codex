//! SD-22 Epic 3 acceptance test — APG Summoner class chassis (criteria
//! 7-8's per-class shape for the fifth APG class in the corrected
//! 6-class ordering: per-class chassis resolution and the cross-book
//! resolution invariant).
//!
//! Source grounding (per `decisions.md §5`, corrected 2026-07-19): the
//! real `apg_classes.lst:139` `CLASS:Summoner` record carries
//! `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4` (three-quarter
//! BAB), `BONUS:SAVE|BASE.Will|classlevel(...)/2+2` (good Will), and
//! `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/3` (poor
//! Fortitude and Reflex), with `MAXLEVEL:20` — the same chassis shape as
//! Oracle, though Summoner is a spontaneous *arcane* caster
//! (`TYPE:Base.PC.SpontaneousArcane.Spontaneous`) rather than divine.
//! `real_summoner_class_line` below re-parses that exact line (via the
//! SD-17/SD-22 `lst_parser::spellcasting_class` engine, widened this
//! cycle to recognize `Summoner`, real-corpus-gated on
//! `PCGEN_CORPUS_ROOT`, mirroring
//! `tests/sd22_apg_class_oracle_resolves.rs`'s established pattern) so
//! the hand-transcribed chassis constants in
//! `rules_tables::apg::class_summoner` stay tied to the source record
//! rather than to memory.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::apg::{ApgClassId, class_chassis_resolve};

#[test]
fn summoner_level_1_chassis_resolves_via_ruleset_apg() {
    let row = class_chassis_resolve(ApgClassId::Summoner, 1, RuleSetId::Apg)
        .expect("Summoner level 1 chassis should resolve via RuleSetId::Apg");
    assert_eq!(row.base_attack_bonus, 0, "level 1 three-quarter BAB is 1*3/4 = 0");
    assert_eq!(row.fort_save, 0, "level 1 poor save is 1/3 = 0");
    assert_eq!(row.ref_save, 0, "level 1 poor save is 1/3 = 0");
    assert_eq!(row.will_save, 2, "level 1 good save is 1/2+2 = 2");
}

#[test]
fn summoner_level_20_chassis_resolves_via_ruleset_apg() {
    let row = class_chassis_resolve(ApgClassId::Summoner, 20, RuleSetId::Apg)
        .expect("Summoner level 20 chassis should resolve via RuleSetId::Apg");
    assert_eq!(row.base_attack_bonus, 15, "level 20 three-quarter BAB is 20*3/4 = 15");
    assert_eq!(row.fort_save, 6, "level 20 poor save is 20/3 = 6");
    assert_eq!(row.ref_save, 6, "level 20 poor save is 20/3 = 6");
    assert_eq!(row.will_save, 12, "level 20 good save is 20/2+2 = 12");
}

#[test]
fn summoner_chassis_is_none_for_level_beyond_maxlevel_20() {
    assert_eq!(
        class_chassis_resolve(ApgClassId::Summoner, 21, RuleSetId::Apg),
        None,
        "the real CLASS:Summoner record's MAXLEVEL:20 bounds the table"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §1.3):
/// an APG class chassis resolves via `RuleSetId::Apg` but must return
/// `None` for `RuleSetId::Crb` — an APG-only class table is never a
/// valid answer for a CRB query.
#[test]
fn summoner_chassis_returns_none_for_ruleset_crb() {
    assert_eq!(
        class_chassis_resolve(ApgClassId::Summoner, 1, RuleSetId::Crb),
        None,
        "APG-only class chassis must not resolve under RuleSetId::Crb"
    );
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_apg_class_oracle_resolves.rs (the
// corpus is a separate ~700MB checkout, not part of the codex repo).
fn real_summoner_class_line() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst");
    let text = fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real apg_classes.lst at {}: {err}", source.display()));
    text.lines()
        .find(|line| line.starts_with("CLASS:Summoner") && line.contains("BASEAB"))
        .unwrap_or_else(|| panic!("expected a CLASS:Summoner BASEAB line in {}", source.display()))
        .to_string()
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn hand_transcribed_chassis_matches_the_real_lst_bonus_tokens() {
    let line = real_summoner_class_line();
    assert!(
        line.contains("BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")*3/4"),
        "real record's BASEAB token should be three-quarter BAB: {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Will|classlevel(\"APPLIEDAS=NONEPIC\")/2+2"),
        "real record's Will save token should be good (level/2+2): {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Fortitude,BASE.Reflex|classlevel(\"APPLIEDAS=NONEPIC\")/3"),
        "real record's Fortitude/Reflex save token should be poor (level/3): {line}"
    );
    assert!(line.contains("MAXLEVEL:20"), "real record should cap at level 20: {line}");
}
