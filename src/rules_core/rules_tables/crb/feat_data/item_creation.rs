//! Generated from the real PCGen corpus file `cr_feats.lst` (`TYPE:` facet
//! containing `ItemCreation`). 8 records, transcribed verbatim
//! (`KEY:`/name, `TYPE:`, `DESC:`) from the corpus's `###Block: General Feats`
//! section -- see `feats.rs`'s own doc comment for why category is derived from
//! the `TYPE:` facet rather than the corpus's `###Block:` markers, and for the
//! generation method. Not hand-authored -- do not hand-edit; regenerate if the
//! corpus changes.

use crate::rules_core::rules_tables::crb::feats::{FeatCategory, FeatTableEntry};

pub const ITEM_CREATION_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Brew Potion", category: FeatCategory::ItemCreation, name: "Brew Potion", description: Some("You can create magic potions.") },
    FeatTableEntry { key: "Craft Magic Arms and Armor", category: FeatCategory::ItemCreation, name: "Craft Magic Arms and Armor", description: Some("You can create magic armor, shields, or weapons.") },
    FeatTableEntry { key: "Craft Rod", category: FeatCategory::ItemCreation, name: "Craft Rod", description: Some("You can create magic rods.") },
    FeatTableEntry { key: "Craft Staff", category: FeatCategory::ItemCreation, name: "Craft Staff", description: Some("You can create magic staves.") },
    FeatTableEntry { key: "Craft Wand", category: FeatCategory::ItemCreation, name: "Craft Wand", description: Some("You can create magic wands.") },
    FeatTableEntry { key: "Craft Wondrous Item", category: FeatCategory::ItemCreation, name: "Craft Wondrous Item", description: Some("You can create wondrous items, a type of magic item.") },
    FeatTableEntry { key: "Forge Ring", category: FeatCategory::ItemCreation, name: "Forge Ring", description: Some("You can create magic rings.") },
    FeatTableEntry { key: "Scribe Scroll", category: FeatCategory::ItemCreation, name: "Scribe Scroll", description: Some("You can create magic scrolls.") },
];

