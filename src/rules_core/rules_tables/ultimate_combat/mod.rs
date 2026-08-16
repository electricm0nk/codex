//! Ultimate Combat (UC). SD-28 Epic 27 (`epic-27-uc-complete`) --
//! from-scratch book ingest, first slice: the 263-record feat catalog. See
//! `feat_tables`'s own module doc comment for the catalog and its
//! own-category-enum ruling (UC's `Style`/`Grit`/`Panache`/`Critical`/
//! `CalledShot` facets have no shared `crb::feats::FeatCategory`
//! equivalent, or are kept deliberately distinct from a same-named one).
//!
//! `SD31-E4-F1-002` (epic-4-mechanism F1) adds the first UC class-chassis
//! wiring: Gunslinger. UC has three real `Base.PC`/`Base.PC.Rogue`/
//! `Base.PC.Cavalier` classes per `SD31-E3-F1-001`'s clearance table
//! (Gunslinger, Ninja, Samurai); Ninja and Samurai measured `named_raw: 0`
//! archetype-replaceable slots there (no archetype content this book's
//! own corpus rows name for them), so `UcClassId` names Gunslinger only
//! this cycle -- Ninja/Samurai are not silently claimed, only not yet
//! reached.

pub mod archetype_tables;
pub mod class_gunslinger;
pub mod equipment_tables;
pub mod feat_tables;

/// One resolved class-chassis row: level, base attack bonus, and the
/// three base saves. Mirrors `rules_tables::acg::ClassTableRow` (kept
/// book-local rather than shared, per that module's own established
/// shape -- `pathfinder_unchained::class_chassis`'s own doc comment on
/// the same non-sharing choice applies here too).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassTableRow {
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// Identifies which UC class a chassis-table query targets. Gunslinger
/// only, this cycle -- see this module's own doc comment for why Ninja
/// and Samurai are not yet named here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UcClassId {
    Gunslinger,
}

impl UcClassId {
    pub const ALL: [UcClassId; 1] = [UcClassId::Gunslinger];

    /// Lowercase class name, matching the `class_id` string convention
    /// `pilot_compute.rs`'s per-class constants use (`"class:<name>"`).
    /// Mirrors `rules_tables::acg::AcgClassId::name`.
    pub const fn name(self) -> &'static str {
        match self {
            UcClassId::Gunslinger => "gunslinger",
        }
    }

    /// Reverse of `name`: resolves a `"class:<name>"` id string back to a
    /// `UcClassId`. Mirrors `rules_tables::acg::AcgClassId::from_class_id_str`.
    pub fn from_class_id_str(class_id_str: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|id| class_id_str == format!("class:{}", id.name()))
    }
}

/// Resolves one `UcClassId`'s chassis row at `level`. `rule_set` is
/// checked the same way every other book's own `class_chassis_resolve`
/// does, for symmetry with the ACG/PU call sites even though UC has only
/// one rule-set variant today.
pub fn class_chassis_resolve(
    class_id: UcClassId,
    level: u8,
    rule_set: crate::rules_core::rules_tables::RuleSetId,
) -> Option<ClassTableRow> {
    if rule_set != crate::rules_core::rules_tables::RuleSetId::Uc {
        return None;
    }
    match class_id {
        UcClassId::Gunslinger => {
            class_gunslinger::class_table().into_iter().find(|row| row.level == level)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_class_id_str_round_trips_gunslinger() {
        assert_eq!(UcClassId::from_class_id_str("class:gunslinger"), Some(UcClassId::Gunslinger));
        assert_eq!(UcClassId::from_class_id_str("class:ninja"), None);
        assert_eq!(UcClassId::from_class_id_str("class:samurai"), None);
    }

    #[test]
    fn class_chassis_resolve_requires_the_uc_rule_set() {
        assert!(class_chassis_resolve(
            UcClassId::Gunslinger,
            1,
            crate::rules_core::rules_tables::RuleSetId::Acg
        )
        .is_none());
        assert!(class_chassis_resolve(
            UcClassId::Gunslinger,
            1,
            crate::rules_core::rules_tables::RuleSetId::Uc
        )
        .is_some());
    }
}
