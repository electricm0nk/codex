//! v0.6 Receipt-to-Sheet slice 1, item 4: `class_chassis.<class>.caster_level`
//! for every class that casts (`docs/release/v0.6/execution-engine-scoping.md`
//! §3 and §7).
//!
//! **Why this record is the highest-leverage number missing from the sheet.**
//! The operator's dice ruling (scoping doc §4) is that the product computes how
//! many dice a character may roll, never the roll. For class features that is
//! already satisfied. For *spells* it is blocked by the corpus, not the engine:
//! PCGen's spell schema carries no damage token at all, so `10d6` would have to
//! be hand-authored 121-347 times. But the Spells tab already renders each
//! spell's full description, so *"1d6 per caster level (maximum 10d6)"* is
//! already on the player's screen. The one number that makes it usable is the
//! caster level.
//!
//! **Corpus derivation.** Every value here is transcribed from the class's own
//! `BONUS:CASTERLEVEL` token and the `BONUS:VAR` chain that token names --
//! never from memory, and never from the class's spell-progression table shape.
//! That distinction is load-bearing: Bloodrager's spell progression has exactly
//! the Paladin/Ranger shape (first spells at class level 4), which invites the
//! assumption that its caster level is likewise `level - 3`. The corpus says
//! otherwise -- `acg_classes.lst:40` defines
//! `BONUS:VAR|Caster_Level_Bloodrager|BloodragerLVL+Caster_Level_Bonus+CasterLevelBLBloodrager`
//! with `BONUS:VAR|BloodragerLVL|CL`, and `acg_classes.lst:44` applies it as
//! `BONUS:CASTERLEVEL|Bloodrager|Caster_Level_Bloodrager|PRECLASS:1,Bloodrager=4`.
//! A Bloodrager's caster level is his **full** class level; only the gate is
//! delayed. Paladin (`cr_classes.lst:164`) and Ranger (`cr_classes.lst:206`)
//! are the ones carrying the literal `-3`
//! (`BONUS:VAR|Caster_Level_Paladin|CL+Caster_Level_Bonus-3+CasterLevelBLPaladin`).
//!
//! **Scope.** Paladin and Ranger are deliberately NOT given this new id: they
//! already ground the identical arithmetic under
//! `class_chassis.<class>.partial_caster.effective_caster_level`, and a second
//! record carrying the same number under a second id is exactly the drift
//! hazard this repo's no-stub doctrine forbids. This test pins both halves of
//! that contract -- the two partial casters keep their existing id and do not
//! gain the new one.
//!
//! This grounds the caster level only. No spells known, no spells per day, no
//! bonus spell slots, no spell save DCs and no per-spell dice are computed by
//! it.

use codex::rules_core::character_input::{
    CharacterClassLevel, CharacterInput, load_character_input_fixture,
};
use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};

const FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// PF1's own class ceiling, and the `MAXLEVEL:20` every CRB/APG/ACG class
/// record in the corpus declares.
const MAX_LEVEL: u8 = 20;

