# beastiary1_monster_templates.lst — Representative Paizo-LST shape (Bestiary 1 monster templates)
# Stub for src/rules_core/rules_tables/beastiary1/monster_templates.rs.
# Column-count contract: 7 columns per row.
# Rust resolver key shape: `beastiary1:template:<lowercase-template-name>`.

[header]
key	template_applied_to	cr_modifier	hp_modifier	feature_swap	description_short	curse_or_affliction	description_key
[/header]

[undead-templates]
skeleton	humanoid_or_quadruped_+0	+0	+0	become_undead_gain_bite_or_claw_curse_or_affliction	undead_resilience_brittle_bones_gain_dr_bludgeoning_or_bow_imp	undead_curse_brittle_bones_dr_curse_etc	description_skeleton_template
zombie	humanoid_with_2_hit_dice_+1	+0	+2_per_hit_die	gain_undead_traits_fortification_partial	brittle_bones_intelligence_score_to_0_takes_dr_5_slashing	undead_curse	partial_zombie	description_zombie_template
ghoul	humanoid_with_4_hit_dice_+2	+0	+2_per_hit_die	gain_giant_paralysis_bite	paralysis_disease_with_high_cr_target	undead_curse	description_ghoul_template
lich	spellcaster_with_5th_level_+4	+30	+30_per_hit_die	retain_spellcasting_feature_unique_curse	phylactery_drain_no_constitution	undead_curse	description_lich_template
vampire	humanoid_with_5_hit_dice_+4	+0	+0	gain_domination_and_energy_drain_curse	no_reflection_sunlight_damage_energy_drain	undead_curse	description_vampire_template
frozen_remain	humanoid_undead_+1	+0	+0	undead_traits_cold_immunity	no_negative_energy_damage_vulnerable_to_fire	undead_curse	description_frozen_remain_template
[/undead-templates]

[construct-templates]
clockwork_construct	construct_+1	+20	+0	gain_dr_or_immunity_keep_special_attack	take_dr_rather_than_weakness_no_sun_drain_no_curse	construct_curse_or_affliction_etc	description_clockwork_template
flesh_golem	humanoid_+2	+0	+0	gain_damage_reduction_berserk_special	berserk_special_dur_damage_golem	description_flesh_golem_template
iron_golem	construct_+4	+0	+0	gain_damage_reduction_weakness_cold	weakness_electricity	aura_dr	description_iron_golem_template
stone_golem	construct_+3	+0	+0	gain_damage_reduction	slowness_against_immune	description_stone_golem_template
animated_object	construct_-1	+0	+0	gain_damage_reduction	no_special_attacks	description_animated_object_template
[/construct-templates]

[dragon-disciple-template]
dragon_disciple	dragonkin_+0	+0	+0	inherit_dragon_draconic_morph_set	breath_weapon_affinity_gain_choice_arcane_advancement_etc.description_dragon_disciple_v1	description_dragon_disciple_template
[/dragon-disciple-template]

[noble-templates]
giant_template	huge_or_larger_with_race_feat_-1	+1_per_HD_or_+10-min	+0	gain_giant_keywords	intimidating_per_size_change	no_race_dup_or_feat_specific_change_per_N_dup	description_giant_template
noble_template	humanoid_+0	+0	+0	gain_noble_birthright_fate	no_special_ability_change	dup_noble_aptitude_armor_intimidating_dup	description_noble_template
[/noble-templates]

# Notes for the parser:
# - Bestiary 1 has roughly 50+ monster templates (undead, construct, dragon-disciple, half-dragon, lycanthrope, etc.).
# - This stub ships 5 undead + 5 construct + 1 dragon-disciple + 2 noble-templates = 13 representative templates.
# - Cross-book invariants:
#     RuleSetId::Bestiary1::resolve("beastiary1:template:skeleton") returns Some.
#     RuleSetId::Apg::resolve("beastiary1:template:skeleton") returns None.
#     RuleSetId::Acg::resolve("beastiary1:template:skeleton") returns None.
#     RuleSetId::Crb::resolve("beastiary1:template:skeleton") returns None (templates are Bestiary-only).
# - Cycle shape: one cycle per template family (undead, construct, dragon-disciple, noble, etc.).
#   Cycle artifacts at:
#     docs/release/SD-22/artifacts/monster-templates/undead_cycle_receipt.md
#     docs/release/SD-22/artifacts/monster-templates/construct_cycle_receipt.md
#     docs/release/SD-22/artifacts/monster-templates/dragon_disciple_cycle_receipt.md
#     docs/release/SD-22/artifacts/monster-templates/noble_cycle_receipt.md

# === operator-replace point ===
