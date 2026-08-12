//! Generated from the real PCGen corpus file `apg_feats.lst` (Advanced Player's Guide):
//! every non-comment, non-`.MOD` record whose `TYPE:` facet resolves to
//! `Metamagic` under the rule `apg::feats` documents. 19 records,
//! transcribed verbatim (`KEY:`/name, `TYPE:`, `DESC:`, `BONUS:`, and
//! every top-level `PRE`-family token). Generated programmatically by
//! the same offline method as `crb/feat_data/` -- do not hand-edit;
//! regenerate if the corpus changes.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatTableEntry};

pub const METAMAGIC_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Bouncing Spell", category: FeatCategory::Metamagic, name: "Bouncing Spell", description: Some("You can direct a failed spell against a different target."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Dazing Spell", category: FeatCategory::Metamagic, name: "Dazing Spell", description: Some("You can daze creatures with the power of your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Disruptive Spell", category: FeatCategory::Metamagic, name: "Disruptive Spell", description: Some("Your magical energies cling to enemies, interfering with their spellcasting."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Ectoplasmic Spell", category: FeatCategory::Metamagic, name: "Ectoplasmic Spell", description: Some("Your spells breach the gulf between dimensions, sending ghostly emanations into the ether."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Elemental Spell", category: FeatCategory::Metamagic, name: "Elemental Spell", description: Some("You can manipulate the elemental nature of your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Elemental Spell ~ Acid", category: FeatCategory::Metamagic, name: "Elemental Spell (Acid)", description: Some("You can manipulate the elemental nature of your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Elemental Spell ~ Cold", category: FeatCategory::Metamagic, name: "Elemental Spell (Cold)", description: Some("You can manipulate the elemental nature of your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Elemental Spell ~ Electricity", category: FeatCategory::Metamagic, name: "Elemental Spell (Electricity)", description: Some("You can manipulate the elemental nature of your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Elemental Spell ~ Fire", category: FeatCategory::Metamagic, name: "Elemental Spell (Fire)", description: Some("You can manipulate the elemental nature of your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Focused Spell", category: FeatCategory::Metamagic, name: "Focused Spell", description: Some("When you cast a spell that affects more than one creature, one opponent finds it more difficult to resist."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Intensified Spell", category: FeatCategory::Metamagic, name: "Intensified Spell", description: Some("Your spells can go beyond several normal limitations."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Lingering Spell", category: FeatCategory::Metamagic, name: "Lingering Spell", description: Some("You spell clings to existence, slowly fading from the world."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Merciful Spell", category: FeatCategory::Metamagic, name: "Merciful Spell", description: Some("Your damaging spells subdue rather than kill."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Persistent Spell", category: FeatCategory::Metamagic, name: "Persistent Spell", description: Some("You can modify a spell to become more tenacious when its targets resist its effect."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Preferred Spell", category: FeatCategory::Metamagic, name: "Preferred Spell", description: Some("You find it very easy to cast one particular spell."), effect: None, prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Heighten Spell", "PRESKILL:1,Spellcraft=5"]) },
    FeatTableEntry { key: "Reach Spell", category: FeatCategory::Metamagic, name: "Reach Spell", description: Some("Your spells go farther than normal."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Selective Spell", category: FeatCategory::Metamagic, name: "Selective Spell", description: Some("Your allies need not fear friendly fire."), effect: None, prerequisites: Some(&["PRESKILL:1,Spellcraft=10"]) },
    FeatTableEntry { key: "Sickening Spell", category: FeatCategory::Metamagic, name: "Sickening Spell", description: Some("You can sicken creatures with your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Thundering Spell", category: FeatCategory::Metamagic, name: "Thundering Spell", description: Some("You can conjure your spells into existence with blaring thunder or fearful shrieks, deafening creatures damaged by their effects."), effect: None, prerequisites: None },
];
