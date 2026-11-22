# beastiary1_tarrasque_edge_case.md — Extreme-CR coverage for the rule-cycle table
# Stub for src/rules_core/rules_tables/beastiary1/monster_tarrasque.rs (rule-cycle-table edge case).
# Per corpus-source-inventory.md §3.2 row: Tarrasque key resolves through Bestiary 1 resolver and
# Encounter::new(...) returns a valid Difficulty without overflow errors.

Tarrasque	30	1,664,000	55	+0	540	+40	+23	+40	Colossal	40ft	Any	NE	5 Bite;10 Gore;20 Tail slap;5 Trample;1 Slap	Siege Monster,Swallow Whole,Frightful Presence (DC 50 Will),Reflect Arrows,Reflect Bullets,Reflect Cone,Reflect Ray,Reflect Spell (DC 22),Reflect Spell (DC 25),Reflect Spell (DC 28),Reflect Spell (DC 32)	Bane of Enemies,Blood Feast,Damage Reduction 30/epic;DR 30/magic;Fast Healing 40;Frightful Presence (DC 50);Haste;Immune to Ability Score Drain,Acid,Cold,Death Effects,Energy Drain,Fire,Negative Energy,Petrification,Poison,Sonic,Teleportation,Transmutation,Water;Magic Resistance 50;Regeneration 40;Spell Resistance 80;Trample (DC 58);Vulnerable to Critical Hits	None

# Notes:
# - The Tarrasque stub exists specifically for Epic 6's rule-cycle-table coverage: encounter math
#   must not overflow / panic on extreme CR. The Rust parser on this row sets hp_max = 540_000
#   (not u32 saturating) and the encounter-math test fixture in tests/sd22_dm_toolkit_deterministic.rs
#   asserts that encounters including the Tarrasque return a valid Difficulty (Easy/Medium/Hard/Deadly)
#   without overflowing.
# - The cap on Encounter difficulty (Deadly) is per PF1 "Encounter Building": nothing beyond Deadly
#   is defined; the algorithm MUST classify "Tarrasque + 4 level-3 PCs" as at-least Deadly.

# === operator-replace point ===
