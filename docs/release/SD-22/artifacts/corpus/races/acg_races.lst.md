# acg_races.lst — Representative Paizo-LST shape (ACG races)
# Stub for src/rules_core/rules_tables/acg/race_<race>.rs (one Rust module per race).
# Column-count contract: 12 columns per row. Same shape as APG races.
# Rust resolver key shape: `acg:race:<lowercase-race-name>`.

[header]
name	type	size	base_speed	str_mod	dex_mod	con_mod	int_mod	wis_mod	cha_mod	ability_flaw	race_traits	favored_class_options
[/header]

[races]
Dhampir	humanoid(undead-blooded)	Medium	30 ft	0	2	0	0	0	2	undead_heritage,sunlight_sensitivity	bloodrager|magus|rogue|alchemist|inquisitor|cleric|oracle|sorcerer|wizard
Duergar(fighter_archetype_specialization)	dwarf	Medium	25 ft	0	0	2	0	0	-2	duergar_psychic_focus,slayer,sunlight_sensitivity	druid|cavalier|cleric|fighter|rogue|alchemist|inquisitor|sorcerer|wizard
Forlorn	half-elf	Medium	30 ft	0	0	0	2	2	-2	elven_immunities_refined,psychic_disquiet	alchemist|inquisitor|oracle|sorcerer|witch
Half-orc(or_doom-shifter)	humanoid	Medium	30 ft	2	0	2	-2	-2	0	darkvision(60ft),intimidating,orc_ferocity,sacred_tattoo	bloodrager|cleric|fighter|oracle|rogue|sorcerer|magus|brawler|inquisitor|warpriest
Skeleton(undead_heritage)	undead	Medium	30 ft	0	0	0	0	1	-2	undead_traits,arcane_spellcasting_lost,brittle_bones	sorcerer|magus|inquisitor|oracle|summoner|witch|cleric
Undying(lich-feathered_undead)	humanoid(undead-blooded)	Medium	30 ft	0	0	0	0	2	0	undaunted,undead_resilience_necrotic	sorcerer|oracle|inquisitor|warpriest|cleric|magus|witch
[/races]

# Notes for the parser (ACG races only — operator-replace swaps the body):
# - 6 ACG races (Dhampir, Duergar-fighter-specialization, Forlorn, Half-Orc-Doom-Shifter, Skeleton, Undying)
# - Skeleton is the ACG version; the Bestiary 1 version is a monster under RuleSetId::Bestiary1.
# - Each `<lowercase-race-name>` resolver key maps to exactly one Rust module.
# - Cross-book invariants:
#     RuleSetId::Acg::resolve("acg:race:dhampir") returns Some.
#     RuleSetId::Apg::resolve("acg:race:dhampir") returns None.
#     RuleSetId::Crb::resolve("acg:race:dhampir") returns None.
# - Test fixture: tests/sd22_acg_race_resolves.rs (one cycle per ACG race).

# === operator-replace point ===
EOF
echo "RACES:" && ls docs/release/SD-22/artifacts/corpus/races/