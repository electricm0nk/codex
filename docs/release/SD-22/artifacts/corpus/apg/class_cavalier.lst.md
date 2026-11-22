# apg_class_cavalier.lst — Representative Paizo-LST shape (cavalier, Advanced Player's Guide)
# Stub for src/rules_core/rules_tables/apg/class_cavalier.rs to consume at RED-phase cycle.
# Operator replaces body with licensed Paizo APG Cavalier LST at cycle-launch.
# Schema: tab-separated, # comment leader, three sections per class.

[header]
name	class_base_features_key	bab	save	starting_gold	hd	skill_list_key	class_skills	level_table_key
Cavalier	apg_class_cavalier_v1	high	fort_ref_will	4d6*10	1d10	cavalier_skill_list	<RIDE, Diplomacy, Intimidate, Linguistics, ... , Use Magic Device>	<feat_table_key>

[/header]

[level_features]
1 Base feature
5 Mid feature
10 Capstone
[/level_features]

# Notes for the parser (mirror of corpus-source-inventory.md §1.1 row, Cavalier):
# - Cross-book invariants from corpus-source-inventory.md §1.3:
#   RuleSetId::Apg::resolve("apg:class:cavalier") returns Some — this is a stub.
#   RuleSetId::Crb::resolve("apg:class:cavalier") returns None.
# - Test fixture: tests/sd22_apg_class_cavalier_resolves.rs (one cycle per APG class).

# === operator-replace point ===
