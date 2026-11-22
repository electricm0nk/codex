# acg_feats.lst — Representative Paizo-LST shape (ACG feats)
# Stub for src/rules_core/rules_tables/acg/feats.rs.
# Column-count contract: 8 columns per row. Same shape as APG.
# Rust resolver key shape: `acg:feat:<key>`.

[header]
name	category	prerequisites	benefits_short	description_key	normal_chain_chain	prerequisites_or_specific_choice	combat_or_general
[/header]

[feats-general-combat]
Arcane Strike	Combat	Caster Level 1+, Arcane pool class feature or similar	additional_damage_to_weapon_attacks_after_cast	feat_benefit_arcane_strike	acg|path_of_sacred_fury	cast	combat
Spell Penetration	Combat (ACG consolidation)	Caster Level 1+	+2_to_spell_resistance_defense	feat_benefit_spell_pen	acg	cast	combat
Greater Spell Penetration	Combat	Caster Level 9+, Spell Penetration	+4_to_spell_resistance_defense	feat_benefit_greater_spell_pen	acg	cast	combat
Quick Draw	Combat (no prereq)	dex_13	free_action_draw_equip	feat_benefit_quick_draw	acg	dex	combat
Combat Versatility	Combat	Slayer class feature (ACG)	switch_versatile_training_one_round_two_weapon_blade_dup	feat_benefit_combat_versatility	N/A	rogue_talent	combat
Quick Beast-Master	Combat (ACG)	Beast-Mastery archetype of Hunter	beast_companion_full_action	feat_benefit_quick_beast_master	acg	hunter_archetype	general
Improved Defensive Combat Training	Combat	Archer archetype of Fighter (ACG)	off_target_when_horse_to_mounted_attack_dup	feat_benefit_improved_defensive_combat_training	N/A	fighter_archetype	combat
[/feats-general-combat]

[feats-magic-discoveries]
Extra Magus Arcana (Quickening Master)	Discovery (ACG)	Magus	class_arcana_feature_multi-dispatch	feat_benefit_extra_magus_arcana	N/A	magus	general
Extra Bloodrage (Kineticist-ish)	Discovery (ACG)	Bloodrager	class_feature_extra_bloodrage_one_per_day	feat_benefit_extra_bloodrage	N/A	bloodrager	general
[/feats-magic-discoveries]

[feats-item-creation-acg]
Craft Wondrous Tattoo	Item Creation	Caster Level 5+ (ACG's variant of Craft Wondrous Item)	craft_tattoo_wondrous	feat_benefit_craft_tattoo_wondrous	N/A	item_creation	general
[/feats-item-creation-acg]

# Notes for the parser:
# - ACG has ~80-100 feats (depending on printing); this stub ships ~10 representative feats across
#   combat, discovery, and item-creation categories.
# - ACG also has archetype-tree feats (Brawler's martial-flexibility feats, Bloodrager's bloodline-feats,
#   Hunter's animal-companion-boost feats, etc.); those cycle under Epic 4's per-class work.
# - Cross-book invariants:
#     RuleSetId::Acg::resolve("acg:feat:arcane-strike") returns Some.
#     RuleSetId::Apg::resolve("acg:feat:arcane-strike") returns None (AcG has its own).
#     RuleSetId::Crb::resolve("acg:feat:arcane-strike") returns None.

# === operator-replace point ===
