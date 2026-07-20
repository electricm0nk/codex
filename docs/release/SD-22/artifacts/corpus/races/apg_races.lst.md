# apg_races.lst — Representative Paizo-LST shape (APG races)
# Stub for src/rules_core/rules_tables/apg/race_<race>.rs (one Rust module per race).
# Column-count contract: 12 columns per row. Parser validates count on load.
# Rust resolver key shape: `apg:race:<lowercase-race-name>`. Example: `apg:race:fetchling`.

[header]
name	type	size	base_speed	str_mod	dex_mod	con_mod	int_mod	wis_mod	cha_mod	ability_flaw	race_traits	favored_class_options
[/header]

[races]
Fetchling	outsider	Medium	30 ft	0	2	0	0	0	-2	enchanted_being(faith,resistance)	sorcerer|wizard
Grippli	fey	Small	30 ft	-2	2	0	0	0	2	small_frame(bonus_hide,bonus_perception,gear_specialization)	bard|sorcerer
Kitsune	shapechanger	Medium	30 ft	0	0	0	0	0	2	agile(kitsune_trickster,fey_heritage)	bard|sorcerer
Nagaji	humanoid	reptilian	Medium	30 ft	2	0	0	-2	0	0	poison-resistant,serpentine_senses	sorcerer|wizard
Samsaran	outsider	Medium	30 ft	0	2	0	2	2	-2	psychic_sensitivity,lifebound	sorcerer|wizard
Strix	shapechanger(outside native_plane)	Medium	30 ft	0	2	0	0	0	-2	telepathy(speech_only,30ft),strix_racial_weapon_familiarity	bard|sorcerer
Svirfneblin	extraplanar	Small	20 ft	0	0	2	0	2	-2	spell_resistance(max 11 + level SR bonus),darkvision(60ft),stonecunning	sorcerer|wizard
Wayang	extraplanar	Small	30 ft	-2	2	2	2	0	0	light_blindness,shadow_resistance,hatred	cleric|sorcerer
[/races]

# Notes for the parser (APG races only — operator-replace swaps the body):
# - 8 APG races (Fetchling, Grippli, Kitsune, Nagaji, Samsaran, Strix, Svirfneblin, Wayang)
# - Many columns have nested entries (e.g., `enchanted_being(faith,resistance)` is a comma-list
#   *inside* the `ability_flaw` column). Parser handles nested () by column-specific lexer rules;
#   see src/rules_core/rules_tables/apg/race_parser.rs.
# - Cross-book invariant:
#     RuleSetId::Apg::resolve("apg:race:fetchling") returns Some.
#     RuleSetId::Crb::resolve("apg:race:fetchling") returns None (Fetchling is APG-only).
#     RuleSetId::Acg::resolve("apg:race:fetchling") returns None.
#     RuleSetId::Bestiary1::resolve("apg:race:fetchling") returns None (race keys and monster keys don't share the resolver keyspace).
# - Test fixture: tests/sd22_apg_race_resolves.rs (one cycle per APG race; can be batched into one cycle for all 8).

# === operator-replace point ===
EOF
ls -la docs/release/SD-22/artifacts/corpus/races/