//! SD-27: the prepared-spell level gate must refuse an out-of-level spell
//! from EVERY ingested book, not just the Core Rulebook.
//!
//! **The defect.** `pilot_compute::unmet_wizard_spellbook_conditions` (and
//! its Arcanist/Warpriest siblings) resolved a prepared spell's level via
//! `parse_wizard_spellbook_spell_id`, which looked the key up in
//! `crb::spell_list::SPELL_LIST` alone. An APG/ACG/ARG key is not in that
//! table and carries no dots, so the fallback `<school>.<level>.<name>`
//! parse also failed and the resolver returned `None`. Both loops that
//! consume it are `filter_map`s, so an unresolved spell was silently
//! dropped from the accessibility check AND from the slot-consumption
//! count: a Wizard 1 could prepare `Tsunami` (APG, Conjuration, Wizard
//! spell level 9) and the engine reached `Computed` without complaint.
//!
//! This is the same failure shape `wizard_spellbook_spell_id_resolution_tests`
//! records for the pre-`SPELL_LIST` era, one book later.
//!
//! **The reproduction is the eight spells the verify agent hit in the live
//! app**, all 9th-level for a Wizard and all outside CRB.

use codex::rules_core::character_input::{
    AcquisitionMode, CharacterInput, SpellSelection, load_character_input_fixture,
};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const WIZARD_LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");

/// The eight out-of-CRB spells the verify agent added to a live Wizard 1
/// and watched persist. Every one is Wizard spell level 9.
const NON_CRB_NINTH_LEVEL_WIZARD_SPELLS: &[&str] = &[
    "Tsunami",
    "Winds of Vengeance",
    "Clashing Rocks",
    "World Wave",
    "Fiery Body",
    "Suffocation (Mass)",
    "Wall of Suppression",
    "Naturalist Summon Nature's Ally IX",
];

fn wizard_at_level(level: u8) -> CharacterInput {
    let result = load_character_input_fixture(WIZARD_LEVEL_3_FIXTURE);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    let mut input = result
        .character_input
        .expect("valid fixture should produce a character input record");
    for class_level in &mut input.chosen.class_levels {
        if class_level.class_id == "class:wizard" {
            class_level.level = level;
        }
    }
    input
}

fn wizard_spell(spell_id: &str, mode: AcquisitionMode) -> SpellSelection {
    SpellSelection {
        spell_id: spell_id.to_string(),
        source_class_id: "class:wizard".to_string(),
        acquisition_mode: mode,
    }
}

/// Records and prepares `spell_id` on a Wizard of `level`, plus a real
/// 1st-level CRB spell so the "nothing recorded / nothing prepared"
/// conditions are already satisfied and the only thing under test is the
/// level gate.
fn wizard_preparing(level: u8, spell_id: &str) -> CharacterInput {
    let mut input = wizard_at_level(level);
    input.chosen.spells_selected = vec![
        wizard_spell("Magic Missile", AcquisitionMode::Known),
        wizard_spell("Magic Missile", AcquisitionMode::Prepared),
        wizard_spell(spell_id, AcquisitionMode::Known),
        wizard_spell(spell_id, AcquisitionMode::Prepared),
    ];
    input
}

/// The blocker text the spellbook posture emits when it cannot be
/// grounded. Empty when the posture grounded cleanly.
///
/// Only `claim_blocking` diagnostics count, because that is exactly the
/// flag `pf1_adapter::resolve_unified_pilot_snapshot` reads: any
/// claim-blocking diagnostic other than `combat.baseline_unsupported` /
/// `skill.selected_modifier.unsupported` makes the whole mutation come back
/// `Blocked`, and `mutate_saved_character_at_root` then leaves the on-disk
/// character untouched. So a non-empty result here IS the live app
/// refusing to save the spell.
fn spellbook_blockers(input: &CharacterInput) -> Vec<String> {
    compute_pilot_base_chassis(input)
        .diagnostics
        .into_iter()
        .filter(|d| d.claim_blocking && d.id.contains("prepared_spellbook"))
        .map(|d| d.message)
        .collect()
}

