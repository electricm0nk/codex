//! The Abilities tab's chosen picks (SD-43 E3.11, `decisions.md §20`): after the Feats block, each
//! sub-tab's picks under the sub-tab's heading, in the tab's order, and the favored class bonus
//! under its own. Each pick prints by the name its sub-tab's Selected list shows and the rule text
//! the engine carries for it; no number (the bonuses' totals are already on the sheet).

use std::path::Path;

use super::SheetSources;
use crate::print::model::{SheetBlock, SheetRow};

/// One heading's picks: each its name and the engine's rule text, when it carries one.
pub struct ChosenAbilities {
    pub heading: &'static str,
    pub picks: Vec<(String, Option<String>)>,
}

/// The sub-tabs after Feats in the tab's order (`AbilitiesTab.tsx` `ABILITY_SUB_TABS`), each its
/// command id and heading; the Favored Class Bonus is a section of Class Features there, so it
/// follows Class Features here.
const SUBTABS: [(&str, &str); 7] = [
    ("racial_abilities", "Racial Abilities"),
    ("traits", "Traits"),
    ("archetype", "Archetype"),
    ("background", "Background"),
    ("class_features", "Class Features"),
    ("favored_class", "Favored Class Bonus"),
    ("misc", "Misc"),
];

/// Each sub-tab's held picks as the sub-tab lists them (`list_ability_options_at_root`).
pub(crate) fn chosen_abilities_at_root(root: &Path) -> Result<Vec<ChosenAbilities>, String> {
    SUBTABS
        .iter()
        .map(|&(subtab, heading)| {
            let listed = crate::abilities::list_ability_options_at_root(root, subtab)?;
            let picks = listed
                .selected
                .iter()
                .map(|held| match listed.options.iter().find(|option| &option.id == held) {
                    Some(option) => (option.label.clone(), option.description.clone()),
                    None => (held.clone(), None),
                })
                .collect();
            Ok(ChosenAbilities { heading, picks })
        })
        .collect()
}

pub fn chosen_abilities(sources: &SheetSources) -> SheetBlock {
    let mut rows: Vec<SheetRow> = Vec::new();
    for chosen in sources.chosen_abilities.iter().filter(|chosen| !chosen.picks.is_empty()) {
        rows.push(SheetRow::new(chosen.heading, Vec::<String>::new()).of_kind("heading"));
        rows.extend(chosen.picks.iter().map(|(name, text)| SheetRow::new(name.clone(), Vec::<String>::new()).with_notes(text.iter().cloned())));
    }
    if rows.is_empty() {
        rows.push(SheetRow::new("No ability is chosen on the Abilities tab.", Vec::<String>::new()));
    }
    SheetBlock::new("Chosen Abilities", rows)
}
