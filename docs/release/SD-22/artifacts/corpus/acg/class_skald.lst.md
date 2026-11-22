# acg_class_skald.lst — Representative Paizo-LST shape (skald, Advanced Class Guide)
# Stub for src/rules_core/rules_tables/acg/class_skald.rs to consume at RED-phase cycle.
# Operator replaces body with licensed Paizo ACG Skald LST at cycle-launch.
# Schema: tab-separated, # comment leader, three sections per class.

[header]
name	class_base_features_key	bab	save	starting_gold	hd	skill_list_key	class_skills	level_table_key
Skald	acg_class_skald_v1	high	fort_ref_will	4d6*10	1d10	skald_skill_list	<Acg skills; see published ACG Skald skill list>	<feat_table_key>

[/header]

[level_features]
1 Base feature (see ACG Skald §Class Features for level-1 features)
...standard progression with chosen-at-level-1 selections + every-N-level additions
[/level_features]

# Notes for the parser (mirror of corpus-source-inventory.md §2.1 row, Skald):
# - Cross-book invariants from corpus-source-inventory.md §2.3:
#   RuleSetId::Acg::resolve("acg:class:skald") returns Some.
#   RuleSetId::Apg::resolve("acg:class:skald") returns None.
# - Alchemist (ACG-side) shares identifier name with APG's Alchemist; the ACG variant lives at
#   src/rules_core/rules_tables/acg/class_alchemist_acg.rs (per corpus-source-inventory.md §2.1
#   footnote — distinct RuleSetId binding, not duplicate identifier).
# - Test fixture: tests/sd22_acg_class_skald_resolves.rs (one cycle per ACG class).

# === operator-replace point ===
