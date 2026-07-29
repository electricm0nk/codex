//! Generated from the real PCGen corpus file `acg_feats.lst` (Advanced Class Guide):
//! every non-comment, non-`.MOD` record whose `TYPE:` facet resolves to
//! `Panache` under the rule `acg::feats` documents. 4 records,
//! transcribed verbatim (`KEY:`/name, `TYPE:`, `DESC:`, `BONUS:`, and
//! every top-level `PRE`-family token). Generated programmatically by
//! the same offline method as `crb/feat_data/` -- do not hand-edit;
//! regenerate if the corpus changes.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatEffectBonus, FeatTableEntry};

pub const PANACHE_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Confounding Tumble Deed", category: FeatCategory::Panache, name: "Confounding Tumble Deed", description: Some("You can befuddle a foe by striking a blow after tumbling."), effect: None, prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Canny Tumble", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PRESKILL:1,Acrobatics=7"]) },
    FeatTableEntry { key: "Disarming Threat Deed", category: FeatCategory::Panache, name: "Disarming Threat Deed", description: Some("Even your threats are curiously charming."), effect: None, prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PRESKILL:2,Diplomacy=2,Intimidate=2"]) },
    FeatTableEntry { key: "Extra Panache", category: FeatCategory::Panache, name: "Extra Panache", description: Some("You have more panache than the ordinary swashbuckler."), effect: Some(&[FeatEffectBonus { qualifiers: &["VAR", "PanachePoints", "2"] }, FeatEffectBonus { qualifiers: &["VAR", "Panache_Cap", "2"] }]), prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Panache,TYPE.Panache]", "PREMULT:1,[!PREABILITY:1,CATEGORY=FEAT,Extra Panache],[PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Panache,TYPE.Panache]"]) },
    FeatTableEntry { key: "Pommel Strike Deed", category: FeatCategory::Panache, name: "Pommel Strike Deed", description: Some("With a surprise swipe with your pommel, you can topple a foe."), effect: None, prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PRETOTALAB:3"]) },
];
