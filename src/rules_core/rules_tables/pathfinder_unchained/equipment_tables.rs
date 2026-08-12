//! Pathfinder Unchained (PU) equipment-modifier catalog. SD-27 Cycle
//! E2.2 per-book pre-build (`docs/release/SD-27-future-state-book-
//! content-ingestion/loop-instruction.md §3.3.3`).
//!
//! **Full corpus coverage.** `pu_equipmods.lst` has exactly 42 real,
//! `KEY:`-bearing records -- all 42 are the Automatic Bonus Progression
//! (ABP) variant "equipment modifiers" that back PU's optional ABP
//! subsystem (p.156-157): a per-slot (Weapon/Ammunition/Armor/Shield)
//! ladder of `+0`..`+5` "Enhancement" modifiers plus a matching
//! `+0`..`+5` "Attunement" modifier ladder for Weapon/Armor/Shield. There
//! is no `pu_equip.lst`-style General/ArmsArmor/MagicItems split for this
//! book's own new content -- `pu_equip.lst` exists in the real corpus but
//! carries mundane inherited-book crossover rows, not new PU equipment,
//! so this catalog covers `pu_equipmods.lst` only, matching this cycle's
//! scoped brief.
//!
//! **No `COST:`/`WT:` token anywhere in this file** (confirmed by direct
//! grep of the live corpus: 0 hits each) -- every record's real cost
//! signal is instead an `ITEMCOST`-formula `BONUS:` token (a runtime
//! formula over the target item's own state, not a flat gp number) and
//! there is no `WT:` token at all, mirroring `rules_tables::acg`'s own
//! documented finding for `acg_equipmods.lst` ("equipment *modifiers*
//! have no independent physical weight of their own"). `cost_gp` and
//! `weight_lbs` are therefore always `None` here -- an honest corpus gap,
//! never fabricated.
//!
//! Every record DOES carry a real `DESC:` token (100% coverage, unlike
//! ACG's `SPROP:`-sourced fallback) -- `description` is populated for
//! all 42 records.
//!
//! Real per-record LST path/sha256/line citations are computed by
//! `src/bin/gen_book_cache.rs` at generation time by reading the
//! live corpus file directly (never hand-transcribed here) -- this
//! module supplies only the compiled data values, matching
//! `src/bin/gen_core_rulebook_cache.rs`'s established generation
//! discipline.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentTableEntry {
    /// The real corpus `KEY:` token -- every one of the 42 real
    /// `pu_equipmods.lst` records carries one (unlike
    /// `rules_tables::crb`/`rules_tables::acg`'s own equipment catalogs,
    /// where `key == name` fallback is common).
    pub key: &'static str,
    pub name: &'static str,
    /// The corpus `TYPE:` token, verbatim (`"Weapon"`, `"Ammunition"`,
    /// `"Armor"`, `"Shield"`, or `"Armor.Clothing"` for the 6 "Attuned
    /// Armor" records, which target both armor and clothing item slots).
    pub equip_type: &'static str,
    /// The corpus `PLUS:` token, parsed to an integer 0-5. `None` for
    /// the 8 real `+0` records, which genuinely carry no `PLUS:` token
    /// at all in the corpus (an honest gap: PCGen's own `PLUS:` token is
    /// omitted, not present-as-zero, for the baseline rung of each
    /// ladder) -- never fabricated as `Some(0)`.
    pub plus: Option<u8>,
    /// The corpus `DESC:` token, verbatim. Populated for all 42 records
    /// (100% real coverage for this file).
    pub description: Option<&'static str>,
}

const ABP_DESC: &str =
    "Add to a currently magical equipment to remove enhancement bonus and cost for ABP system.";
const ATTUNE_BASE_DESC: &str =
    "Attunement for equipment that has magic abilities, powering the ability rather than granting bonuses.";
const ATTUNE_PLUS_DESC: &str = "Adds enhancement bonus to attuned equipment.";

