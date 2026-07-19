# apg_class_inquisitor.lst — Representative Paizo-LST shape (inquisitor, Advanced Player's Guide)
# Stub for src/rules_core/rules_tables/apg/class_inquisitor.rs to consume at RED-phase cycle.
# Operator replaces body with licensed Paizo APG Inquisitor LST at cycle-launch.
# Schema: tab-separated, # comment leader, three sections per class.

[header]
name	class_base_features_key	bab	save	starting_gold	hd	skill_list_key	class_skills	level_table_key
Inquisitor	apg_class_inquisitor_v1	high	fort_ref_will	4d6*10	1d10	inquisitor_skill_list	<RIDE, Diplomacy, Intimidate, Linguistics, ... , Use Magic Device>	<feat_table_key>

[/header]

[level_features]
1 Base feature
5 Mid feature
10 Capstone
[/level_features]

# Notes for the parser (mirror of corpus-source-inventory.md §1.1 row, Inquisitor):
# - Cross-book invariants from corpus-source-inventory.md §1.3:
#   RuleSetId::Apg::resolve("apg:class:inquisitor") returns Some — this is a stub.
#   RuleSetId::Crb::resolve("apg:class:inquisitor") returns None.
# - Test fixture: tests/sd22_apg_class_inquisitor_resolves.rs (one cycle per APG class).

# === operator-replace point ===
