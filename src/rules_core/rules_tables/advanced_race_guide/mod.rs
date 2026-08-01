//! Advanced Race Guide (ARG) table data. SD-27 Cycle E2.1/E2.2 per-book
//! pre-build — sibling directory to `rules_tables::apg`/`rules_tables::acg`/
//! `rules_tables::crb`, mirroring `rules_tables::acg`'s file layout/naming
//! (closer in scale to this book's real content than CRB's much larger
//! set).
//!
//! **Scope (per this cycle's execution brief).** In scope: the 5 LST files
//! that follow the established simple-record pattern
//! `class_tables`/`spell_list`/`equipment_tables.rs` already handle across
//! every ingested book — `arg_spells.lst` (spells), `arg_equip_arms_armor.lst`
//! / `arg_equip_general.lst` / `arg_equip_magic_items.lst` / `arg_equipmods.lst`
//! (equipment), and `arg_feats.lst` (feats). See `spell_list.rs`,
//! `equipment_tables.rs`, and `feats.rs` for each content kind's own
//! doc comment, including the real, independently re-verified record
//! counts (which differ from the cycle's scoping-brief rough estimates —
//! documented plainly in each module rather than silently reconciled).
//!
//! **Deliberately out of scope, this cycle:** `arg_abilities_class.lst`
//! (792 records), `arg_abilities_race.lst` / `arg_abilities_builder.lst` /
//! `arg_races.lst` / `arg_races_companion.lst` / `arg_templates.lst`
//! (racial trait + race-builder content). This content uses PCGen's
//! low-level ability/`BONUS:`/`DEFINE:`/`PREREQ:` formula-engine syntax —
//! no book in this codebase, including CRB itself, has ever represented
//! this content type in the Shape B JSON cache (CRB's own `class_tables.rs`
//! only holds BAB/save-formula summaries, never full ability trees; no
//! book's `data/corpus/` has ever had a `race/` content-kind directory).
//! Building a faithful cache for this content would require inventing a
//! new formula-parsing architecture with no precedent to follow — genuinely
//! out of this cycle's bounded scope, not corner-cutting.
//!
//! **Registered in the shared module tree.** Originally landed compiled
//! only via `#[path]` inclusion into `src/bin/sd27_gen_book_cache.rs`,
//! because SD-27's per-cycle file-touch partition
//! (`docs/release/SD-27-future-state-book-content-ingestion/loop-instruction.md`
//! §6) allow-listed `src/rules_core/rules_tables/advanced_race_guide/` but
//! not `src/rules_core/rules_tables/mod.rs`. A later cycle that did own
//! `rules_tables/mod.rs` registered this module as
//! `pub mod advanced_race_guide;` and added the `RuleSetId::Arg` variant,
//! so it is now reachable through the `codex` library's public module tree
//! as well as through the generator binary's `#[path]` include. ARG
//! contributes no player class of its own either way (its content is
//! spells/equipment/feats layered onto existing classes/races), so unlike
//! `rules_tables::acg::mod`, there is no `class_chassis_resolve` here.

pub mod class_spell_levels;
pub mod equipment_data;
pub mod equipment_tables;
pub mod feat_data;
pub mod feats;
pub mod json_cache;
pub mod spell_list;