/// Full PU equipment-modifier catalog: all 42 real `pu_equipmods.lst`
/// records, in source order (matches this module's own generation
/// citation lookup, which scans the live corpus top-to-bottom).
pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    &[
        // ---- ABP Weapon Enhancement (lines 4-9) ----
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +0 ~ Weapon",
            name: "+0 ABP (Enhancement to Weapon)",
            equip_type: "Weapon",
            plus: None,
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +1 ~ Weapon",
            name: "+1 ABP (Enhancement to Weapon)",
            equip_type: "Weapon",
            plus: Some(1),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +2 ~ Weapon",
            name: "+2 ABP (Enhancement to Weapon)",
            equip_type: "Weapon",
            plus: Some(2),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +3 ~ Weapon",
            name: "+3 ABP (Enhancement to Weapon)",
            equip_type: "Weapon",
            plus: Some(3),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +4 ~ Weapon",
            name: "+4 ABP (Enhancement to Weapon)",
            equip_type: "Weapon",
            plus: Some(4),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +5 ~ Weapon",
            name: "+5 ABP (Enhancement to Weapon)",
            equip_type: "Weapon",
            plus: Some(5),
            description: Some(ABP_DESC),
        },
        // ---- ABP Ammunition Enhancement (lines 10-15) ----
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +0 ~ Ammunition",
            name: "+0 ABP (Enhancement to Ammunition)",
            equip_type: "Ammunition",
            plus: None,
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +1 ~ Ammunition",
            name: "+1 ABP (Enhancement to Ammunition)",
            equip_type: "Ammunition",
            plus: Some(1),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +2 ~ Ammunition",
            name: "+2 ABP (Enhancement to Ammunition)",
            equip_type: "Ammunition",
            plus: Some(2),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +3 ~ Ammunition",
            name: "+3 ABP (Enhancement to Ammunition)",
            equip_type: "Ammunition",
            plus: Some(3),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +4 ~ Ammunition",
            name: "+4 ABP (Enhancement to Ammunition)",
            equip_type: "Ammunition",
            plus: Some(4),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +5 ~ Ammunition",
            name: "+5 ABP (Enhancement to Ammunition)",
            equip_type: "Ammunition",
            plus: Some(5),
            description: Some(ABP_DESC),
        },
        // ---- ABP Armor Enhancement (lines 16-21) ----
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +0 ~ Armor",
            name: "+0 ABP (Enhancement to Armor)",
            equip_type: "Armor",
            plus: None,
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +1 ~ Armor",
            name: "+1 ABP (Enhancement to Armor)",
            equip_type: "Armor",
            plus: Some(1),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +2 ~ Armor",
            name: "+2 ABP (Enhancement to Armor)",
            equip_type: "Armor",
            plus: Some(2),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +3 ~ Armor",
            name: "+3 ABP (Enhancement to Armor)",
            equip_type: "Armor",
            plus: Some(3),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +4 ~ Armor",
            name: "+4 ABP (Enhancement to Armor)",
            equip_type: "Armor",
            plus: Some(4),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +5 ~ Armor",
            name: "+5 ABP (Enhancement to Armor)",
            equip_type: "Armor",
            plus: Some(5),
            description: Some(ABP_DESC),
        },
        // ---- ABP Shield Enhancement (lines 22-27) ----
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +0 ~ Shield",
            name: "+0 ABP (Enhancement to Shield)",
            equip_type: "Shield",
            plus: None,
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +1 ~ Shield",
            name: "+1 ABP (Enhancement to Shield)",
            equip_type: "Shield",
            plus: Some(1),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +2 ~ Shield",
            name: "+2 ABP (Enhancement to Shield)",
            equip_type: "Shield",
            plus: Some(2),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +3 ~ Shield",
            name: "+3 ABP (Enhancement to Shield)",
            equip_type: "Shield",
            plus: Some(3),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +4 ~ Shield",
            name: "+4 ABP (Enhancement to Shield)",
            equip_type: "Shield",
            plus: Some(4),
            description: Some(ABP_DESC),
        },
        EquipmentTableEntry {
            key: "Special Ability ~ ABP +5 ~ Shield",
            name: "+5 ABP (Enhancement to Shield)",
            equip_type: "Shield",
            plus: Some(5),
            description: Some(ABP_DESC),
        },
        // ---- Attuned Weapon (lines 29-34) ----
        EquipmentTableEntry {
            key: "ABP ~ +0 Attunement ~ Weapon",
            name: "+0 Attuned Weapon",
            equip_type: "Weapon",
            plus: None,
            description: Some(ATTUNE_BASE_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +1 Attunement ~ Weapon",
            name: "+1 Attuned Weapon",
            equip_type: "Weapon",
            plus: Some(1),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +2 Attunement ~ Weapon",
            name: "+2 Attuned Weapon",
            equip_type: "Weapon",
            plus: Some(2),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +3 Attunement ~ Weapon",
            name: "+3 Attuned Weapon",
            equip_type: "Weapon",
            plus: Some(3),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +4 Attunement ~ Weapon",
            name: "+4 Attuned Weapon",
            equip_type: "Weapon",
            plus: Some(4),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +5 Attunement ~ Weapon",
            name: "+5 Attuned Weapon",
            equip_type: "Weapon",
            plus: Some(5),
            description: Some(ATTUNE_PLUS_DESC),
        },
        // ---- Attuned Armor (lines 35-40) ----
        EquipmentTableEntry {
            key: "ABP ~ +0 Attunement ~ Armor",
            name: "+0 Attuned Armor",
            equip_type: "Armor.Clothing",
            plus: None,
            description: Some(ATTUNE_BASE_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +1 Attunement ~ Armor",
            name: "+1 Attuned Armor",
            equip_type: "Armor.Clothing",
            plus: Some(1),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +2 Attunement ~ Armor",
            name: "+2 Attuned Armor",
            equip_type: "Armor.Clothing",
            plus: Some(2),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +3 Attunement ~ Armor",
            name: "+3 Attuned Armor",
            equip_type: "Armor.Clothing",
            plus: Some(3),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +4 Attunement ~ Armor",
            name: "+4 Attuned Armor",
            equip_type: "Armor.Clothing",
            plus: Some(4),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +5 Attunement ~ Armor",
            name: "+5 Attuned Armor",
            equip_type: "Armor.Clothing",
            plus: Some(5),
            description: Some(ATTUNE_PLUS_DESC),
        },
        // ---- Attuned Shield (lines 41-46) ----
        EquipmentTableEntry {
            key: "ABP ~ +0 Attunement ~ Shield",
            name: "+0 Attuned Shield",
            equip_type: "Shield",
            plus: None,
            description: Some(ATTUNE_BASE_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +1 Attunement ~ Shield",
            name: "+1 Attuned Shield",
            equip_type: "Shield",
            plus: Some(1),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +2 Attunement ~ Shield",
            name: "+2 Attuned Shield",
            equip_type: "Shield",
            plus: Some(2),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +3 Attunement ~ Shield",
            name: "+3 Attuned Shield",
            equip_type: "Shield",
            plus: Some(3),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +4 Attunement ~ Shield",
            name: "+4 Attuned Shield",
            equip_type: "Shield",
            plus: Some(4),
            description: Some(ATTUNE_PLUS_DESC),
        },
        EquipmentTableEntry {
            key: "ABP ~ +5 Attunement ~ Shield",
            name: "+5 Attuned Shield",
            equip_type: "Shield",
            plus: Some(5),
            description: Some(ATTUNE_PLUS_DESC),
        },
    ]
}
