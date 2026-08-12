//! Generated from the real PCGen corpus file `cr_feats.lst` (`TYPE:` facet
//! containing `Metamagic`). 17 records, transcribed verbatim
//! (`KEY:`/name, `TYPE:`, `DESC:`, `BONUS:`) from the corpus's `###Block:
//! General Feats` section -- see `feats.rs`'s own doc comment for why
//! category is derived from the `TYPE:` facet rather than the corpus's
//! `###Block:` markers, and for the generation method. Not hand-authored --
//! do not hand-edit; regenerate if the corpus changes.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatEffectBonus, FeatTableEntry};

pub const METAMAGIC_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Empower Spell", category: FeatCategory::Metamagic, name: "Empower Spell", description: Some("You can increase the power of your spells, causing them to deal more damage."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Enlarge Spell", category: FeatCategory::Metamagic, name: "Enlarge Spell", description: Some("You can increase the range of your spells."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Extend Spell", category: FeatCategory::Metamagic, name: "Extend Spell", description: Some("You can make your spells last twice as long."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Heighten Spell", category: FeatCategory::Metamagic, name: "Heighten Spell", description: Some("You can cast spells as if they were a higher level."), effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "1"] }]), prerequisites: None },
    FeatTableEntry { key: "Maximize Spell", category: FeatCategory::Metamagic, name: "Maximize Spell", description: Some("Your spells have the maximum possible effect."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Quicken Spell", category: FeatCategory::Metamagic, name: "Quicken Spell", description: Some("You can cast spells in the fraction of the normal time."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Silent Spell", category: FeatCategory::Metamagic, name: "Silent Spell", description: Some("You can cast your spells without making any sound."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Still Spell", category: FeatCategory::Metamagic, name: "Still Spell", description: Some("You can cast spells without moving."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Widen Spell", category: FeatCategory::Metamagic, name: "Widen Spell", description: Some("You can cast your spells so that they occupy a larger space."), effect: None, prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +2", category: FeatCategory::Metamagic, name: "Heighten Spell +2", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "2"] }]), prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +3", category: FeatCategory::Metamagic, name: "Heighten Spell +3", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "3"] }]), prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +4", category: FeatCategory::Metamagic, name: "Heighten Spell +4", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "4"] }]), prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +5", category: FeatCategory::Metamagic, name: "Heighten Spell +5", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "5"] }]), prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +6", category: FeatCategory::Metamagic, name: "Heighten Spell +6", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "6"] }]), prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +7", category: FeatCategory::Metamagic, name: "Heighten Spell +7", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "7"] }]), prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +8", category: FeatCategory::Metamagic, name: "Heighten Spell +8", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "8"] }]), prerequisites: None },
    FeatTableEntry { key: "Heighten Spell +9", category: FeatCategory::Metamagic, name: "Heighten Spell +9", description: None, effect: Some(&[FeatEffectBonus { qualifiers: &["DC", "FEATBONUS", "9"] }]), prerequisites: None },
];

