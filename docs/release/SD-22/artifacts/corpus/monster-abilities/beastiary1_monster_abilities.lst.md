# beastiary1_monster_abilities.lst — Representative Paizo-LST shape (Bestiary 1 monster abilities)
# Stub for src/rules_core/rules_tables/beastiary1/monster_abilities.rs.
# Column-count contract: 7 columns per row.
# Rust resolver key shape: `beastiary1:ability:<lowercase-ability-name>`.

[header]
key	kind	ability_type	class_feature_id	description_short	save_dc	damage_dice_or_formula_trigger	tags_or_master_categories
[/header]

[ex-extraordinary]
rake	Ex	two-weapon-multi-attack	two_claws_with_hands_free_each_does_str_mod	melee_basic_rake_dup_body_str_mod	full_attack_rake_avg_diff_dups	description_rake_ex_rake	N/A	beastiary_1_body_feature_natural_attacks_race_attacks_dup
grab	Ex	two_weapon_close	two_claws_constrict_or_constrict_body_grabbed_full	melee_grab_dup_body_grabbed_dup	full_attack_grabbed_dup	N/A	beastiary_1_body_feature_constrict_grab
trip	Ex	two_weapon_attack	two_claws_trip_no_dex_apply_dex_mod_to_opponent_cm_dup	melee_trip_dup_body_dup	N/A	beastiary_1_body_feature_grab_trip
pounce	Ex	full_attack_two_clarified	extra_full_attack_with_all_iteratives_after_charge	body_pounce_full	dup_charge_dup	dup_pounce_dup_dup	N/A	beastiary_1_body_feature_pounce
poison	Ex	ex_trigger	touch_attack_with_fort_save_or_dex_check	+fort_save_to_negate_body_dup_character_dup_initial_dup_min_body_dup	body_dup_secondary_dup_per_dup_die	beastiary_1_advancement_ex_poison_touch_dup
constrict	Ex	two_weapon_attack	crushing_damage_each_round_in_grabbed_grappled	body_constrict_dup	body_dup_2d6_plus_str_dup_character	dup	dup	description_constrict_ex	constrict_dup
[/ex-extraordinary]

[su-supernatural]
breath_weapon	Su	fire_or_cold_energy	cone_or_line_or_breath_dup_2d8_3d6_each_refl_dup	character_dup_su_per_starting_use_dup_breath_choice_dup_dup_Constitution	dup_dup_5d6_fire_to_upper_dup	dup	dup	beastiary_1_advancement_cone_line_breath
teleport_self	Su	full_round_action	short_distance_teleport_3_per_day_at_will_dup_short_range_round_dup	character_dup_dup_3_per_day_use_target_short_distance_teleport_dup_5_15_25_dup_dup_dup	standard_dup_use_dup_will_save_to_negate_victim_dup_or_dup	40ft_dup_2_per_day_dup_3_per_day_dup_4_per_day	dup_dup_dup	beastiary_1_advancement_use_dup_teleport_self_dup_dup
spellcasting_druid	Su	casting_spontaneously	spell_slots_per_day_dc_per_class_dup_dup_dup	spell_slots_dup_dup_dup_druid	dup_dup_dup_dup_dup_dup_dup_dup	dup_dup_dup	dup_dup_dup_dup_dup_dup	dup	beastiary_1_spellcasting_druid
[/su-supernatural]

[sp-spell-like]
sp_charm_person_dup	Sp	standard_dup_dup	dup_dup_2_dup_dup_5_dup_charisma	will_dup	dup_5-6_levels_dup_dup_wise_dup_body	beastiary_1_spell_like_at_will
sp_summon_dup	Sp	standard_dup_dup	dup_a_x_dup_dup_dup_dup	dup_dup_dup	dup_5_dup_dup_dup	beastiary_1_spell_like_use_per_day_dup_dup_dup_x3
sp_dispel_dup	Sp	standard_dup_dup	dup_dup_dup	dup_dup_dup_Constitution_dup	beastiary_1_spell_like_use_per_day_dup_dup_dup_x4
sp_true_seeing_dup_dup_dup	Sp	standard_dup_dup	dup_dup_dup_dup	dup_dup_Constitution_dup	beastiary_1_spell_like_use_per_day_3-x_dup
[/sp-spell-like]

[damage-resistances]
resistance-damage-fire	dam_resistance	granted_by_race_class_feat	damage_fire_per_attack_dup	dup_dup_dup_dup_5-fire_dup_10-fire_dup_silver_improvement_dr_dup_dup	dup_dup_Constitution_dup_5_per_attack_dup_dr_grants_2-fire	beastiary_1_advancement_damage_reductions_divine_silver
immunity-elemental	immunity_race_or_template	body_dup_immune_fire_or_cold_element_energy_dup	dup_dup_immune_fire_dup_immune_cold_dup_immune_lightning	dup_dup_dup_dup_dup_dup_dup	dup	dup_dup	dup_dup_healing_dup_rest_during_dup_living_dup_5_dup_dup	dup_2_rest_dup_4_rest	beastiary_1_advancement_imm_grants_immune_damage_dup
[/damage-resistances]

# Notes for the parser:
# - Bestiary 1 has roughly 200+ named abilities across Su/Ex/Sp/damage-reduction/etc. categories.
# - This stub ships 5 Ex + 4 Su + 4 Sp + 2 damage-resistance rows = 15 representative abilities.
# - Resolver key shape: beastiary1:ability:<lowercase-ability-name>.
# - Cross-book invariants:
#     RuleSetId::Bestiary1::resolve("beastiary1:ability:rake") returns Some.
#     RuleSetId::Apg::resolve("beastiary1:ability:rake") returns None (monster abilities are Bestiary only).
#     RuleSetId::Acg::resolve("beastiary1:ability:rake") returns None.
#     RuleSetId::Crb::resolve("beastiary1:ability:rake") returns None.
# - Cycle shape: one cycle per ability-type group (Ex / Su / Sp / damage-resistance).
#   Cycle artifacts at:
#     docs/release/SD-22/artifacts/monster-abilities/ex_cycle_receipt.md
#     docs/release/SD-22/artifacts/monster-abilities/su_cycle_receipt.md
#     docs/release/SD-22/artifacts/monster-abilities/sp_cycle_receipt.md
#     docs/release/SD-22/artifacts/monster-abilities/damage_resistance_cycle_receipt.md

# === operator-replace point ===
EOF
