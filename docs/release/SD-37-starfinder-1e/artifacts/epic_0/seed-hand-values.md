---
canonical: true
bundle_id: SD-37
card: E0.4
artifact_type: seed-hand-values
fetched: 2026-10-02
review_status: pending independent Opus review (E0.4 reviewer step)
---

# SD-37 Starfinder seed hand values (E0.4 deliverable 2)

One row per value, for the four SF seeds in `seed-builds.md`. Each row has the shape
`| SF-<seed> | <field> | <value> | <SRD URL + section> — <arithmetic> |`. Every input was
transcribed on 2026-10-02 from the Starfinder Reference Document as hosted by Archives of Nethys
(`https://www.aonsrd.com/`); none comes from the PCGen `.lst` files or from recall. Page sizes and
sha256 digests are in `E0.4-srd-fetch-log.txt`. The build choices (ability scores, ranks, gear)
these values depend on are in `seed-builds.md`.

**Fields.** BAB; Fort/Ref/Will; HP; Stamina; Resolve; EAC; KAC; all 20 Core Rulebook skills; and,
for the Mystic and Technomancer, spells per day (1st, 2nd) and spells known (0, 1st, 2nd).

**Rules applied** (each row cites the ones it uses):

- Saves = class base save + Con (Fort) / Dex (Ref) / Wis (Will) modifier
  (https://www.aonsrd.com/Rules.aspx?ID=106, Saving Throw Types).
- HP = racial HP at 1st + class HP × level; Stamina = (class SP + Con modifier, never below 0) ×
  level (https://www.aonsrd.com/Rules.aspx?ID=49).
- Resolve = ½ character level rounded down (minimum 1) + key ability modifier
  (https://www.aonsrd.com/Rules.aspx?ID=50).
- EAC/KAC = 10 + armour bonus + Dex modifier, Dex limited by the armour's max Dex
  (https://www.aonsrd.com/Rules.aspx?ID=102; max Dex: https://www.aonsrd.com/Rules.aspx?ID=87).
- Skill total = ranks + 3 (trained class skill, ≥ 1 rank) + ability modifier + other modifiers,
  armour check penalty on skills marked "Armor Check Penalty"; a trained-only skill with 0 ranks
  cannot be attempted, so it has no total (https://www.aonsrd.com/Rules.aspx?ID=78,
  https://www.aonsrd.com/Skills.aspx?ItemName=All).
- Spells per day = class table + bonus spells for the key ability score, only at spell levels the
  caster can already cast; spells known = class table (+ one connection spell per castable level
  for the Mystic) (https://www.aonsrd.com/Classes.aspx?ItemName=Mystic,
  https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer).

**Printed, not summed** (situational or roll-time, so no total includes them): android +2 racial
bonus to saves against disease, mind-affecting effects, poison and sleep; Mobility's +4 against
attacks of opportunity; the envoy's expertise die; the Empath's Empathy +2 circumstance bonus;
the Engineering −2 for working without an engineering kit.

| Seed | Field | Value | Source (SRD URL + section) — arithmetic |
|---|---|---|---|
| SF-Soldier-3 | BAB | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (class table, 3rd level) — Base Attack Bonus column |
| SF-Soldier-3 | Fort | +4 | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (class table, 3rd level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +3 + Con mod +1 |
| SF-Soldier-3 | Ref | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (class table, 3rd level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +1 + Dex mod +2 |
| SF-Soldier-3 | Will | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (class table, 3rd level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +3 + Wis mod +0 |
| SF-Soldier-3 | HP | 25 | https://www.aonsrd.com/Races.aspx?ItemName=Human (Hit Points 4); https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Hit Points: 7); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Hit Points) — race 4 + class 7 × 3 |
| SF-Soldier-3 | Stamina | 24 | https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Stamina Points: 7); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Stamina Points) — (7 + Con mod +1) × 3 |
| SF-Soldier-3 | Resolve | 4 | https://www.aonsrd.com/Rules.aspx?ID=50 (Calculating Resolve Points); https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Key Ability Score - Str) — max(1, ⌊3/2⌋) + Str mod +3 |
| SF-Soldier-3 | EAC | 16 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Squad&Family=Defiance%20Series (EAC bonus +5, Max. Dex +1) — 10 + 5 + Dex mod +2 capped at max Dex +1 → +1 |
| SF-Soldier-3 | KAC | 19 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Squad&Family=Defiance%20Series (KAC bonus +8, Max. Dex +1) — 10 + 8 + Dex mod +2 capped at max Dex +1 → +1 |
| SF-Soldier-3 | Skill: Acrobatics | −2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Acrobatics: Dex; Armor Check Penalty); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Squad&Family=Defiance%20Series (Armor Check Penalty −4) — Dex +2; ACP −4 = −2 |
| SF-Soldier-3 | Skill: Athletics | +6 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Athletics: Str; Armor Check Penalty); https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Class Skills); https://www.aonsrd.com/Themes.aspx?ItemName=Mercenary; https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Squad&Family=Defiance%20Series (Armor Check Penalty −4) — 3 ranks; +3 trained class skill; Str +3; +1 Mercenary theme knowledge: Athletics already a soldier class skill → +1; ACP −4 = +6 |
| SF-Soldier-3 | Skill: Bluff | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Bluff: Cha) — Cha +0 = +0 |
| SF-Soldier-3 | Skill: Computers | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Computers: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Culture | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Culture: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Diplomacy | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Diplomacy: Cha) — Cha +0 = +0 |
| SF-Soldier-3 | Skill: Disguise | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Disguise: Cha) — Cha +0 = +0 |
| SF-Soldier-3 | Skill: Engineering | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Engineering: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Intimidate | +6 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Intimidate: Cha); https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Class Skills) — 3 ranks; +3 trained class skill; Cha +0 = +6 |
| SF-Soldier-3 | Skill: Life Science | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Life Science: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Medicine | +6 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Medicine: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Class Skills) — 3 ranks; +3 trained class skill; Int +0 = +6 |
| SF-Soldier-3 | Skill: Mysticism | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Mysticism: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Perception | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Perception: Wis) — Wis +0 = +0 |
| SF-Soldier-3 | Skill: Physical Science | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Physical Science: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Piloting | +8 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Piloting: Dex); https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Class Skills) — 3 ranks; +3 trained class skill; Dex +2 = +8 |
| SF-Soldier-3 | Skill: Profession | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Profession: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Sense Motive | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Sense Motive: Wis) — Wis +0 = +0 |
| SF-Soldier-3 | Skill: Sleight of Hand | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Sleight of Hand: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Soldier-3 | Skill: Stealth | −2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Stealth: Dex; Armor Check Penalty); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Squad&Family=Defiance%20Series (Armor Check Penalty −4) — Dex +2; ACP −4 = −2 |
| SF-Soldier-3 | Skill: Survival | +6 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Survival: Wis); https://www.aonsrd.com/Classes.aspx?ItemName=Soldier (Class Skills) — 3 ranks; +3 trained class skill; Wis +0 = +6 |
| SF-Mystic-5 | BAB | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (class table, 5th level) — Base Attack Bonus column |
| SF-Mystic-5 | Fort | +0 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (class table, 5th level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +1 + Con mod −1 |
| SF-Mystic-5 | Ref | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (class table, 5th level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +1 + Dex mod +2 |
| SF-Mystic-5 | Will | +8 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (class table, 5th level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +4 + Wis mod +4 |
| SF-Mystic-5 | HP | 34 | https://www.aonsrd.com/Races.aspx?ItemName=Lashunta (Hit Points 4); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Hit Points: 6); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Hit Points) — race 4 + class 6 × 5 |
| SF-Mystic-5 | Stamina | 25 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Stamina Points: 6); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Stamina Points) — (6 + Con mod −1) × 5 |
| SF-Mystic-5 | Resolve | 6 | https://www.aonsrd.com/Rules.aspx?ID=50 (Calculating Resolve Points); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Key Ability Score - Wis) — max(1, ⌊5/2⌋) + Wis mod +4 |
| SF-Mystic-5 | EAC | 16 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Basic&Family=Lashunta%20Tempweave (EAC bonus +4, Max. Dex +5) — 10 + 4 + Dex mod +2 (max Dex +5 not reached) |
| SF-Mystic-5 | KAC | 16 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Basic&Family=Lashunta%20Tempweave (KAC bonus +4, Max. Dex +5) — 10 + 4 + Dex mod +2 (max Dex +5 not reached) |
| SF-Mystic-5 | Skill: Acrobatics | +2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Acrobatics: Dex; Armor Check Penalty) — Dex +2 = +2 |
| SF-Mystic-5 | Skill: Athletics | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Athletics: Str; Armor Check Penalty) — Str +0 = +0 |
| SF-Mystic-5 | Skill: Bluff | +10 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Bluff: Cha); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills) — 5 ranks; +3 trained class skill; Cha +2 = +10 |
| SF-Mystic-5 | Skill: Computers | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Computers: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Mystic-5 | Skill: Culture | +10 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Culture: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills) — 5 ranks; +3 trained class skill; Int +2 = +10 |
| SF-Mystic-5 | Skill: Diplomacy | +12 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Diplomacy: Cha); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills); https://www.aonsrd.com/Races.aspx?ItemName=Lashunta — 5 ranks; +3 trained class skill; Cha +2; +2 Lashunta student: +2 racial (chosen skill) = +12 |
| SF-Mystic-5 | Skill: Disguise | +2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Disguise: Cha) — Cha +2 = +2 |
| SF-Mystic-5 | Skill: Engineering | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Engineering: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Mystic-5 | Skill: Intimidate | +2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Intimidate: Cha) — Cha +2 = +2 |
| SF-Mystic-5 | Skill: Life Science | +10 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Life Science: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills) — 5 ranks; +3 trained class skill; Int +2 = +10 |
| SF-Mystic-5 | Skill: Medicine | +12 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Medicine: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills); https://www.aonsrd.com/Races.aspx?ItemName=Lashunta — 5 ranks; +3 trained class skill; Int +2; +2 Lashunta student: +2 racial (chosen skill) = +12 |
| SF-Mystic-5 | Skill: Mysticism | +13 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Mysticism: Wis; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills); https://www.aonsrd.com/Themes.aspx?ItemName=Priest — 5 ranks; +3 trained class skill; Wis +4; +1 Priest theme knowledge: Mysticism already a mystic class skill → +1 = +13 |
| SF-Mystic-5 | Skill: Perception | +14 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Perception: Wis); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic — 5 ranks; +3 trained class skill; Wis +4; +2 channel skill +2 insight at 5th (Empath associated skill) = +14 |
| SF-Mystic-5 | Skill: Physical Science | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Physical Science: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Mystic-5 | Skill: Piloting | +2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Piloting: Dex) — Dex +2 = +2 |
| SF-Mystic-5 | Skill: Profession | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Profession: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Mystic-5 | Skill: Sense Motive | +14 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Sense Motive: Wis); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Class Skills); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic — 5 ranks; +3 trained class skill; Wis +4; +2 channel skill +2 insight at 5th (Empath associated skill) = +14 |
| SF-Mystic-5 | Skill: Sleight of Hand | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Sleight of Hand: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Mystic-5 | Skill: Stealth | +2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Stealth: Dex; Armor Check Penalty) — Dex +2 = +2 |
| SF-Mystic-5 | Skill: Survival | +4 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Survival: Wis) — Wis +4 = +4 |
| SF-Mystic-5 | Spells per day: 1st | 5 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Spells Per Day column, 5th level; Bonus Spells Per Day table, Wis 19) — 4 + bonus 1 (Wis 18-19 row) |
| SF-Mystic-5 | Spells per day: 2nd | 3 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Spells Per Day column, 5th level; Bonus Spells Per Day table, Wis 19) — 2 + bonus 1 (Wis 18-19 row) |
| SF-Mystic-5 | Spells known: 0 | 6 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Spells Known table, 5th level) — 6 from table |
| SF-Mystic-5 | Spells known: 1st | 5 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Spells Known table, 5th level); https://www.aonsrd.com/MysticConnections.aspx?ItemName=Empath (Spells: 1st - detect thoughts); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Connection Spell) — 4 from table + 1 connection spell (detect thoughts) |
| SF-Mystic-5 | Spells known: 2nd | 4 | https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Spells Known table, 5th level); https://www.aonsrd.com/MysticConnections.aspx?ItemName=Empath (Spells: 2nd - zone of truth); https://www.aonsrd.com/Classes.aspx?ItemName=Mystic (Connection Spell) — 3 from table + 1 connection spell (zone of truth) |
| SF-Technomancer-5 | BAB | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (class table, 5th level) — Base Attack Bonus column |
| SF-Technomancer-5 | Fort | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (class table, 5th level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +1 + Con mod +2 |
| SF-Technomancer-5 | Ref | +4 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (class table, 5th level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +1 + Dex mod +3 |
| SF-Technomancer-5 | Will | +5 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (class table, 5th level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +4 + Wis mod +1 |
| SF-Technomancer-5 | HP | 29 | https://www.aonsrd.com/Races.aspx?ItemName=Android (Hit Points 4); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Hit Points: 5); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Hit Points) — race 4 + class 5 × 5 |
| SF-Technomancer-5 | Stamina | 35 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Stamina Points: 5); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Stamina Points) — (5 + Con mod +2) × 5 |
| SF-Technomancer-5 | Resolve | 6 | https://www.aonsrd.com/Rules.aspx?ID=50 (Calculating Resolve Points); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Key Ability Score - Int) — max(1, ⌊5/2⌋) + Int mod +4 |
| SF-Technomancer-5 | EAC | 18 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=I&Family=D-Suit (EAC bonus +5, Max. Dex +5) — 10 + 5 + Dex mod +3 (max Dex +5 not reached) |
| SF-Technomancer-5 | KAC | 19 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=I&Family=D-Suit (KAC bonus +6, Max. Dex +5) — 10 + 6 + Dex mod +3 (max Dex +5 not reached) |
| SF-Technomancer-5 | Skill: Acrobatics | +3 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Acrobatics: Dex; Armor Check Penalty) — Dex +3 = +3 |
| SF-Technomancer-5 | Skill: Athletics | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Athletics: Str; Armor Check Penalty) — Str +0 = +0 |
| SF-Technomancer-5 | Skill: Bluff | −1 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Bluff: Cha) — Cha −1 = −1 |
| SF-Technomancer-5 | Skill: Computers | +13 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Computers: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Class Skills); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer — 5 ranks; +3 trained class skill; Int +4; +1 techlore +1 insight (3rd level) = +13 |
| SF-Technomancer-5 | Skill: Culture | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Culture: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Technomancer-5 | Skill: Diplomacy | −1 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Diplomacy: Cha) — Cha −1 = −1 |
| SF-Technomancer-5 | Skill: Disguise | −1 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Disguise: Cha) — Cha −1 = −1 |
| SF-Technomancer-5 | Skill: Engineering | +12 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Engineering: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Class Skills) — 5 ranks; +3 trained class skill; Int +4 = +12 |
| SF-Technomancer-5 | Skill: Intimidate | −1 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Intimidate: Cha) — Cha −1 = −1 |
| SF-Technomancer-5 | Skill: Life Science | +12 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Life Science: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Class Skills) — 5 ranks; +3 trained class skill; Int +4 = +12 |
| SF-Technomancer-5 | Skill: Medicine | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Medicine: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Technomancer-5 | Skill: Mysticism | +10 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Mysticism: Wis; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Class Skills); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer — 5 ranks; +3 trained class skill; Wis +1; +1 techlore +1 insight (3rd level) = +10 |
| SF-Technomancer-5 | Skill: Perception | +6 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Perception: Wis) — 5 ranks; Wis +1 = +6 |
| SF-Technomancer-5 | Skill: Physical Science | +13 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Physical Science: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Class Skills); https://www.aonsrd.com/Themes.aspx?ItemName=Scholar — 5 ranks; +3 trained class skill; Int +4; +1 Scholar theme knowledge (Physical Science chosen; already a technomancer class skill) → +1 = +13 |
| SF-Technomancer-5 | Skill: Piloting | +11 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Piloting: Dex); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Class Skills) — 5 ranks; +3 trained class skill; Dex +3 = +11 |
| SF-Technomancer-5 | Skill: Profession | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Profession: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Technomancer-5 | Skill: Sense Motive | −1 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Sense Motive: Wis); https://www.aonsrd.com/Races.aspx?ItemName=Android — Wis +1; −2 Android flat affect: −2 Sense Motive = −1 |
| SF-Technomancer-5 | Skill: Sleight of Hand | +11 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Sleight of Hand: Dex; Armor Check Penalty; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Class Skills) — 5 ranks; +3 trained class skill; Dex +3 = +11 |
| SF-Technomancer-5 | Skill: Stealth | +3 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Stealth: Dex; Armor Check Penalty) — Dex +3 = +3 |
| SF-Technomancer-5 | Skill: Survival | +1 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Survival: Wis) — Wis +1 = +1 |
| SF-Technomancer-5 | Spells per day: 1st | 5 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Spells Per Day column, 5th level; Bonus Spells Per Day table, Int 19) — 4 + bonus 1 (Int 18-19 row) |
| SF-Technomancer-5 | Spells per day: 2nd | 3 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Spells Per Day column, 5th level; Bonus Spells Per Day table, Int 19) — 2 + bonus 1 (Int 18-19 row) |
| SF-Technomancer-5 | Spells known: 0 | 6 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Spells Known table, 5th level) — 6 from table |
| SF-Technomancer-5 | Spells known: 1st | 4 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Spells Known table, 5th level) — 4 from table |
| SF-Technomancer-5 | Spells known: 2nd | 3 | https://www.aonsrd.com/Classes.aspx?ItemName=Technomancer (Spells Known table, 5th level) — 3 from table |
| SF-Envoy-3 | BAB | +2 | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (class table, 3rd level) — Base Attack Bonus column |
| SF-Envoy-3 | Fort | +2 | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (class table, 3rd level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +1 + Con mod +1 |
| SF-Envoy-3 | Ref | +4 | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (class table, 3rd level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +3 + Dex mod +1 |
| SF-Envoy-3 | Will | +3 | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (class table, 3rd level); https://www.aonsrd.com/Rules.aspx?ID=106 (Saving Throw Types) — base +3 + Wis mod +0 |
| SF-Envoy-3 | HP | 20 | https://www.aonsrd.com/Races.aspx?ItemName=Ysoki (Hit Points 2); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Hit Points: 6); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Hit Points) — race 2 + class 6 × 3 |
| SF-Envoy-3 | Stamina | 21 | https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Stamina Points: 6); https://www.aonsrd.com/Rules.aspx?ID=49 (Calculating Stamina Points) — (6 + Con mod +1) × 3 |
| SF-Envoy-3 | Resolve | 5 | https://www.aonsrd.com/Rules.aspx?ID=50 (Calculating Resolve Points); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Key Ability Score - Cha) — max(1, ⌊3/2⌋) + Cha mod +4 |
| SF-Envoy-3 | EAC | 14 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Graphite&Family=Carbon%20Skin (EAC bonus +3, Max. Dex +4) — 10 + 3 + Dex mod +1 (max Dex +4 not reached) |
| SF-Envoy-3 | KAC | 15 | https://www.aonsrd.com/Rules.aspx?ID=102 (Armor Class formula); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Graphite&Family=Carbon%20Skin (KAC bonus +4, Max. Dex +4) — 10 + 4 + Dex mod +1 (max Dex +4 not reached) |
| SF-Envoy-3 | Skill: Acrobatics | +0 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Acrobatics: Dex; Armor Check Penalty); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Graphite&Family=Carbon%20Skin (Armor Check Penalty −1) — Dex +1; ACP −1 = +0 |
| SF-Envoy-3 | Skill: Athletics | −2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Athletics: Str; Armor Check Penalty); https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Graphite&Family=Carbon%20Skin (Armor Check Penalty −1) — Str −1; ACP −1 = −2 |
| SF-Envoy-3 | Skill: Bluff | +10 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Bluff: Cha); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills) — 3 ranks; +3 trained class skill; Cha +4 = +10 |
| SF-Envoy-3 | Skill: Computers | +7 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Computers: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills) — 3 ranks; +3 trained class skill; Int +1 = +7 |
| SF-Envoy-3 | Skill: Culture | +8 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Culture: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills); https://www.aonsrd.com/Themes.aspx?ItemName=Icon — 3 ranks; +3 trained class skill; Int +1; +1 Icon theme knowledge: Culture already an envoy class skill → +1 = +8 |
| SF-Envoy-3 | Skill: Diplomacy | +10 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Diplomacy: Cha); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills) — 3 ranks; +3 trained class skill; Cha +4 = +10 |
| SF-Envoy-3 | Skill: Disguise | +4 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Disguise: Cha) — Cha +4 = +4 |
| SF-Envoy-3 | Skill: Engineering | +9 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Engineering: Int; Trained Only); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills); https://www.aonsrd.com/Races.aspx?ItemName=Ysoki — 3 ranks; +3 trained class skill; Int +1; +2 Ysoki scrounger +2 racial = +9 |
| SF-Envoy-3 | Skill: Intimidate | +10 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Intimidate: Cha); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills) — 3 ranks; +3 trained class skill; Cha +4 = +10 |
| SF-Envoy-3 | Skill: Life Science | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Life Science: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Envoy-3 | Skill: Medicine | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Medicine: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Envoy-3 | Skill: Mysticism | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Mysticism: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Envoy-3 | Skill: Perception | +6 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Perception: Wis); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills) — 3 ranks; +3 trained class skill; Wis +0 = +6 |
| SF-Envoy-3 | Skill: Physical Science | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Physical Science: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Envoy-3 | Skill: Piloting | +1 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Piloting: Dex) — Dex +1 = +1 |
| SF-Envoy-3 | Skill: Profession | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Profession: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Envoy-3 | Skill: Sense Motive | +6 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Sense Motive: Wis); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills) — 3 ranks; +3 trained class skill; Wis +0 = +6 |
| SF-Envoy-3 | Skill: Sleight of Hand | untrained (trained only) | https://www.aonsrd.com/Skills.aspx?ItemName=All (Sleight of Hand: Trained Only); https://www.aonsrd.com/Rules.aspx?ID=78 (trained-only skills) — 0 ranks, no check possible, no total printed |
| SF-Envoy-3 | Skill: Stealth | +8 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Stealth: Dex; Armor Check Penalty); https://www.aonsrd.com/Classes.aspx?ItemName=Envoy (Class Skills); https://www.aonsrd.com/Races.aspx?ItemName=Ysoki; https://www.aonsrd.com/ArmorDisplay.aspx?ItemName=Graphite&Family=Carbon%20Skin (Armor Check Penalty −1) — 3 ranks; +3 trained class skill; Dex +1; +2 Ysoki scrounger +2 racial; ACP −1 = +8 |
| SF-Envoy-3 | Skill: Survival | +2 | https://www.aonsrd.com/Rules.aspx?ID=78 (Skill Check Type table); https://www.aonsrd.com/Skills.aspx?ItemName=All (Survival: Wis); https://www.aonsrd.com/Races.aspx?ItemName=Ysoki — Wis +0; +2 Ysoki scrounger +2 racial = +2 |

## Row count

```bash
f=docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-hand-values.md
awk -F'|' '/^\| *SF-/{n[$2]++} END{for(s in n) print s, n[s]}' "$f" | sort
```

Output on the committed file: SF-Envoy-3 29, SF-Mystic-5 34, SF-Soldier-3 29,
SF-Technomancer-5 34 (126 rows). Each seed has 9 core rows (BAB, Fort, Ref, Will, HP, Stamina,
Resolve, EAC, KAC) and 20 skill rows; the two casters add 2 spells-per-day rows and 3
spells-known rows.

## Independent review

Pending. The E0.4 reviewer (a separate Opus agent) re-fetches every URL above and adds a table
with one `agree` / `disagree` verdict per row. E0.4 is not complete until that table exists and
has 0 `disagree` rows.
