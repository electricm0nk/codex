//! SD-22 Epic 4 acceptance test — ACG Warpriest class chassis (criteria
//! 10-12: per-class chassis resolution and the cross-book resolution
//! invariant). Warpriest is the tenth and last real ACG class ingested
//! (class 10 of the corrected 10-class roster: Arcanist, Bloodrager,
//! Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler,
//! Warpriest — see `progress.md`'s `## Open blockers` entry for the
//! roster correction that dropped the non-existent "Alchemist (ACG-side)"
//! row and added `Slayer`). With Warpriest landed, Epic 4's class-roster
//! criteria (10-12) are complete for the full 10-class roster.
//!
//! Source grounding (per `decisions.md §5`): the real
//! `acg_classes.lst:364` `CLASS:Warpriest` record carries
//! `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4|TYPE=Base.REPLACE`
//! (three-quarter BAB — same posture as ACG's Hunter/Investigator/Shaman
//! and APG's Alchemist/Inquisitor/Oracle/Summoner),
//! `BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2`
//! (good Fortitude, its own single-save token),
//! `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2`
//! (good Will, its own single-save token),
//! `BONUS:SAVE|BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3`
//! (poor Reflex, its own single-save token — the class's only poor save,
//! the classic divine-caster Cleric-shaped save spread), with
//! `MAXLEVEL:20`, `EXCLASS:Ex-Warpriest`, and (a separate `CLASS:Warpriest`
//! line further down the block) `SPELLSTAT:WIS` with no `MEMORIZE:NO` and
//! no `SPELLBOOK:YES` (standard-prepared casting, `SPELLLIST:1|Cleric` —
//! belongs in `pcgen_import::lst_parser::spellcasting_class`'s
//! `SPELLCASTING_CLASS_NAMES` allowlist, not `class.rs`'s
//! `MARTIAL_CLASS_NAMES`). The corpus also carries a separate
//! `CLASS:Ex-Warpriest` record (`VISIBLE:NO`, no `EXCLASS:` token, no
//! `SPELLSTAT:` token) — an internal fallen-Warpriest NPC variant, not
//! player-facing content; this cycle deliberately chassis'd only the
//! real, player-facing `Warpriest` class.
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` below
//! re-parses that exact line (real-corpus-gated on `PCGEN_CORPUS_ROOT`)
//! so the hand-transcribed chassis constants in
//! `rules_tables::acg::class_warpriest` stay tied to the source record
//! rather than to memory.

use std::fs;
use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::{AcgClassId, class_chassis_resolve};

#[test]
fn warpriest_level_1_chassis_resolves_via_ruleset_acg() {
    let row = class_chassis_resolve(AcgClassId::Warpriest, 1, RuleSetId::Acg)
        .expect("Warpriest level 1 chassis should resolve via RuleSetId::Acg");
    assert_eq!(row.base_attack_bonus, 0, "level 1 three-quarter BAB is 1*3/4 = 0");
    assert_eq!(row.fort_save, 2, "level 1 good save is 1/2+2 = 2");
    assert_eq!(row.ref_save, 0, "level 1 poor save is 1/3 = 0");
    assert_eq!(row.will_save, 2, "level 1 good save is 1/2+2 = 2");
}

#[test]
fn warpriest_level_20_chassis_resolves_via_ruleset_acg() {
    let row = class_chassis_resolve(AcgClassId::Warpriest, 20, RuleSetId::Acg)
        .expect("Warpriest level 20 chassis should resolve via RuleSetId::Acg");
    assert_eq!(row.base_attack_bonus, 15, "level 20 three-quarter BAB is 20*3/4 = 15");
    assert_eq!(row.fort_save, 12, "level 20 good save is 20/2+2 = 12");
    assert_eq!(row.ref_save, 6, "level 20 poor save is 20/3 = 6");
    assert_eq!(row.will_save, 12, "level 20 good save is 20/2+2 = 12");
}

#[test]
fn warpriest_chassis_is_none_for_level_beyond_maxlevel_20() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Warpriest, 21, RuleSetId::Acg),
        None,
        "the real CLASS:Warpriest record's MAXLEVEL:20 bounds the table"
    );
}

/// Cross-book resolution invariant (`corpus-source-inventory.md` §2.3):
/// an ACG class chassis resolves via `RuleSetId::Acg` but must return
/// `None` for `RuleSetId::Apg` and `RuleSetId::Crb` — an ACG-only class
/// table is never a valid answer for a different book's query.
#[test]
fn warpriest_chassis_returns_none_for_ruleset_crb() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Warpriest, 1, RuleSetId::Crb),
        None,
        "ACG-only class chassis must not resolve under RuleSetId::Crb"
    );
}

#[test]
fn warpriest_chassis_returns_none_for_ruleset_apg() {
    assert_eq!(
        class_chassis_resolve(AcgClassId::Warpriest, 1, RuleSetId::Apg),
        None,
        "ACG-only class chassis must not resolve under RuleSetId::Apg"
    );
}

// Cross-class regression: Arcanist, Bloodrager, Brawler, Hunter,
// Investigator, Shaman, Skald, Slayer, and Swashbuckler (the first nine
// ACG classes landed) must still resolve after Warpriest is added to the
// same enum/match arm.
#[test]
fn prior_acg_classes_still_resolve_after_warpriest_lands() {
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

    let slayer = class_chassis_resolve(AcgClassId::Slayer, 1, RuleSetId::Acg)
        .expect("Slayer level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(slayer.base_attack_bonus, 1, "level 1 full BAB is 1");

    let swashbuckler = class_chassis_resolve(AcgClassId::Swashbuckler, 1, RuleSetId::Acg)
        .expect("Swashbuckler level 1 chassis should still resolve via RuleSetId::Acg");
    assert_eq!(swashbuckler.base_attack_bonus, 1, "level 1 full BAB is 1");
}

// Real-corpus-gated grounding test, opt-in via `PCGEN_CORPUS_ROOT` per the
// established pattern in tests/sd22_acg_class_shaman_resolves.rs (the
// corpus is a separate ~700MB checkout, not part of the codex repo).
fn real_warpriest_class_line() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst");
    let text = fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real acg_classes.lst at {}: {err}", source.display()));
    text.lines()
        .find(|line| line.starts_with("CLASS:Warpriest") && line.contains("BASEAB"))
        .unwrap_or_else(|| panic!("expected a CLASS:Warpriest BASEAB line in {}", source.display()))
        .to_string()
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn hand_transcribed_chassis_matches_the_real_lst_bonus_tokens() {
    let line = real_warpriest_class_line();
    assert!(
        line.contains("BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")*3/4"),
        "real record's BASEAB token should be three-quarter BAB: {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Fortitude|classlevel(\"APPLIEDAS=NONEPIC\")/2+2"),
        "real record's Fortitude save token should be good (level/2+2): {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Will|classlevel(\"APPLIEDAS=NONEPIC\")/2+2"),
        "real record's Will save token should be good (level/2+2): {line}"
    );
    assert!(
        line.contains("SAVE|BASE.Reflex|classlevel(\"APPLIEDAS=NONEPIC\")/3"),
        "real record's Reflex save token should be poor (level/3): {line}"
    );
    assert!(line.contains("MAXLEVEL:20"), "real record should cap at level 20: {line}");
}

// Distinguishes the real player-facing CLASS:Warpriest record from the
// corpus's separate internal CLASS:Ex-Warpriest (VISIBLE:NO) record —
// this cycle deliberately did not chassis the ex-class NPC variant.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn ex_warpriest_variant_is_a_distinct_internal_record_not_chassis_d_here() {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst");
    let text = fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real acg_classes.lst at {}: {err}", source.display()));
    let ex_line = text
        .lines()
        .find(|line| line.starts_with("CLASS:Ex-Warpriest") && line.contains("BASEAB"))
        .unwrap_or_else(|| panic!("expected a CLASS:Ex-Warpriest BASEAB line in {}", source.display()));
    assert!(
        ex_line.contains("VISIBLE:NO"),
        "CLASS:Ex-Warpriest should be marked internal (VISIBLE:NO): {ex_line}"
    );
}
