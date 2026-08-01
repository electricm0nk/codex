//! Integrity and provenance proof for the two GE-05 pilot-case PCGen `.pcg`
//! build fixtures, now vendored **into this repository** at
//! `docs/release/GE-05-oracle-validation-and-parity-harness/artifacts/`.
//!
//! Why this file exists. Both `sd26_pilot_case_verification.rs` and
//! `v06_wizard_pilot_case_verification.rs` previously reached these fixtures
//! through a hardcoded absolute path into another machine's
//! `programs/codex/requirements/` *planning* tree, with no environment
//! override of any kind. That made two root-suite tests unrunnable
//! anywhere except one box. Per the operator ruling — *"we always
//! include artifacts needed for the build in the artifact folder for the
//! build instead of referring to an external source"* — the fixtures are now
//! committed alongside their own GE-05 receipts and resolved repo-relative.
//!
//! Path resolution follows `ge08_workbench::resolve_package_path`'s contract
//! ("repo-relative paths anchor at the codex repo root"). That helper lives
//! in the separate `codex-desktop` crate and is not reachable from a `codex`
//! root-crate integration test, so this file uses the identical anchor its
//! fallback uses — `CARGO_MANIFEST_DIR` — which is exactly the in-crate
//! pattern `sd27_advanced_race_guide_parity.rs` already established for its
//! own vendored `.pcg` fixture. One pattern, not two.
//!
//! The pinned sha256 digests below are the whole point of this file: a
//! silently swapped, truncated, or regenerated fixture changes the parity
//! results those two suites publish as evidence, so it must fail loudly here
//! rather than quietly shift a number in a parity report.

use codex::rules_core::cache_gen::apg::sha256_file;

use std::path::PathBuf;

/// Repo-relative home of the GE-05 build's own artifacts, mirroring the
/// `docs/release/<bundle>/artifacts/` convention every other bundle uses.
const GE05_ARTIFACTS_DIR: &str = "docs/release/GE-05-oracle-validation-and-parity-harness/artifacts";

const FIGHTER_PCG_FILE: &str = "pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg";
const WIZARD_PCG_FILE: &str = "pf1-crb-human-wizard-level1-v06-alpha-swarm.pcg";

/// sha256 of the vendored Fighter pilot `.pcg` as committed.
pub const FIGHTER_PCG_SHA256: &str =
    "d0c6b2a2e9c190d0be97c20caf247b96108299340331d044547d9a57bdb64f4f";

/// sha256 of the vendored Wizard pilot `.pcg` as committed.
pub const WIZARD_PCG_SHA256: &str =
    "e2bcdae8cfccecbf871f7c587d4af577a86b5466747e41b329d0e36ee777330b";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn ge05_artifact(file_name: &str) -> PathBuf {
    repo_root().join(GE05_ARTIFACTS_DIR).join(file_name)
}

fn read_fixture_or_panic(file_name: &str) -> String {
    let path = ge05_artifact(file_name);
    assert!(
        path.is_file(),
        "GE-05 build fixture must be vendored in this repo at {} -- build artifacts live in the \
         build's own artifact folder, not behind an absolute path into another machine",
        path.display()
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("cannot read {}: {err}", path.display()))
}

/// Both fixtures are present in the repo and byte-identical to what the
/// parity evidence in the GE-05 receipts was produced from.
#[test]
fn vendored_pcg_fixtures_match_their_pinned_digests() {
    for (file_name, expected_sha) in [
        (FIGHTER_PCG_FILE, FIGHTER_PCG_SHA256),
        (WIZARD_PCG_FILE, WIZARD_PCG_SHA256),
    ] {
        let path = ge05_artifact(file_name);
        assert!(
            path.is_file(),
            "GE-05 build fixture must be vendored in this repo at {}",
            path.display()
        );
        let actual = sha256_file(&path)
            .unwrap_or_else(|err| panic!("cannot hash {}: {err}", path.display()));
        assert_eq!(
            actual,
            expected_sha,
            "{file_name} was swapped, regenerated, or truncated -- the parity results the GE-05 \
             suites publish were produced from the pinned content, so re-verify and re-record \
             before changing this digest"
        );
    }
}

