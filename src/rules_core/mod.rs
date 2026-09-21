//! Headless rules-core surfaces.

pub mod archetype_resolver;
pub mod character_input;
pub mod class_census;
pub mod class_feature_pool_catalog;
pub mod class_seeds;
pub mod codex_neutral_name;
pub mod composed_input;
pub mod contract;
pub mod converted_prose;
pub mod corpus_loader;
pub mod damage_total;
// SD-35 `AT-35-E6-003-RULED` cycle 14: the SETTLED result of reading a row's
// bonus chains. Declared live so a live module can hold the answer without
// naming the converter that derived it (`decisions.md` §19, ruling B16).
pub mod declared_bonuses;
pub mod derived_evaluator_fixture_check;
pub mod description_completion;
/// A record description settled into an op list at ingest, and the live renderer that fills it
/// with one character's numbers (SD-35 `AT-35-E6-003-RULED` cycle 16).
pub mod desc_template;
pub mod durability;
pub mod encounters;
pub mod encumbrance;
pub mod equipment_effects;
pub mod equipment_record;
pub mod equipment_resolver;
pub mod feat_effects;
pub mod feat_identity;
pub mod feat_prereqs;
pub mod level_up;
pub mod level_up_option_filter;
pub mod money;
pub mod party_cr;
pub mod pi_screening;
pub mod pi_table_sweep;
pub mod pilot_compute;
pub mod pilot_compute_corpus;
pub mod pilot_failure;
pub mod pilot_view_model;
pub mod race_creation;
// SD-35 `AT-35-E6-003-RULED` cycle 14: the SETTLED canonical race and
// race-trait records, the race-side sibling of `equipment_record`.
pub mod race_record;
pub mod record_vars;
pub mod race_resolver;
pub mod racial_sla;
pub mod rules_tables;
pub mod shape_b_v1;
pub mod sheet_rule;
pub mod sheet_rule_catalog;
pub mod sheet_rule_package;
pub mod size;
pub mod skill_allocation;
pub mod skinwalker_change_shape;
pub mod source_content;
pub mod settled_corpus;
pub mod spell_record;
pub mod spell_resolver;
pub mod spellbook;
pub mod trait_effects;
pub mod trait_pool;