/// The seventeen classes whose corpus `BONUS:CASTERLEVEL` token resolves to a
/// caster level equal to the class level, paired with the class level at which
/// the corpus's own gate on that token opens (`1` where the token carries no
/// `PRECLASS:` gate at all).
///
/// Transcribed one class at a time from the `BONUS:CASTERLEVEL` token and the
/// `BONUS:VAR` chain it names. `Caster_Level_Bonus` is `DEFINE`d to 0 in
/// `core_essentials/ce_abilities.lst:11` and is only ever raised by an opt-in
/// item (the Orange Prism Ioun Stone) or the APG Magical Knack trait
/// (`apg_abilities.lst:82-103`), neither of which is base-class content, so it
/// contributes 0 here. Each `CasterLevelBL<Class>` term is likewise `DEFINE`d
/// to 0 and raised only by bloodline/archetype records.
const EXPECTED: &[(&str, u8)] = &[
    // --- CRB (`core_rulebook/cr_classes.lst`) ---
    // :24/:28 BONUS:VAR|Caster_Level_Bard|CL+Caster_Level_Bonus+CasterLevelBLBard
    ("bard", 1),
    // :55/:59 BONUS:VAR|Caster_Level_Cleric|CL+Caster_Level_Bonus+CasterLevelBLCleric
    ("cleric", 1),
    // :93/:99 BONUS:VAR|Caster_Level_Druid|CL+Caster_Level_Bonus+CasterLevelBLDruid
    ("druid", 1),
    // :246/:250 BONUS:VAR|Caster_Level_Sorcerer|CL+Caster_Level_Bonus+CasterLevelBLSorcerer
    ("sorcerer", 1),
    // :277/:281 BONUS:VAR|Caster_Level_Wizard|WizardLVL+... with BONUS:VAR|WizardLVL|CL
    ("wizard", 1),
    // --- APG (`advanced_players_guide/apg_classes.lst`) ---
    // :11/:15 BONUS:VAR|Caster_Level_Alchemist|CL+Caster_Level_Bonus+CasterLevelBLAlchemist
    ("alchemist", 1),
    // :50/:56 BONUS:VAR|Caster_Level_Inquisitor|CL+...
    ("inquisitor", 1),
    // :107/:111 BONUS:VAR|Caster_Level_Oracle|CL+...
    ("oracle", 1),
    // :139/:145 BONUS:VAR|Caster_Level_Summoner|CL+...
    ("summoner", 1),
    // :172/:176 BONUS:VAR|Caster_Level_Witch|CL+...
    ("witch", 1),
    // --- ACG (`advanced_class_guide/acg_classes.lst`) ---
    // :15 BONUS:CASTERLEVEL|Arcanist|CL -- the token names `CL` directly
    ("arcanist", 1),
    // :40/:44 the one gated class: PRECLASS:1,Bloodrager=4, full class level above it
    ("bloodrager", 4),
    // :114 BONUS:CASTERLEVEL|Hunter|CL
    ("hunter", 1),
    // :168/:172 BONUS:VAR|Caster_Level_Investigator|InvestigatorLVL+...
    ("investigator", 1),
    // :225 BONUS:CASTERLEVEL|Shaman|CL
    ("shaman", 1),
    // :274/:278 BONUS:VAR|Caster_Level_Skald|SkaldLVL+... with BONUS:VAR|SkaldLVL|CL
    ("skald", 1),
    // :368 BONUS:CASTERLEVEL|Warpriest|CL (and |Cleric|CL, the borrowed list)
    ("warpriest", 1),
];

/// The eight classes in the 27-class roster that cast nothing at all. None of
/// them may gain a caster-level record.
const NON_CASTERS: &[&str] = &[
    "barbarian",
    "fighter",
    "monk",
    "rogue",
    "cavalier",
    "brawler",
    "slayer",
    "swashbuckler",
];

fn fixture() -> CharacterInput {
    let result = load_character_input_fixture(FIXTURE);
    assert!(
        result.diagnostics.is_empty(),
        "shared fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn compute(class_name: &str, level: u8) -> PilotBaseChassisComputation {
    let mut input = fixture();
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_name}"),
        level,
    }];
    compute_pilot_base_chassis(&input)
}

fn caster_level_id(class_name: &str) -> String {
    format!("class_chassis.{class_name}.caster_level")
}

fn value_of(computation: &PilotBaseChassisComputation, id: &str) -> i16 {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("expected explanation id '{id}' to be grounded"))
        .value
}

