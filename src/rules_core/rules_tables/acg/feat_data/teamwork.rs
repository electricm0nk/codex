//! Generated from the real PCGen corpus file `acg_feats.lst` (Advanced Class Guide):
//! every non-comment, non-`.MOD` record whose `TYPE:` facet resolves to
//! `Teamwork` under the rule `acg::feats` documents. 4 records,
//! transcribed verbatim (`KEY:`/name, `TYPE:`, `DESC:`, `BONUS:`, and
//! every top-level `PRE`-family token). Generated programmatically by
//! the same offline method as `crb/feat_data/` -- do not hand-edit;
//! regenerate if the corpus changes.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatTableEntry};

pub const TEAMWORK_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Improved Duck and Cover", category: FeatCategory::Teamwork, name: "Improved Duck and Cover", description: Some("Whenever you use Duck and Cover, your ally has evasion or improved evasion, and your ally's saving throw roll succeeds, half of the damage you would have taken is transferred to your ally. (This damage is not reduced by the ally's evasion or improved evasion.)"), effect: None, prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Duck and Cover"]) },
    FeatTableEntry { key: "Improved Spell Sharing", category: FeatCategory::Teamwork, name: "Improved Spell Sharing", description: Some("Your link with your companion creature allows you to share your magic with it."), effect: None, prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Eidolon,TYPE.Familiar,TYPE.Special Mount],[PREVARGT:MasterLevel,0]"]) },
    FeatTableEntry { key: "Pack Flanking", category: FeatCategory::Teamwork, name: "Pack Flanking", description: Some("You and your companion creature are adept at fighting together against foes."), effect: None, prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]) },
    FeatTableEntry { key: "Share Healing", category: FeatCategory::Teamwork, name: "Share Healing", description: Some("Your link with your companion creature allows you to share with it any healing magic that's cast upon you."), effect: None, prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Eidolon,TYPE.Familiar,TYPE.Special Mount,TYPE.Mount],[PREVARGT:MasterLevel,0]"]) },
];
