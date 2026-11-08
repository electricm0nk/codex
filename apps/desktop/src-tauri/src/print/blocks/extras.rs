//! Custom (the Custom tab's grants, records, conditions and override flags, as `custom.json` saves
//! them) and Pets (the companion stat block the engine computed; `snapshot.companion`).

use super::SheetSources;
use crate::character_custom::CustomRecordDto;
use crate::print::layout::signed;
use crate::print::model::{Derivation, SheetBlock, SheetRow};

fn record_row(kind: &str, record: &CustomRecordDto) -> SheetRow {
    let stats = record.stats.iter().map(|stat| format!("{}: {}", stat.label, stat.value));
    let bonuses = record.bonuses.iter().map(|bonus| format!("{} {} on {}", signed_i32(bonus.value), bonus.bonus_type, bonus.target));
    SheetRow::new(format!("{kind}: {}", record.name), Vec::<String>::new())
        .with_notes(stats)
        .with_notes(bonuses)
        .with_notes(record.description.split('\n').map(str::to_owned))
}

fn signed_i32(value: i32) -> String {
    if value >= 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

pub fn custom(sources: &SheetSources) -> SheetBlock {
    let custom = &sources.custom;
    let mut rows: Vec<SheetRow> = custom
        .grants
        .iter()
        .map(|grant| SheetRow::new(grant.label.clone(), [grant.target.clone(), signed_i32(grant.value)]).with_notes([grant.reason.clone()]))
        .collect();
    for (kind, records) in [("Feat", &custom.feats), ("Item", &custom.equipment), ("Spell", &custom.spells), ("Device", &custom.devices)] {
        rows.extend(records.iter().map(|record| record_row(kind, record)));
    }
    let conditions = if custom.conditions.is_empty() { "Normal".to_owned() } else { custom.conditions.join(", ") };
    rows.push(SheetRow::new("Conditions", [conditions]));
    let flags = custom.overrides;
    let on: Vec<String> = [
        (flags.feats_without_prerequisites, crate::character_ledger::OverrideFlag::FeatsWithoutPrerequisites),
        (flags.weapons_without_proficiency, crate::character_ledger::OverrideFlag::WeaponsWithoutProficiency),
        (flags.spells_outside_class_list, crate::character_ledger::OverrideFlag::SpellsOutsideClassList),
        (flags.armor_without_proficiency, crate::character_ledger::OverrideFlag::ArmorWithoutProficiency),
        (flags.ignore_slot_limits, crate::character_ledger::OverrideFlag::IgnoreSlotLimits),
        (flags.ignore_feat_budget, crate::character_ledger::OverrideFlag::IgnoreFeatBudget),
        (flags.ignore_alignment_restrictions, crate::character_ledger::OverrideFlag::IgnoreAlignmentRestrictions),
        (flags.ignore_encumbrance, crate::character_ledger::OverrideFlag::IgnoreEncumbrance),
    ]
    .into_iter()
    .filter(|(set, _)| *set)
    .map(|(_, flag)| super::override_tag(flag))
    .collect();
    if !on.is_empty() {
        rows.push(SheetRow::new("Overrides on", Vec::<String>::new()).with_notes(on));
    }
    SheetBlock::new("Custom", rows)
}

pub fn pets(sources: &SheetSources) -> SheetBlock {
    let Some(companion) = sources.loaded.snapshot.as_ref().and_then(|snapshot| snapshot.companion.as_ref()) else {
        return SheetBlock::new("Pets", vec![SheetRow::new("No animal companion or mount.", Vec::<String>::new())]);
    };
    let mut rows = vec![
        SheetRow::new(companion.role_label.clone(), [companion.species.clone()])
            .with_notes([format!("From {}", companion.owner_class_label), companion.summary_detail.clone()]),
    ];
    rows.extend(companion.stats.iter().map(|stat| {
        SheetRow::new(stat.label.clone(), [signed(stat.value)])
            .derived(vec![Derivation { source: format!("companion: {} = {}", stat.label, stat.value), detail: stat.detail.clone() }])
    }));
    if !companion.notes.is_empty() || companion.advancement_note.is_some() {
        rows.push(
            SheetRow::new("Notes", Vec::<String>::new())
                .with_notes(companion.notes.iter().cloned())
                .with_notes(companion.advancement_note.iter().cloned()),
        );
    }
    SheetBlock::new("Pets", rows)
}