/// The fixtures are genuine, complete PCGen character saves for the
/// Pathfinder RPG game mode against the Core Rulebook -- not stubs, not
/// placeholders, not truncated copies. A truncated `.pcg` would still hash
/// stably once mis-pinned, so shape is checked independently of the digest.
#[test]
fn vendored_pcg_fixtures_are_complete_pcgen_character_saves() {
    for file_name in [FIGHTER_PCG_FILE, WIZARD_PCG_FILE] {
        let text = read_fixture_or_panic(file_name);

        assert!(
            text.starts_with("PCGVERSION:2.0"),
            "{file_name} must begin with the PCGen save-format header"
        );
        for marker in [
            "CAMPAIGN:Core Rulebook",
            "GAMEMODE:Pathfinder_RPG",
            "RACE:Human",
            "ALIGN:TN",
        ] {
            assert!(
                text.contains(marker),
                "{file_name} must carry the PCGen marker `{marker}`"
            );
        }
        // PCGen writes the suppressed-biography trailer last; its presence is
        // how a truncated save is told apart from a complete one.
        assert!(
            text.trim_end().ends_with("SUPPRESSBIOFIELDS:"),
            "{file_name} is truncated -- a complete PCGen save ends with the \
             SUPPRESSBIOFIELDS trailer"
        );
    }
}

/// The Fighter fixture really is the CRB Human Fighter 1 that
/// `sd26_pilot_case_verification.rs` compares against: the fixed
/// STR16/DEX14/CON14/INT10/WIS12/CHA8 loadout, Power Attack / Dodge /
/// Weapon Focus (Longsword), Climb+Intimidate+Swim at rank 1 as real
/// **class** skills, and the Longsword + Chain Shirt kit.
#[test]
fn fighter_fixture_is_the_crb_human_fighter_level1_pilot_case() {
    let text = read_fixture_or_panic(FIGHTER_PCG_FILE);

    assert!(
        text.contains("CHARACTERNAME:pf1-crb-human-fighter-level1"),
        "fighter fixture must be the pf1-crb-human-fighter-level1 pilot case"
    );
    assert!(
        text.contains("CLASS:Fighter|LEVEL:1|"),
        "fighter fixture must be a level 1 Fighter"
    );
    assert!(
        text.contains("CLASSABILITIESLEVEL:Fighter=1|HITPOINTS:10|"),
        "fighter fixture must carry the d10 level-1 Fighter hit points"
    );

    for stat in [
        "STAT:STR|SCORE:16",
        "STAT:DEX|SCORE:14",
        "STAT:CON|SCORE:14",
        "STAT:INT|SCORE:10",
        "STAT:WIS|SCORE:12",
        "STAT:CHA|SCORE:8",
    ] {
        assert!(text.contains(stat), "fighter fixture must carry `{stat}`");
    }

    for feat in ["KEY:Power Attack", "KEY:Dodge", "KEY:Weapon Focus|APPLIEDTO:Longsword"] {
        assert!(text.contains(feat), "fighter fixture must carry feat `{feat}`");
    }

    // Climb/Intimidate/Swim ARE Fighter class skills -- the Wizard fixture is
    // the deliberate CLASSSKILL:N counterpart, and mixing the two up would
    // silently change the skill-posture parity dimension.
    for skill in ["Climb", "Intimidate", "Swim"] {
        let line = text
            .lines()
            .find(|l| l.starts_with(&format!("SKILL:{skill}|")))
            .unwrap_or_else(|| panic!("fighter fixture must carry skill `{skill}`"));
        assert!(
            line.contains("CLASS:Fighter|RANKS:1.0|COST:1|CLASSSKILL:Y"),
            "`{skill}` must be a rank-1 Fighter CLASS skill, got: {line}"
        );
    }

    for gear in ["EQUIPNAME:Chain Shirt|", "EQUIPNAME:Longsword|"] {
        assert!(text.contains(gear), "fighter fixture must carry `{gear}`");
    }
}

