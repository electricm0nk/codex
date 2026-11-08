//! Bio and campaign notes (operator ruling R17: use what exists): the character's bio
//! (`bio.json`) and the description of each campaign whose party lists the character, which the
//! caller sends (the campaigns live in the frontend's store, `campaignModel.ts`). No new field.

use super::SheetSources;
use crate::print::model::{SheetBlock, SheetRow};

pub fn bio_notes(sources: &SheetSources) -> SheetBlock {
    let bio = &sources.bio;
    let fields = [
        ("Player", &bio.player_name),
        ("Alignment", &bio.alignment),
        ("Deity", &bio.deity),
        ("Sex", &bio.sex),
        ("Age", &bio.age),
        ("Height", &bio.height),
        ("Weight", &bio.weight),
        ("Hair", &bio.hair),
        ("Eyes", &bio.eyes),
    ];
    let mut rows: Vec<SheetRow> = fields
        .into_iter()
        .filter(|(_, value)| !value.trim().is_empty())
        .map(|(label, value)| SheetRow::new(label, [value.clone()]))
        .collect();
    if rows.is_empty() {
        rows.push(SheetRow::new("No bio is recorded.", Vec::<String>::new()));
    }
    if sources.request.campaigns.is_empty() {
        rows.push(SheetRow::new("No campaign lists this character.", Vec::<String>::new()));
    }
    for campaign in &sources.request.campaigns {
        rows.push(SheetRow::new(format!("Campaign: {}", campaign.name), Vec::<String>::new()).with_notes(campaign.description.split('\n').map(str::to_owned)));
    }
    SheetBlock::new("Bio and Campaign Notes", rows).on_new_page()
}
