//! Generated from the real PCGen corpus file `acg_feats.lst` (Advanced Class Guide):
//! every non-comment, non-`.MOD` record whose `TYPE:` facet resolves to
//! `Teamwork` under the rule `acg::feats` documents. 4 records,
//! transcribed verbatim (`KEY:`/name, `TYPE:`, `DESC:`, `BONUS:`, and
//! every top-level `PRE`-family token). Generated programmatically by
//! the same offline method as `crb/feat_data/` -- do not hand-edit;
//! regenerate if the corpus changes.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatTableEntry};

pub const TEAMWORK_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Improved Duck and Cover", category: FeatCategory::Teamwork, name: "Improved Duck and Cover", description: Some("Whenever you use Duck and Cover, your ally has evasion or improved evasion, and your ally's saving throw roll succeeds, half of the damage you would have taken is transferred to your ally. (This damage is not reduced by the ally's evasion or improved evasion.)"), effect: None},
    FeatTableEntry { key: "Improved Spell Sharing", category: FeatCategory::Teamwork, name: "Improved Spell Sharing", description: Some("Your link with your companion creature allows you to share your magic with it."), effect: None},
    FeatTableEntry { key: "Pack Flanking", category: FeatCategory::Teamwork, name: "Pack Flanking", description: Some("You and your companion creature are adept at fighting together against foes."), effect: None},
    FeatTableEntry { key: "Share Healing", category: FeatCategory::Teamwork, name: "Share Healing", description: Some("Your link with your companion creature allows you to share with it any healing magic that's cast upon you."), effect: None},
];
