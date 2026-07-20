# acg_mitems.lst — Representative Paizo-LST shape (ACG magic items)
# Stub for src/rules_core/rules_tables/acg/magic_items.rs.
# Column-count contract: 8 columns per row. Same shape as APG.
# Rust resolver key shape: `acg:mitem:<key>` (lowercase-with-hyphens).

[header]
key	item_name	item_slot	cost	gp	slot_required	slot_optional	body_aura	body_activation_use	body_weight	item_description_key
[/header]

[wondrous-items]
blood_drinker_boots	Boots of Arcane Persuasion — ACG archetype specialization	Feet	12000	gp	feet	—	moderate_divination	passive	4lb	item_mitem_acg_blood_drinker_boots_charisma_arcane
cloak_of_fortune	Cloth of the Unspeakable (fortune-by-circumstance-bonus)	Head	35000	gp	shoulders	head	moderate_divination	1_round	1lb	item_mitem_acg_cloak_fortune_curse_to_replace
dead_hand_gloves	Hand of the Mighty (Dead)	Hands	8000	gp	hands	—	moderate_necromancy	standard-action	2lb	item_mitem_acg_dead_hand_channel_neg_levels_one_per_day
dawnflower_charm	Badge of the Bear (variant with dawnflower alignment-affinity)	Neck	5000	gp	neck	head	moderate_transmutation	passive	1lb	item_mitem_acg_dawnflower_charm_animal_companion_emblem_dual_boost
gem_of_insight	Gem of Brightness (Insight-Focused)	Hand	25000	gp	held	—	strong_divination	standard-action	0lb	item_mitem_acg_gem_insight_arcane_sight_once_per_day
hospitality_soothsayer_panel	Chime of Opening (ACG door-class)	Bandolier	60000	gp	bandolier	—	strong_conjuration	standard-action	2lb	item_mitem_acg_hospitality_chime_open_lock_arcane_servant_dup_serv_3d6
knights_periapts	Apostle's Crown	Head	9000	gp	head	—	moderate_evocation	passive	1lb	item_mitem_acg_apostles_crown_channel_positive_energy_one_per_day
magmas_heartstone_kinetics_seal	Heartstone of the Fire Lord	Neck	18000	gp	neck	—	moderate_evocation	standard-action	1lb	item_mitem_acg_heartstone_fire_immunity_dr_10_fire
minuscule_chronicle_strip	Watch of the Wasted Hours (time-debuff cure)	Body	4000	gp	neck	body_via_necklace	—	moderate_conjuration	standard-action	1lb	item_mitem_acg_watch_wasted_hours_one_save_dup_via_perception
[/wondrous-items]

[weapons]
adamantine_composite_shortbow	Composite Shortbow (Adamantine, +1)	Two-hand	8000	gp	two-hand	—	moderate_abjuration	passive	2lb	item_mweapon_acg_composite_shortbow_against_dragon
bashing_thorn_strip	Brass Knuckles of the Brawler (DG-stripped variant)	Hands	low_cost	gp	hands	—	faint_transmutation	passive	1lb	item_mweapon_acg_bashing_thorn_brawler_focus
fountainhead_quarterstaff	Staff of the Magus (Fountainhead Lead)	Two-hand	low_cost	gp	two-hand	—	moderate_divination	standard-action	4lb	item_mweapon_acg_fountainhead_quarterstaff_knowledge_arcana
vorpal_dagger	Dagger of Blade-Feints (Magus-style)	Off-hand	20000	gp	off-hand	—	moderate_illusion	standard-action	1lb	item_mweapon_acg_dagger_blade_feints_magus_spell_trigger
[/weapons]

[armor]
scaled_plate	Adamantine Plate Mail (Scaled)	Body	18000	gp	body	—	moderate_abjuration	passive	50lb	item_marmor_acg_scaled_plate_dr_3_plus_scaled_layer
[/armor]

# Notes for the parser:
# - ACG has a smaller magic items roster than APG; representative here covers 12 wondrous + 4 weapons + 1 armor.
# - Schema-of-record is identical to APG. Parser uses the same lexer+state-machine; resolver chain branches on
#   the apg:/acg: key prefix.
# - Cross-book invariants:
#     RuleSetId::Acg::resolve("acg:mitem:cape_of_fortune") returns Some.
#     RuleSetId::Apg::resolve("acg:mitem:cloak_of_fortune") returns None.
#     RuleSetId::Crb::resolve("acg:mitem:cloak_of_fortune") returns None.
# - Cycle shape: same aisle-grouped cycles as APG.

# === operator-replace point ===
