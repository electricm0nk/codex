# apg_mitems.lst — Representative Paizo-LST shape (APG magic items)
# Stub for src/rules_core/rules_tables/apg/magic_items.rs (one row per item;
# cycle artifact per aisle group, not per item).
# Column-count contract: 8 columns per row.
# Rust resolver key shape: `apg:mitem:<key>` (lowercase-with-hyphens).

[header]
key	item_name	item_slot	cost	gp	slot_required	slot_optional	body_aura	body_activation_use	body_weight	item_description_key
[/header]

[wondrous-items]
cape-of-feathers	Feather Tokens, Slippers of Spider Climbing — see CRB analogue	Cape	1400	gp	neck	shoulders	moderate_perception_or_hide	standard-action	N/A	item_mitem_apg_fall_avoidance_100ft
healing-potion-belt	Belt of Healing	Waist	12000	gp	waist	—	moderate_conjuration	standard-action	4lb	item_mitem_apg_healing_2d8_per_day_1_5hd
pipes-of-souls_dup	Pipes of the Sewers	Mouth	8000	gp	throat	—	moderate_divination	standard-action	2lb	item_mitem_apg_pipes_summon_rat_swarm_dup
adamantine-armor	Treant-Bonded Composite Longbow	Body	5000	gp	body	—	moderate_transmutation	passive	weight_varies	item_mitem_apg_adamantine_armor_dr_3
amulet_of_focused_might	Amulet of Mighty Fists (Magical Beasts)	Neck	4000	gp	neck	—	moderate_transmutation	standard-action	1lb	item_mitem_apg_amulet_enhance_fist_attack_1_to_5
bracers_of_archery_2	Bracers of Falcon's Aim (companion)	Arms	8000	gp	bracers_slot	—	moderate_divination	passive	2lb	item_mitem_apg_bracers_archery_composite_bow_bonus_dup
bracers_of_armor	Bracers of Armor	Arms	low-cost	gp	varies	—	faint_abjuration	passive	weight_varies	item_mitem_apg_bracers_armor_deflection_2_to_8
brooch_of_shielding	Brooch of Shielding (Jr)	Neck	1500	gp	neck	chest_neck_slot	faint_abjuration	passive	0lb	item_mitem_apg_brooch_shielding_5_per_day_max_5hp
candle_of_truth	Candle of Truth (sustain)	Hand	1500	gp	held	—	faint_evocation	1_minute	1lb	item_mitem_apg_candle_truth_save_bonus_vs_illusion
cats-eye_tablets	Cloak of the Bat (BAT-form entry)	Shoulders	4500	gp	back	—	moderate_transmutation	standard-action	1lb	item_mitem_apg_cats_eye_darkvision_30ft
cloak_of_arachnida	Cloak of Arachnida	Shoulders	18000	gp	shoulders	—	moderate_conjuration	standard-action	1lb	item_mitem_apg_cloak_arachnida_climb_30ft_perception
deck_of_illusions_fate_fool	Deck of Illusions	Quick-access	800	gp	belt_pouch	hand	faint_illusion	full-round	1lb	item_mitem_apg_deck_illusions_random_summon
elven_circlet_red	Headband of Focused Aim (signature variant)	Head	4000	gp	head	—	moderate_divination	passive	0lb	item_mitem_apg_elven_circlet_mental_plus_2_to_2_uses
gloves_of_locksmithing	Gloves of the Locksmith (skill-focused)	Hands	300	gp	hands	—	rare_to_no_resonance	standard-action	0lb	item_mitem_apg_gloves_locksmith_disable_device_5
horn_of_blasting_small	Horn of Blasting	Sling	200	gp	held	—	faint_evocation	standard-action	2lb	item_mitem_apg_horn_blasting_5d6_max_2_uses
iron_variora_slime_companion	Iron Beads of Haste	Mouth	1500	gp	throat	—	moderate_divination	standard-action	0lb	item_mitem_apg_iron_beads_haste_one_round_one_use
lagomorph_seal_scroll_strip	Scroll of Tireless Pursuit — see Scroll Material section	Varies	125	gp	minor_scroll_tube	—	faint_conjuration	1_minute	0lb	item_mitem_apg_lagomorph_seal_dash_thrice_per_day_4_seconds_dup_lc_bonus
magicians_hat	Hat of Disguise (Self-Only)	Head	1500	gp	head	shoulders	moderate_illusion	standard-action	1lb	item_mitem_apg_magicians_hat_disguise_self_at_will_per_dur_30min
mauler_belt	Belt of Restoration (Limited)	Waist	6000	gp	waist	—	moderate_conjuration	standard-action	1lb	item_mitem_apg_mauler_belt_negate_neg_levels_once_per_day_x_per_day
mirovanech_pantheon_medallion	Medallion of Thought (Trickery)	Neck	1800	gp	neck	—	minor_illusion_passive	passive	0lb	item_mitem_apg_medallion_thought_save_bonus_illusion_three_per_day
[/wondrous-items]

