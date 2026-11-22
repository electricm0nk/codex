# apg_feats.lst — Representative Paizo-LST shape (APG feats)
# Stub for src/rules_core/rules_tables/apg/feats.rs.
# Column-count contract: 8 columns per row.
# Rust resolver key shape: `apg:feat:<lowercase-feat-name>`. Examples below.

[header]
name	category	prerequisites	benefits_short	description_key	normal_chain_chain	prerequisites_or_specific_choice	combat_or_general
[/header]

[feats-general]
Combat reflexes	Combat	Combat Reflexes (no prereq)	extra_aoo_per_dex_bonus	feat_benefit_combat_reflexes	N/A	combat	general
Greater TWF	Combat	Bab +6, Two-Weapon Fighting, Dex 15	additional_iterative_attack_off_hand	feat_benefit_greater_twf	N/A	combat	general
Improved Critical	Combat	Bab 8, Proficient with weapon	chance_to_double_damage_doubling	feat_benefit_improved_critical	N/A	combat, weapon	general
Snake Sidewind	Combat	Dex 13, One-Weapon or Two-Weapon Fighting	retreat_still_aoo	feat_benefit_snake_sidewind	N/A	combat	general
Quicken Spell	Metamagic	Caster Level 4+	spell_as_free_action	feat_benefit_quicken_spell	N/A	metamagic	combat
Heighten Spell	Metamagic	Caster Level 3+	spell_effective_level_plus_one_to_9	feat_benefit_heighten_spell	N/A	metamagic	combat
Reach Spell	Metamagic	Caster Level 1+	spell_range_doubled	feat_benefit_reach_spell	N/A	metamagic	combat
Intensified Spell	Metamagic	Caster Level 6+	spell_effect_dice_cap_double_to_max_level	feat_benefit_intensified_spell	N/A	metamagic	combat
Persuasive	General	None	bribe_combat_bonus_intimidate	feat_benefit_persuasive	N/A	social	general
Child of Stone	General	Half-orc, Earth Mustadle Gnome, or Dwarf ancestry	+2 natural armor vs critical hits	feat_benefit_child_of_stone	N/A	racial	general
Focused Mind	General	Wis 13, Combat Casting	+4 concentration_check_after_damage	feat_benefit_focused_mind	N/A	defensive	general
[/feats-general]

[feats-item-creation]
Craft Wondrous Item	Item Creation	Caster Level 3+	craft_magical_wondrous_item	feat_benefit_craft_wondrous_item	N/A	item_creation	general
Forge Ring	Item Creation	Caster Level 5+	forge_magical_ring	feat_benefit_forge_ring	N/A	item_creation	general
Craft Magic Arms and Armor	Item Creation	Caster Level 5+	craft_magical_weapon_armor_shield	feat_benefit_craft_arms_armor	N/A	item_creation	combat
Craft Wondrous Tattoo	Item Creation (ACG)	Caster Level 5+	craft_tattoo_wondrous	feat_benefit_craft_tattoo_wondrous	N/A	item_creation	general
Brew Potion	Item Creation	Caster Level 3+	brew_potion	feat_benefit_brew_potion	N/A	item_creation	general
[/feats-item-creation]

[feats-racial]
Demoniac Adept	Racial	Demon worship required, Cleric or Sorcerer	archetype_specific_demoniac_aptitude_variants	feat_benefit_demoniac_adept	N/A	racial	general
[/feats-racial]

[feats-acg-convergence]
Many-shot	Combat	Dex 17, Point-Blank Shot, Rapid Shot	shoot_n_arrows_one_attack_round	feat_benefit_many_shot	N/A	combat	general
Rapid-shot	Combat	Dex 13, Point-Blank Shot	additional_attack_two_weapon_or_firearm	feat_benefit_rapid_shot	N/A	combat	general
[/feats-acg-convergence]

# Notes for the parser:
# - APG has ~150-200 feats (depending on printing); this stub ships ~17 representative feats across
#   combat, metamagic, item-creation, racial, and convergence categories. Schema-of-record is fixed;
#   the operator-supplied file may have additional categories (Teamwork feats, Critical feats, etc.).
# - Resolver key shape: `apg:feat:<lowercase-with-hyphens>`.
# - Cross-book invariants:
#     RuleSetId::Apg::resolve("apg:feat:combat-reflexes") returns Some.
#     RuleSetId::Crb::resolve("apg:feat:combat-reflexes") returns None
#       (CRB has a Combat Reflexes feat but it's keyed under crb:feat:combat-reflexes).
#     RuleSetId::Acg::resolve("apg:feat:quick-build") returns None
#       (ACG's Quick Build is keyed under acg:feat:quick-build).
# - Test fixture: tests/sd22_apg_feats_resolves.rs (covers one assertion per category).

# === operator-replace point ===