fn refuses_with_level_message(input: &CharacterInput) -> bool {
    spellbook_blockers(input)
        .iter()
        .any(|message| message.contains("not yet accessible at wizard level"))
}

/// **The named reproduction.** A Wizard 1 must not be able to prepare
/// Tsunami, and must be told why in the same words CRB already produces.
#[test]
fn a_wizard_1_cannot_prepare_tsunami() {
    let input = wizard_preparing(1, "Tsunami");
    let blockers = spellbook_blockers(&input);
    assert!(
        blockers
            .iter()
            .any(|message| message
                .contains("a prepared spell targets spell level 9, not yet accessible at wizard level 1")),
        "Tsunami is a 9th-level Wizard spell (apg_spells.lst: \
         `CLASSES:Druid,Sorcerer,Wizard=9`); a Wizard 1 must be refused with the \
         same message CRB produces, got: {blockers:?}"
    );
}

/// And the gate is a level gate, not a book ban: once the Wizard is high
/// enough to have 9th-level slots, Tsunami grounds cleanly.
///
/// **The level is 17, not 9.** The brief that commissioned this work asked
/// for "a Wizard 9 CAN", conflating the spell's level with the caster's.
/// PF1's Wizard "Spells per Day" table opens 9th-level slots at class
/// level 17 (`wizard_base_spells_per_day`), so a Wizard 9 genuinely
/// cannot cast Tsunami and the assertion below pins that too. Writing the
/// brief's number would have shipped a test asserting an illegal
/// character.
#[test]
fn a_wizard_9_still_cannot_prepare_tsunami_but_a_wizard_17_can() {
    assert!(
        refuses_with_level_message(&wizard_preparing(9, "Tsunami")),
        "a Wizard 9 has no 9th-level slot in PF1 and must still be refused"
    );

    let blockers = spellbook_blockers(&wizard_preparing(17, "Tsunami"));
    assert!(
        blockers.is_empty(),
        "a Wizard 17 has 9th-level slots, so Tsunami must ground cleanly: {blockers:?}"
    );
}

/// All eight spells the verify agent landed on a live Wizard 1 are now
/// refused — none of them reaches `Computed`.
#[test]
fn every_non_crb_spell_the_live_app_accepted_is_now_refused() {
    let accepted: Vec<&str> = NON_CRB_NINTH_LEVEL_WIZARD_SPELLS
        .iter()
        .copied()
        .filter(|spell_id| spellbook_blockers(&wizard_preparing(1, spell_id)).is_empty())
        .collect();
    assert!(
        accepted.is_empty(),
        "these non-CRB spells were still accepted by a Wizard 1: {accepted:?}"
    );
}

/// Seven of the eight carry a real `CLASSES:...Wizard=9` token, so they get
/// the level message CRB already produced.
#[test]
fn the_seven_with_a_corpus_class_mapping_are_refused_by_level() {
    let not_level_refused: Vec<&str> = NON_CRB_NINTH_LEVEL_WIZARD_SPELLS
        .iter()
        .copied()
        .filter(|key| *key != "Naturalist Summon Nature's Ally IX")
        .filter(|spell_id| !refuses_with_level_message(&wizard_preparing(1, spell_id)))
        .collect();
    assert!(
        not_level_refused.is_empty(),
        "these should have been refused by spell level: {not_level_refused:?}"
    );
}

/// **The eighth has no class mapping in the corpus at all**, and must say
/// so rather than have one invented for it. Its `acg_spells.lst` row
/// (`Summon Nature's Ally IX  KEY:Naturalist Summon Nature's Ally IX
/// TYPE:Arcane  SCHOOL:Conjuration ...`) carries no `CLASSES:` token, so no
/// book states any class's level for it — and a spell whose level is
/// unknown cannot be checked against a slot, so it is refused.
#[test]
fn the_one_with_no_corpus_class_mapping_is_refused_as_unknown_not_invented() {
    let blockers = spellbook_blockers(&wizard_preparing(1, "Naturalist Summon Nature's Ally IX"));
    assert!(
        blockers.iter().any(|message| message.contains(
            "has no 'class:wizard' spell level in any ingested book, so the spell level it \
             would occupy is unknown"
        )),
        "an unmapped spell must be refused as unknown, never assigned a plausible level: \
         {blockers:?}"
    );
    assert!(
        !refuses_with_level_message(&wizard_preparing(1,
            "Naturalist Summon Nature's Ally IX")),
        "and it must NOT claim a spell level the corpus never states"
    );
}