[weapons]
adamantine_axe	Greataxe of Defending (Adamantine)	Main-hand	30000	gp	main-hand	off-hand	moderate_abjuration	passive	8lb	item_mweapon_apg_axe_deflection_1_dup_parry_disarm_negated_compact
cold-iron_longsword_of_puncture	Longsword of Puncturing (Cold-Iron)	Main-hand	6500	gp	main-hand	—	faint_transmutation	passive	4lb	item_mweapon_apg_longsword_puncture_plus_1d6_ranged_attack_misses
ghost_touch_dagger	Dagger of Ghost Touch (Cold-Iron)	Off-hand	10000	gp	off-hand	—	faint_necromancy	passive	1lb	item_mweapon_apg_ghost_touch_incidental_dam_dup_intang_dup
dragonbane_spear	Spear of Dragonbane (adamantine)	Two-hand	25000	gp	two-hand	—	moderate_abjuration	passive	6lb	item_mweapon_apg_dragonbane_dragon_drach_neutral_dup_scaled
[/weapons]

[armor]
dragonplate_barding	Dogbearing Full Plate (Dragon)	Barding	20000	gp	mount_armor	—	moderate_transmutation	passive	50lb	item_marmor_apg_dragonplate_dragon_drach_neutral
mithral_chain_shaded	Mithral Chain (Shaded)	Body	4500	gp	body	—	moderate_illusion	passive	25lb	item_marmor_apg_mithral_chain_no_dex_check_shaded_sanct_dup_arcanist_no_prohibit
[/armor]

# Notes for the parser (per operator directive 2026-07-19):
# - APG has approximately 200+ magic items; this stub ships ~20 representative items across the load-bearing surfaces
#   (wondrous, weapons, armor). Rust parser doesn't enumerate every item; it ingests whatever the operator-supplied
#   file contains. The schema-of-record is fixed; the licensed data is operator-supplied.
# - Resolver key shape: `apg:mitem:<lowercase-hyphenated>`. Examples above.
# - Cross-book invariants:
#     RuleSetId::Apg::resolve("apg:mitem:cape_of_feathers") returns Some.
#     RuleSetId::Crb::resolve("apg:mitem:cape_of_feathers") returns None (Feather Tokens are APG-specific enhancement).
#     RuleSetId::Acg::resolve("apg:mitem:cape_of_feathers") returns None.
#     RuleSetId::Bestiary1::resolve("apg:mitem:cape_of_feathers") returns None (mitems and monsters have distinct keyspaces).
# - Cycle shape: one cycle per aisle (wondrous / weapons / armor). Cycle artifacts at:
#     docs/release/SD-22/artifacts/magic-items/apg_wondrous_aisle_cycle_receipt.md
#     docs/release/SD-22/artifacts/magic-items/apg_weapons_aisle_cycle_receipt.md
#     docs/release/SD-22/artifacts/magic-items/apg_armor_aisle_cycle_receipt.md
# - Test fixture: tests/sd22_apg_mitems_resolves.rs (covers one assertion per aisle).

# === operator-replace point ===
