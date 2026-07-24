//! SD-20 Epic 6 (damage-total engine): third damage-class criterion per
//! `scope-draft.md` §1.6's work-unit order — weapon-enhancement modifier
//! (sequential after base-dice round-trip `208f326` and STR-modifier
//! handling `f1188fe`).
//!
//! PF1's weapon-enhancement rule: a magic weapon's enhancement bonus
//! (e.g. a "+1 longsword") adds to **both** the attack roll and the
//! damage roll — confirmed via `technical-design.md` §2.4's own
//! illustrative equipment-effects deliverable ("Magic weapons with
//! enhancement bonuses contribute to `attack_bonus_delta`") plus §2.5's
//! `damage_modifier` doc comment ("STR mod + weapon enhancement + ...").
//! Some `equipmods` records carry a narrower, `TOHIT`-only enhancement
//! (masterwork/special materials, e.g. `Adamantine`) rather than the
//! full `DAMAGE,TOHIT` a true magical "+N" enhancement carries — this
//! engine reads that distinction verbatim off the corpus token rather
//! than assuming every enhancement source affects both rolls uniformly.
//!
//! This work-unit composes with Epic 5's already-landed, already-closed
//! equipment-effect engine (`equipment_effects.rs`,
//! `compute_equipment_effects` / `equipment_effects::equipmods::compute_equipmods_effect`)
//! rather than re-deriving the `BONUS:WEAPON|...|TYPE=Enhancement` corpus
//! lookup independently — per this cycle's brief. `equipment_effects.rs`
//! and its category files are read-only from this test/module; nothing
//! here modifies them.
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_arms_armor.lst` (the `Longsword (Base)`
//! weapon record, same fixture the sibling `sd20_damage_str_modifier.rs`
//! cycle already used) and `core_rulebook/cr_equipmods.lst` (`KEY:Special
//! Ability ~ +1 ~ Weapon`, line 219, `BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement`;
//! `KEY:Material ~ Adamantine ~ Weapon`, line 101,
//! `BONUS:WEAPON|TOHIT|1|TYPE=Enhancement`) — confirmed directly against
//! the live corpus at
//! `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equipmods.lst`.
//! Same parser (`parse_equipment_entries`) and IR converter
//! (`convert_equipment_record`) the sibling tests already use — so this
//! test genuinely proves the engine's real
//! `resolve_weapon_enhancement_modifier` seam composes Epic 5's real,
//! already-resolved `EquipmentEffects` into a real attack/damage
//! contribution, not a hand-rolled fixture.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::damage_total::resolve_weapon_enhancement_modifier;
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows: one weapon-block record
/// (`core_rulebook/cr_equip_arms_armor.lst`) plus two equipmods-block
/// records (`core_rulebook/cr_equipmods.lst`) spanning both the
/// `DAMAGE,TOHIT` (true magical enhancement) and `TOHIT`-only
/// (masterwork material) shapes.
const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement
Adamantine\tKEY:Material ~ Adamantine ~ Weapon\tTYPE:BaseMaterial.MasterworkQuality.Weapon\tCOST:3000\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
    let source_ref = SourceRef {
        lst_file: "cr_equip_arms_armor.lst".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
    for record in result.entries {
        let record: &'static EquipmentRecord = Box::leak(Box::new(record));
        corpus.push(convert_equipment_record(record));
    }
    corpus
}

fn selection(item_id: &str) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    }
}

#[test]
fn plus_one_weapon_adds_its_bonus_to_both_attack_and_damage() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipped = vec![
        selection("Longsword (Base)"),
        selection("Special Ability ~ +1 ~ Weapon"),
    ];
    let effects = compute_equipment_effects(&equipped, &corpus);

    let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
        .expect("Longsword (Base) must resolve a weapon-enhancement round-trip");

    assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
    assert_eq!(resolved.attack_bonus, 1, "a +1 weapon adds +1 to the attack roll");
    assert_eq!(resolved.damage_bonus, 1, "a +1 weapon adds +1 to the damage roll");
}

#[test]
fn tohit_only_enhancement_does_not_add_to_damage() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipped = vec![
        selection("Longsword (Base)"),
        selection("Material ~ Adamantine ~ Weapon"),
    ];
    let effects = compute_equipment_effects(&equipped, &corpus);

    let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
        .expect("Longsword (Base) must resolve a weapon-enhancement round-trip");

    assert_eq!(
        resolved.attack_bonus, 1,
        "an Adamantine-material TOHIT-only enhancement still adds to the attack roll"
    );
    assert_eq!(
        resolved.damage_bonus, 0,
        "an Adamantine-material TOHIT-only enhancement must not fabricate a damage bonus"
    );
}

#[test]
fn no_enhancement_equipped_yields_zero_bonuses_honest_zero_not_fabricated() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipped = vec![selection("Longsword (Base)")];
    let effects = compute_equipment_effects(&equipped, &corpus);

    let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
        .expect("Longsword (Base) must resolve a weapon-enhancement round-trip");

    assert_eq!(resolved.attack_bonus, 0);
    assert_eq!(resolved.damage_bonus, 0);
}

#[test]
fn unresolvable_item_id_yields_none_not_fabricated() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let effects = compute_equipment_effects(&[], &corpus);

    let resolved = resolve_weapon_enhancement_modifier(
        "item:does-not-exist-in-this-corpus",
        &corpus,
        &effects,
    );

    assert!(resolved.is_none());
}