fn has(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

/// The headline: caster level equals class level, at every level 1-20, for
/// every ungated casting class.
#[test]
fn every_casting_class_grounds_its_caster_level_at_every_level() {
    for (class_name, first_casting_level) in EXPECTED {
        let id = caster_level_id(class_name);
        for level in 1..=MAX_LEVEL {
            let computation = compute(class_name, level);
            let expected = if level >= *first_casting_level {
                i16::from(level)
            } else {
                0
            };
            assert_eq!(
                value_of(&computation, &id),
                expected,
                "{class_name} caster level at class level {level}"
            );
        }
    }
}

/// The three levels the brief pins explicitly, spelled out as literals rather
/// than recomputed from the same formula the production code uses -- a loop
/// that re-derives its own expectation cannot catch a wrong formula.
#[test]
fn caster_level_is_pinned_by_literal_at_levels_1_10_and_20() {
    for (class_name, _) in EXPECTED {
        if *class_name == "bloodrager" {
            continue; // gated; pinned by its own test below
        }
        let id = caster_level_id(class_name);
        assert_eq!(value_of(&compute(class_name, 1), &id), 1, "{class_name} @1");
        assert_eq!(
            value_of(&compute(class_name, 10), &id),
            10,
            "{class_name} @10"
        );
        assert_eq!(
            value_of(&compute(class_name, 20), &id),
            20,
            "{class_name} @20"
        );
    }
}

/// The corpus trap: Bloodrager's spell progression has the Paladin/Ranger
/// shape, but its caster level is the FULL class level once the gate opens --
/// `BONUS:CASTERLEVEL|Bloodrager|Caster_Level_Bloodrager|PRECLASS:1,Bloodrager=4`
/// with `Caster_Level_Bloodrager` resolving to `BloodragerLVL` (= `CL`).
/// A `level - 3` reading would give 1/7/17 at levels 4/10/20.
#[test]
fn bloodrager_caster_level_is_the_full_class_level_not_level_minus_three() {
    let id = caster_level_id("bloodrager");

    for level in 1..4 {
        assert_eq!(
            value_of(&compute("bloodrager", level), &id),
            0,
            "bloodrager below the PRECLASS:1,Bloodrager=4 gate must ground a correct 0"
        );
    }

    assert_eq!(value_of(&compute("bloodrager", 4), &id), 4);
    assert_eq!(value_of(&compute("bloodrager", 10), &id), 10);
    assert_eq!(value_of(&compute("bloodrager", 20), &id), 20);
}

/// Paladin and Ranger keep the partial-caster id they already ground, and do
/// not gain a second record carrying the same number under the new id.
#[test]
fn the_two_partial_casters_keep_their_existing_id_and_do_not_gain_the_new_one() {
    for (class_name, expected_at_10) in [("paladin", 7_i16), ("ranger", 7)] {
        let computation = compute(class_name, 10);
        assert!(
            !has(&computation, &caster_level_id(class_name)),
            "{class_name} must not gain a duplicate caster-level record"
        );
        let existing =
            format!("class_chassis.{class_name}.partial_caster.effective_caster_level");
        assert_eq!(
            value_of(&computation, &existing),
            expected_at_10,
            "{class_name} keeps its corpus-verified level-3 partial-caster record"
        );
    }
}

/// Nothing that does not cast gains a caster level.
#[test]
fn non_casting_classes_ground_no_caster_level_at_any_level() {
    for class_name in NON_CASTERS {
        for level in [1, 10, 20] {
            let computation = compute(class_name, level);
            assert!(
                !has(&computation, &caster_level_id(class_name)),
                "{class_name} casts nothing and must ground no caster level (level {level})"
            );
        }
    }
}

/// The record must carry its own corpus derivation, not a bare number, and must
/// not overclaim: grounding a caster level fabricates no spell math.
#[test]
fn the_record_cites_its_corpus_source_and_disclaims_the_spell_math_it_does_not_compute() {
    let computation = compute("wizard", 10);
    let record = computation
        .explanations
        .iter()
        .find(|e| e.id == caster_level_id("wizard"))
        .expect("wizard caster level must be grounded");

    assert!(
        record.detail.contains("BONUS:CASTERLEVEL"),
        "record must cite the corpus token it is transcribed from: {}",
        record.detail
    );
    assert!(
        record.detail.contains("cr_classes.lst"),
        "record must cite the corpus file: {}",
        record.detail
    );
    for disclaimed in ["spells per day", "spell save DC"] {
        assert!(
            record.detail.contains(disclaimed),
            "record must disclaim '{disclaimed}': {}",
            record.detail
        );
    }
}

/// A multiclass mix grounds NO caster level, matching the convention every
/// other per-class slice in this engine already follows.
///
/// This is the opposite of what the first draft of this slice did, and the
/// change was forced by evidence rather than taste: emitting a per-class caster
/// level into a mix broke 71 pre-existing
/// `multiclass_<class>_level<N>_is_not_promoted_by_this_slice` controls across
/// the Bard/Cleric/Druid/Sorcerer/Wizard families, each of which asserts that a
/// mix surfaces no `class_chassis.<class>.*` id at all.
/// `compute_multiclass_base_chassis` discards its per-class sub-computations'
/// explanations for the same reason. A multiclass caster level is a real thing
/// a player wants; it belongs to whichever slice promotes the multiclass
/// class-chassis surface as a whole.
#[test]
fn a_multiclass_mix_grounds_no_caster_level_at_all() {
    let mut input = fixture();
    input.chosen.class_levels = vec![
        CharacterClassLevel {
            class_id: "class:wizard".to_owned(),
            level: 6,
        },
        CharacterClassLevel {
            class_id: "class:cleric".to_owned(),
            level: 3,
        },
    ];
    let computation = compute_pilot_base_chassis(&input);

    for class_name in ["wizard", "cleric", "fighter"] {
        assert!(
            !has(&computation, &caster_level_id(class_name)),
            "a multiclass mix must surface no class_chassis.{class_name}.caster_level"
        );
    }
}

/// PF1 has no 21st character level, and every class record in the corpus
/// declares `MAXLEVEL:20`. Each class already pins a
/// `<class>_level_21_is_not_promoted_by_this_slice` control; an ungated loop
/// broke 27 of them. Nothing above level 20 grounds a caster level.
#[test]
fn no_caster_level_is_grounded_above_pf1s_twentieth_level() {
    for (class_name, _) in EXPECTED {
        let id = caster_level_id(class_name);
        for level in [21, 30] {
            assert!(
                !has(&compute(class_name, level), &id),
                "{class_name} at level {level} is past PF1's MAXLEVEL:20 and must ground no \
                 caster level"
            );
        }
    }
}
