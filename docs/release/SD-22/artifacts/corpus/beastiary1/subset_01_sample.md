# beastiary1_subset_01_sample.md — Representative Paizo-Bestiary-1 shape (CR-1 subset, alphabetical)
# Stub for src/rules_core/rules_tables/beastiary1/monster_subset_01.rs to consume.
# Operator replaces body with licensed Bestiary 1 subset 1 (alphabetical monsters at CR 0-1) at cycle-launch.

# Schema: tab-separated rows; # comment leader. Each row is a single monster.
#   name<TAB>cr<TAB>xp<TAB>ac<TAB>initiative<TAB>hp_max<TAB>fort<TAB>ref<TAB>will<TAB>size<TAB>speed<TAB>environment<TAB>alignment<TAB>treasure<TAB>attack_profile<TAB>special_attacks<TAB>special_qualities<TAB>feats

# Column-count contract: FIXED at 18 columns. Loader validates count on parse.

[header]
name	cr	xp	ac	initiative	hp_max	fort	ref	will	size	speed	environment	alignment	treasure	attack_profile	special_attacks	special_qualities	feats
[/header]

[monsters]
Goblin	0.333	400	15	+1	5	+3	+0	+0	Small	30ft	Deserts,Forests,Mountains,Scrublands	CE	Standard short sword +0 (1d6-2); javelin (short spear) +0 (1d4-2);Sneaky	None	Alertness
Kobold	0.25	200	15	+1	4	+0	+3	-2	Small	30ft	Caves,Deserts,Forests,Temperate forests,Marshes,Swamps,Hills,Mountains,Urbans	LE	Standard spear +2 (1d8); sling (range 50 ft) +3 (1d4-2);Sneaky	All-Around Vision,Sensitivity to Sunlight,Trip	---
Orc	0.333	400	14	+1	5	+3	-1	+0	Medium	30ft	Cold deserts,Forests,Hills,Mountains,Scrublands,Temperate forests,Urban,Underground	CE	Greataxe +4 (1d12+2) (Str 13) standard; javelin (short spear) +1 (1d6+1) or (1d8+1) ranged;Sneaky (Ranged)	None	Iron Will
Skeleton	0.333	400	16	+1	4	+0	+0	+2	Medium	30ft	Temperate forests,Any desert,Any forest,Any marsh,Tropic deserts,Mountains,Plains,Underground,Tundra,Urban,Water,Sunken sea caves	NE	Claw +1 (1d4+1) or two claws +1 (1d4+1); two slashes +1 (1d4+1) or by weapons	None	Channel Resistance +4,Damage Reduction 5/bludgeoning,Immunity to Cold,Vulnerable to Bludgeoning	---
Zombie	0.5	600	11	+0	12	+3	+0	+1	Medium	30ft	Any	NE	standard slam +3 (1d6+3); bite +3 (1d4+2); or by weapon	Staggered,Slow and Stupid	None	Undead Traits
[/monsters]

# Notes for the parser:
# - Cross-book invariants from corpus-source-inventory.md §3.2:
#   RuleSetId::Bestiary1::resolve("beastiary1:monster:goblin") returns Some.
#   RuleSetId::Crb::resolve("beastiary1:monster:goblin") returns None (monsters aren't spells/equipment).
#   RuleSetId::Apg::resolve("beastiary1:monster:goblin") returns None.
# - Test fixture: tests/sd22_beastiary1_subset_01_resolves.rs — assertions for each monster's CR/XP
#   at minimum (Goblin: 0.333/400, Kobold: 0.25/200, etc.).
# - This is subset 1 (alphabetical-by-name within CR band, default ordering). Operator-pinned at SD-22
#   cycle-launch; default 8 subsets expand this to subsets 2 through 8.

# === operator-replace point ===