/// **The census: what the gate now covers, per book.**
///
/// Every ingested spell record classified by what a Wizard 1 gets when it
/// tries to prepare it. `accessible` = the corpus states a Wizard level of
/// 0 or 1; `gated by level` = it states a higher Wizard level; `no wizard
/// mapping` = no ingested book states any Wizard level for that record
/// (most are simply not Wizard spells at all — Cleric-only, Druid-only,
/// Summoner eidolon spells — plus a handful, like ACG's `Naturalist Summon
/// Nature's Ally IX`, whose corpus row carries no `CLASSES:` token at all).
///
/// **Before this cycle the second and third columns were both zero for
/// APG, ACG and ARG**: 297 + 144 + 92 = 533 records were accepted by a
/// Wizard of any level, because `class_spell_levels` was consulted by
/// nothing and the CRB-only `SPELL_LIST` lookup returned `None` for them.
///
/// The third column is dominated by spells that are simply not Wizard
/// spells (Cleric-only, Druid-only, Summoner eidolon spells). Only **79 of
/// the 1185** carry no `CLASSES:` token for ANY class anywhere in the
/// corpus — CRB 29, APG 40, ACG 10, ARG 0 — and that split cannot be
/// re-derived here because it needs the raw `.lst` files. Re-derive it with
/// `cargo run --bin ingest_class_spell_levels_arg`, which prints it.
#[test]
fn the_gate_now_covers_every_ingested_book_and_the_census_is_pinned() {
    use codex::rules_core::rules_tables::class_spell_levels;
    use codex::rules_core::rules_tables::{acg, advanced_race_guide, apg, crb};

    let books: Vec<(&str, Vec<&str>)> = vec![
        ("CRB", crb::spell_list::SPELL_LIST.iter().map(|e| e.key).collect()),
        ("APG", apg::spell_list::SPELL_LIST.iter().map(|e| e.key).collect()),
        ("ACG", acg::spell_list::SPELL_LIST.iter().map(|e| e.key).collect()),
        (
            "ARG",
            advanced_race_guide::spell_list::SPELL_LIST.iter().map(|e| e.key).collect(),
        ),
    ];

    // (book, total, accessible at wizard 1, gated by level, no wizard mapping)
    let census: Vec<(&str, usize, usize, usize, usize)> = books
        .iter()
        .map(|(book, keys)| {
            let mut accessible = 0;
            let mut gated = 0;
            let mut unmapped = 0;
            for key in keys {
                match class_spell_levels::class_spell_level("class:wizard", key) {
                    Some(level) if level <= 1 => accessible += 1,
                    Some(_) => gated += 1,
                    None => unmapped += 1,
                }
            }
            (*book, keys.len(), accessible, gated, unmapped)
        })
        .collect();

    assert_eq!(
        census,
        vec![
            ("CRB", 652, 62, 334, 256),
            ("APG", 297, 18, 77, 202),
            ("ACG", 144, 23, 66, 55),
            ("ARG", 92, 15, 47, 30),
        ]
    );
}

/// The CRB behaviour that already worked must keep working — an in-level
/// CRB spell still grounds, and an out-of-level one is still refused.
#[test]
fn the_crb_gate_is_unchanged_in_both_directions() {
    assert!(
        spellbook_blockers(&wizard_preparing(1, "Mage Armor")).is_empty(),
        "Mage Armor is a 1st-level Wizard spell; a Wizard 1 must still be able to prepare it"
    );
    assert!(
        refuses_with_level_message(&wizard_preparing(1, "Fireball")),
        "Fireball is a 3rd-level Wizard spell; a Wizard 1 must still be refused"
    );
}