/// The Wizard fixture really is the CRB Human Wizard 1 that
/// `v06_wizard_pilot_case_verification.rs` compares against: the same fixed
/// ability array and gear as the Fighter, but an INT-based Evoker with a
/// real prohibited-schools list, the same three skills correctly marked
/// **not** class skills, and the seeded "Light" cantrip as both Known and
/// Prepared.
#[test]
fn wizard_fixture_is_the_crb_human_wizard_level1_pilot_case() {
    let text = read_fixture_or_panic(WIZARD_PCG_FILE);

    assert!(
        text.contains("CHARACTERNAME:pf1-crb-human-wizard-level1"),
        "wizard fixture must be the pf1-crb-human-wizard-level1 pilot case"
    );

    let class_line = text
        .lines()
        .find(|l| l.starts_with("CLASS:Wizard"))
        .expect("wizard fixture must carry a Wizard class line");
    for part in [
        "SUBCLASS:Evoker",
        "LEVEL:1",
        "SPELLBASE:INT",
        "PROHIBITED:Necromancy,Transmutation",
    ] {
        assert!(
            class_line.contains(part),
            "wizard class line must carry `{part}`, got: {class_line}"
        );
    }
    assert!(
        text.contains("CLASSABILITIESLEVEL:Wizard=1|HITPOINTS:6|"),
        "wizard fixture must carry the d6 level-1 Wizard hit points"
    );

    // Same fixed ability array and gear as the Fighter -- the two pilot cases
    // deliberately differ only by class posture.
    for stat in [
        "STAT:STR|SCORE:16",
        "STAT:DEX|SCORE:14",
        "STAT:CON|SCORE:14",
        "STAT:INT|SCORE:10",
        "STAT:WIS|SCORE:12",
        "STAT:CHA|SCORE:8",
    ] {
        assert!(text.contains(stat), "wizard fixture must carry `{stat}`");
    }
    for gear in ["EQUIPNAME:Chain Shirt|", "EQUIPNAME:Longsword|"] {
        assert!(text.contains(gear), "wizard fixture must carry `{gear}`");
    }

    // Climb/Intimidate/Swim are NOT Wizard class skills; the fixture records
    // that correctly, which is what makes the skill-posture comparison honest.
    for skill in ["Climb", "Intimidate", "Swim"] {
        let line = text
            .lines()
            .find(|l| l.starts_with(&format!("SKILL:{skill}|")))
            .unwrap_or_else(|| panic!("wizard fixture must carry skill `{skill}`"));
        assert!(
            line.contains("CLASS:Wizard|RANKS:1.0|COST:1|CLASSSKILL:N"),
            "`{skill}` must be a rank-1 Wizard NON-class skill, got: {line}"
        );
    }

    // The seeded cantrip must be present in both books, or the spellbook
    // parity dimension has nothing real to compare.
    for book in ["Known Spells", "Prepared Spells"] {
        assert!(
            text.lines().any(|l| l.starts_with("SPELLNAME:Light|")
                && l.contains(&format!("BOOK:{book}|"))
                && l.contains("SPELLLEVEL:0")),
            "wizard fixture must carry the level-0 `Light` cantrip in `{book}`"
        );
    }
}

/// The two fixtures are genuinely different characters. A copy-paste slip
/// that vendored the same file twice would still satisfy most shape checks
/// above, and would make the Wizard parity proof silently re-prove Fighter.
#[test]
fn the_two_fixtures_are_not_the_same_character() {
    assert_ne!(
        FIGHTER_PCG_SHA256, WIZARD_PCG_SHA256,
        "the pinned digests must differ"
    );
    assert_ne!(
        read_fixture_or_panic(FIGHTER_PCG_FILE),
        read_fixture_or_panic(WIZARD_PCG_FILE),
        "the Fighter and Wizard fixtures must be different characters"
    );
}
