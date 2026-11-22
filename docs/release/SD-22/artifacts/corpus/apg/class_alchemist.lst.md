# apg_class_alchemist.lst — Representative Paizo-LST shape (Alchemist, Advanced Player's Guide)
# This is the file-shape stub for src/rules_core/rules_tables/apg/class_alchemist.rs to consume.
# Operator replaces the body of this file with the licensed Paizo APG Alchemist LST at cycle-launch.
# Schema: tab-separated rows, with # as comment leader; first non-comment line is the field-list header.

# row schema (tab-separated, fixed column order):
#   name<TAB>class_base_features_key<TAB>bab_progression<TAB>save_progression<TAB>starting_gold<TAB>hd_die<TAB>skill_list_key<TAB>class_skills<TAB>level_table_key
# follower rows are feature progression at level 1, 2, ..., 20:
#   level<TAB>feature_key<TAB>uses_per_day<TAB>suppress_or_replaces<TAB>notes
# spell-list cross-refs are at the tail:
#   spell-level<TAB>spells-per-day <TAB>extracts-known<TAB>discovery-known<TAB>mutagen-bomb-mutex-at-level<TAB>discovery-count-at-level

[header]
name	class_base_features_key	bab	save	starting_gold	hd	skill_list_key	class_skills	level_table_key
Alchemist	apg_class_alchemist_v1	medium	fort_ref_will	3d6*10	1d8	alchemist_skill_list	Appraise,Craft(Alchemy),Disable Device,Heal,Knowledge(Arcana),Knowledge(Nature),Perception,Profession,Sleight of Hand,Spellcraft,Use Magic Device	caster_extract_6
[/header]

[level_features]
level	feature_key	uses_per_day	suppress_or_replaces	notes
1	alchemist_class_feature	1	none	alchemy; bombs_su_lvl1; brew_potion_su; throw_anything_ex_lvl1; poison_resistance_ex_lvl1; mutagen_su
2	alchemist_class_feature	1	none	discoveries_su_at_lvl2; bomb_discovery_acid
3	alchemist_class_feature	1	none	+
4	alchemist_class_feature	1	none	swift_alchemy_su_lvl4
5	alchemist_class_feature	1	none	+
6	alchemist_class_feature	1	none	discoveries_su_at_lvl2
7	alchemist_class_feature	1	none	+
8	alchemist_class_feature	1	none	swift_alchemy_su_lvl4
9	alchemist_class_feature	1	none	+
10	alchemist_class_feature	1	none	+
11	alchemist_class_feature	1	none	+
12	alchemist_class_feature	1	none	+
13	alchemist_class_feature	1	none	+
14	alchemist_class_feature	1	none	+
15	alchemist_class_feature	1	none	+
16	alchemist_class_feature	1	none	swift_alchemy_su_lvl4
17	alchemist_class_feature	1	none	+
18	alchemist_class_feature	1	none	+
19	alchemist_class_feature	1	none	+
20	alchemist_class_feature	1	none	grand_discovery_su_lvl20
[/level_features]

[spells]
spell-level	spells-per-day	extracts-known	discovery-known	mutagen-bomb-mutex-at-level	discovery-count-at-level
1	0	0	0	1	0
2	0	1	0	1	1
3	1	1	0	1	1
4	1	1	1	1	1
5	2	1	1	1	1
6	2	2	1	1	2
7	3	2	1	1	2
8	3	2	2	1	2
9	3	3	2	1	3
10	3	3	2	1	3
11	4	3	2	1	3
12	4	4	3	1	4
13	4	4	3	1	4
14	4	4	3	1	4
15	5	4	3	1	5
16	5	5	4	1	5
17	5	5	4	1	5
18	5	5	4	1	6
19	6	5	4	1	6
20	6	6	5	1	6
[/spells]

# Notes for the parser (load this file via the cycle's `corpus_input_path`):
# - The column counts in [header], [level_features], [spells] are FIXED. The parser validates
#   the count on load and fails the cycle's RED phase if columns don't align with this stub.
# - Feature counts at level N follow canonical PF1 APG Alchemist: discoveries_at_lvl_2 + 1 per 2 levels;
#   mutagen+weapon_focus(bomb) at level 1; bomb_cap_die_by_level follows the published APG
#   table (1d6→2d6 at level 3, 3d6 at level 5, 4d6 at level 7, +1 die per 2 levels thereafter).
# - Operationally, cycle 1 writes tests/sd22_apg_class_alchemist_resolves.rs with assertions for:
#   1. RuleSetId::Apg::resolve("apg:class:alchemist") returns Some(alchemist canonical record)
#   2. RuleSetId::Crb::resolve("apg:class:alchemist") returns None
#   3. The alchemist_class_alchemist.discovery_known_at_level(6) returns 2 (per row above)
#   4. The mutagen_bomb_mutex_at_level flag holds for all levels (mutex semantics)
#   5. bomb_cap_die(7) returns 4d6 (per the published APG table)

# === operator-replace point ===
# The Paizo APG Alchemist LST file (licensed content) replaces everything between this row
# and "[/spells]" above. The Rust parser's loader only depends on the column-count contract,
# so the licensed body parses cleanly against the same schema once the operator drops it in.
