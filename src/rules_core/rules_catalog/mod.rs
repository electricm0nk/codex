//! The package-backed rules catalog (SD-37 E4a.2, `docs/release/SD-37-starfinder-1e/decisions.md §19`).
//!
//! Every importer of the Pathfinder rules tables reads them through this module, which mirrors
//! the module tree of `rules_tables`. A table is loaded from the data package
//! (`data/rules_tables/<table id>.json`, [`crate::rules_core::rules_data_package`]) the first
//! time it is read and is cached for the life of the process. A table that cannot be loaded
//! stops the caller with the package error: there is no fallback to a compiled copy.
//!
//! What a module here exports:
//! - **row and id types** (`FeatTableEntry`, `ClassId`, `MonsterStatBlock`, ...) and **pure
//!   functions** (formulas over their arguments): re-exported from where they are defined. A
//!   type is not data.
//! - **tables**: a [`Table`] / [`Derived`] static or a function over the package's rows, named as
//!   the compiled table was, so a call site changes only its module path.
//! - **lookups and views over tables** (`class_chassis_resolve`, `spell_resolve`, `feat_tables`,
//!   `monster_resolve`, ...): the compiled function's own source, reading the tables above.
//!
//! The files below are generated from `rules_tables` by
//! `docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.2_gen_catalog/gen_catalog.py`;
//! `crate::rules_core::rules_tables` is named here, and in the package writer, only.
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::explicit_auto_deref,
    clippy::needless_borrow,
    clippy::type_complexity
)]

#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod equivalence_tests;
#[cfg(test)]
mod override_tests;

use std::ops::Deref;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::rules_core::rules_data_package as pkg;
use crate::rules_core::rules_tables as rt;

/// The module name PF explanation text cites for its tables (Elowen's base attack bonus line reads
/// "Wizard level 5 base attack bonus from <name>, then the crb class table's Wizard row"). That
/// text predates the package, the PF render pins it byte for byte (decisions.md §12.1 SD-i), and
/// an importer must not name a compiled path, so a call site that formats it prints the name from
/// here. Renaming the citation is a PF re-baseline, which only the operator may rule.
pub const COMPILED_MODULE_CITATION: &str = "rules_tables";

/// Rows of table `id`, loaded once per process. A missing or malformed package file is a
/// defect of the install, reported with the file's path.
pub fn rows<T>(id: &'static str) -> &'static [T]
where
    T: Deserialize<'static> + Send + Sync + 'static,
{
    match pkg::table::<T>(id) {
        Ok(rows) => rows,
        Err(error) => panic!("rules_tables package table {id}: {error}"),
    }
}

/// [`rows`] as an owned vector, for the tables a compiled function returned by value.
pub fn rows_vec<T>(id: &'static str) -> Vec<T>
where
    T: Deserialize<'static> + Clone + Send + Sync + 'static,
{
    rows::<T>(id).to_vec()
}

/// A package table read like the compiled `&'static [T]` constant it replaces: it derefs to
/// `[T]`, iterates by reference, and loads on first use.
pub struct Table<T: 'static> {
    id: &'static str,
    cell: OnceLock<&'static [T]>,
}

impl<T: 'static> Table<T> {
    pub const fn new(id: &'static str) -> Self {
        Table {
            id,
            cell: OnceLock::new(),
        }
    }

    /// The table id (`crb/spell_list/SPELL_LIST`).
    pub const fn id(&self) -> &'static str {
        self.id
    }
}

impl<T> Deref for Table<T>
where
    T: Deserialize<'static> + Send + Sync + 'static,
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.cell.get_or_init(|| rows::<T>(self.id))
    }
}

impl<'a, T> IntoIterator for &'a Table<T>
where
    T: Deserialize<'static> + Send + Sync + 'static,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().iter()
    }
}

/// A table built once from package rows (a registry that names other tables), read like a
/// `&'static [T]` constant.
pub struct Derived<T: 'static> {
    build: fn() -> Vec<T>,
    cell: OnceLock<&'static [T]>,
}

impl<T: 'static> Derived<T> {
    pub const fn new(build: fn() -> Vec<T>) -> Self {
        Derived {
            build,
            cell: OnceLock::new(),
        }
    }
}

impl<T: Send + Sync + 'static> Deref for Derived<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.cell
            .get_or_init(|| &*Box::leak((self.build)().into_boxed_slice()))
    }
}

impl<'a, T: Send + Sync + 'static> IntoIterator for &'a Derived<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().iter()
    }
}

pub mod acg;
pub mod advanced_race_guide;
pub mod adventurers_guide;
pub mod apg;
pub mod archetype_swap;
pub mod beastiary1;
pub mod bestiary;
pub mod bestiary_2;
pub mod bestiary_3;
pub mod bestiary_4;
pub mod bestiary_5;
pub mod bestiary_6;
pub mod bonus_bestiary;
pub mod book_of_the_damned_volume_1;
pub mod book_of_the_damned_volume_2;
pub mod class_spell_levels;
pub mod companion_chassis;
pub mod crb;
pub mod equipment_gap_tables;
pub mod feat_gap_tables;
pub mod feats_all;
pub mod horror_adventures;
pub mod inner_sea_bestiary;
pub mod inner_sea_combat;
pub mod inner_sea_faiths;
pub mod inner_sea_gods;
pub mod inner_sea_intrigue;
pub mod inner_sea_magic;
pub mod inner_sea_races;
pub mod inner_sea_temples;
pub mod inner_sea_world_guide;
pub mod monster_chassis;
pub mod monster_codex;
pub mod mythic_adventures;
pub mod occult_adventures;
pub mod pathfinder_unchained;
pub mod simple_kind_tables;
pub mod ultimate_campaign;
pub mod ultimate_combat;
pub mod ultimate_equipment;
pub mod ultimate_intrigue;
pub mod ultimate_magic;
pub mod ultimate_magic_wordsofpower;
pub mod ultimate_psionics;
pub mod ultimate_wilderness;
pub use rt::RuleSetId;
