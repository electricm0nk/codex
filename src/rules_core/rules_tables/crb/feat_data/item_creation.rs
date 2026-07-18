//! Generated from the real PCGen corpus file `cr_feats.lst` (`TYPE:` facet
//! containing `ItemCreation`). 8 records, transcribed verbatim
//! (`KEY:`/name, `TYPE:`, `DESC:`, `BONUS:`) from the corpus's `###Block:
//! General Feats` section -- see `feats.rs`'s own doc comment for why
//! category is derived from the `TYPE:` facet rather than the corpus's
//! `###Block:` markers, and for the generation method. Not hand-authored --
//! do not hand-edit; regenerate if the corpus changes. None of these 8
//! records carry a `BONUS:` token (`effect: None` for every entry) --
//! `ItemCreation` feats' real mechanical effect is a crafting-rule
//! paragraph, not a numeric bonus, so no `FeatEffectBonus` import is
//! needed in this file.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatTableEntry};

pub const ITEM_CREATION_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Brew Potion", category: FeatCategory::ItemCreation, name: "Brew Potion", description: Some("You can create magic potions."), effect: None },
    FeatTableEntry { key: "Craft Magic Arms and Armor", category: FeatCategory::ItemCreation, name: "Craft Magic Arms and Armor", description: Some("You can create magic armor, shields, or weapons."), effect: None },
    FeatTableEntry { key: "Craft Rod", category: FeatCategory::ItemCreation, name: "Craft Rod", description: Some("You can create magic rods."), effect: None },
    FeatTableEntry { key: "Craft Staff", category: FeatCategory::ItemCreation, name: "Craft Staff", description: Some("You can create magic staves."), effect: None },
    FeatTableEntry { key: "Craft Wand", category: FeatCategory::ItemCreation, name: "Craft Wand", description: Some("You can create magic wands."), effect: None },
    FeatTableEntry { key: "Craft Wondrous Item", category: FeatCategory::ItemCreation, name: "Craft Wondrous Item", description: Some("You can create wondrous items, a type of magic item."), effect: None },
    FeatTableEntry { key: "Forge Ring", category: FeatCategory::ItemCreation, name: "Forge Ring", description: Some("You can create magic rings."), effect: None },
    FeatTableEntry { key: "Scribe Scroll", category: FeatCategory::ItemCreation, name: "Scribe Scroll", description: Some("You can create magic scrolls."), effect: None },
];

