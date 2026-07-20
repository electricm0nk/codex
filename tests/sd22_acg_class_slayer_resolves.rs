//! SD-22 Epic 4 acceptance test — ACG Slayer class chassis (criteria
//! 10-12: per-class chassis resolution and the cross-book resolution
//! invariant). Slayer is the eighth real ACG class ingested (class 8 of
//! the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter,
//! Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest — see
//! `progress.md`'s `## Open blockers` entry for the roster correction
//! that dropped the non-existent "Alchemist (ACG-side)" row and added
//! `Slayer`).
//!
//! Source grounding (per `decisions.md §5`): the real
//! `acg_classes.lst:327` `CLASS:Slayer` record carries
//! `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE`
//! (full BAB — no fractional divisor, same posture as Bloodrager/Brawler),
//! `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2`
//! (good Fortitude and Reflex, combined in one token — same shape as
//! Brawler's save token), `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3`
//! (poor Will), with `MAXLEVEL:20`, and no `SPELLSTAT:` line (non-caster —
//! belongs in `pcgen_import::lst_parser::class`'s `MARTIAL_CLASS_NAMES`
//! allowlist, not the spellcasting-class parser).
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` below
//! re-parses that exact line (real-corpus-gated on `PCGEN_CORPUS_ROOT`)
//! so the hand-transcribed chassis constants in
//! `rules_tables::acg::class_slayer` stay tied to the source record
//! rather than to memory.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::{AcgClassId, class_chassis_resolve};

#[test]
fn slayer_level_1_chassis_resolves_via_ruleset_acg() {
    let row = class_chassis_resolve(AcgClassId::Slayer, 1, RuleSetId::Acg)
        .expect("Slayer level 1 chassis should resolve via RuleSetId::Acg");
    assert_eq!(row.base_attack_bonus, 1, "level 1 full BAB is 1");
    assert_eq!(row.fort_save, 2, "level 1 good save is 1/2+2 = 2");
    assert_eq!(row.ref_save, 2, "level 1 good save is 1/2+2 = 2");
    assert_eq!(row.will_save, 0, "level 1 poor save is 1/3 = 0");
}

#[test]
fn slayer_level_20_chassis_resolves_via_ruleset_acg() {
    let row = class_chassis_resolve(AcgClassId::Slayer, 20, RuleSetId::Acg)
        .expect("Slayer level 20 chassis should resolve via RuleSetId::Acg");
    assert_eq!(row.base_attack_bonus, 20, "level 20 full BAB is 20");
    assert_eq!(row.fort_save, 12, "level 20 good save is 20/2+2 = 12");
    assert_eq!(row.ref_save, 12, "level 20 good save is 20/2+2 = 12");
    assert_eq!(row.will_save, 6, "level 20 poor save is 20/3 = 6");
}

#[test]
fn slayer_chassis_is_none_for_level_beyond_maxlevel_20() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Slayer, 21, RuleSetId::Acg),
        None,
        "the real CLASS:Slayer record's MAXLEVEL:20 bounds the table"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §2.3):
/// an ACG class chassis resolves via `RuleSetId::Acg` but must return
/// `None` for `RuleSetId::Apg` and `RuleSetId::Crb` — an ACG-only class
/// table is never a valid answer for a different book's query.
#[test]
fn slayer_chassis_returns_none_for_ruleset_crb() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Slayer, 1, RuleSetId::Crb),
        None,
        "ACG-only class chassis must not resolve under RuleSetId::Crb"
    );
}

#[test]
fn slayer_chassis_returns_none_for_ruleset_apg() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Slayer, 1, RuleSetId::Apg),
        None,
        "ACG-only class chassis must not resolve under RuleSetId::Apg"
    );
}

// Cross-class regression: Arcanist, Bloodrager, Brawler, Hunter,
// Investigator, Shaman, and Skald (the first seven ACG classes landed)
// must still resolve after Slayer is added to the same enum/match arm.
#[test]
fn prior_acg_classes_still_resolve_after_slayer_lands() {
    let arcanist = class_chassis_resolve(AcgClassId::Arcanist, 1, RuleSetId::Acg)
        .expect("Arcanist level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(arcanist.base_attack_bonus, 0, "level 1 poor/half BAB is 1/2 = 0");

    let bloodrager = class_chassis_resolve(AcgClassId::Bloodrager, 1, RuleSetId::Acg)
        .expect("Bloodrager level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(bloodrager.base_attack_bonus, 1, "level 1 full BAB is 1");

    let brawler = class_chassis_resolve(AcgClassId::Brawler, 1, RuleSetId::Acg)
        .expect("Brawler level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(brawler.base_attack_bonus, 1, "level 1 full BAB is 1");

    let hunter = class_chassis_resolve(AcgClassId::Hunter, 1, RuleSetId::Acg)
        .expect("Hunter level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(hunter.base_attack_bonus, 0, "level 1 three-quarter BAB is 1*3/4 = 0");

    let investigator = class_chassis_resolve(AcgClassId::Investigator, 1, RuleSetId::Acg)
        .expect("Investigator level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(investigator.base_attack_bonus, 0, "level 1 three-quarter BAB is 1*3/4 = 0");

    let shaman = class_chassis_resolve(AcgClassId::Shaman, 1, RuleSetId::Acg)
        .expect("Shaman level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(shaman.base_attack_bonus, 0, "level 1 three-quarter BAB is 1*3/4 = 0");

    let skald = class_chassis_resolve(AcgClassId::Skald, 1, RuleSetId::Acg)
        .expect("Skald level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(skald.base_attack_bonus, 0, "level 1 three-quarter BAB is 1*3/4 = 0");
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_acg_class_brawler_resolves.rs (the
// corpus is a separate ~700MB checkout, not part of the codex repo).
fn real_slayer_class_line() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst");
    let text = fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real acg_classes.lst at {}: {err}", source.display()));
    text.lines()
        .find(|line| line.starts_with("CLASS:Slayer") && line.contains("BASEAB"))
        .unwrap_or_else(|| panic!("expected a CLASS:Slayer BASEAB line in {}", source.display()))
        .to_string()
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn hand_transcribed_chassis_matches_the_real_lst_bonus_tokens() {
    let line = real_slayer_class_line();
    assert!(
        line.contains("BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")|TYPE=Base.REPLACE"),
        "real record's BASEAB token should be full BAB (no fractional divisor): {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Fortitude,BASE.Reflex|classlevel(\"APPLIEDAS=NONEPIC\")/2+2"),
        "real record's Fortitude/Reflex save token should be good (level/2+2): {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Will|classlevel(\"APPLIEDAS=NONEPIC\")/3"),
        "real record's Will save token should be poor (level/3): {line}"
    );
    assert!(line.contains("MAXLEVEL:20"), "real record should cap at level 20: {line}");
    assert!(
        !line.contains("SPELLSTAT"),
        "Slayer should be a non-caster (no SPELLSTAT token): {line}"
    );
}
