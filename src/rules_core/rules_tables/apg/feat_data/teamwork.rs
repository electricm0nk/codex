//! Generated from the real PCGen corpus file `apg_feats.lst` (Advanced Player's Guide):
//! every non-comment, non-`.MOD` record whose `TYPE:` facet resolves to
//! `Teamwork` under the rule `apg::feats` documents. 3 records,
//! transcribed verbatim (`KEY:`/name, `TYPE:`, `DESC:`, `BONUS:`, and
//! every top-level `PRE`-family token). Generated programmatically by
//! the same offline method as `crb/feat_data/` -- do not hand-edit;
//! regenerate if the corpus changes.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatTableEntry};

pub const TEAMWORK_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Allied Spellcaster", category: FeatCategory::Teamwork, name: "Allied Spellcaster", description: Some("With the aid of an ally, you are skilled at piercing the protections of other creatures with your spells."), effect: None, prerequisites: Some(&["PREMULT:1,[PRECLASS:1,SPELLCASTER=1],[PREVARGTEQ:CasterLevel_Highest,1]"]) },
    FeatTableEntry { key: "Duck and Cover", category: FeatCategory::Teamwork, name: "Duck and Cover", description: Some("Your allies assist you in avoiding certain attacks."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Shielded Caster", category: FeatCategory::Teamwork, name: "Shielded Caster", description: Some("Your allies cover you while you cast complicated spells."), effect: None, prerequisites: None },
];
