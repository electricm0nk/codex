# apg_spell_list.lst — Representative Paizo-LST shape (APG shared spell list)
# Stub for src/rules_core/rules_tables/apg/spell_list.rs (criterion 9 cycle).
# Cross-book context: APG-only spells resolve via RuleSetId::Apg, return None for others.

[header]
spell-level<TAB>school<TAB>name<TAB>classes<TAB>casting-time<TAB>duration<TAB>range<TAB>components<TAB>description_key

[apg-spells-by-level]
0<TAB>Universal<TAB>Bleed<TAB>Alchemist,Bloodrager,Cavalier (Order of the Tome),Hunter,Inquisitor,Magus,Summoner,Witch (Cleric,Oracle,Druid,Temptest Cleric,Devilbane Crusader,Sea Witch,Life Oracle)	TAB>TAB>instantaneous	TAB>Touch (Self)	TAB>V,S,M	TAB>cause a wound to bleed
0<TAB>Divination<TAB>Detect Poison<TAB>Hunter,Alchemist,Cavalier,Summoner,Magus	Bloodrager,Witch,Oracle,Cleric,Temptest Cleric,Devilbane Crusader,Sea Witch,Inquisitor,White Mage	Bard,Cleric,Druid,Pagan Priest,Sorcerer,Wizard	TAB>1 standard action	TAB>instantaneous	TAB>Close	TAB>V,S	TAB>detect poison in target
1<TAB>Abjuration<TAB>Ablative Barrier<TAB>Magus,Summoner,Bloodrager (see class)	TAB>1 standard action	TAB>1 minute/level or until discharged (D)	TAB>Personal	TAB>V,S	M/DF	TAB>absorb damage from one attack
1<TAB>Necromancy<TAB>Bleed (greater)<TAB>Alchemist,Bloodrager,Cavalier (Order of the Tome),Hunter,Inquisitor,Magus,Summoner,Witch,Bloodrager (Reign of Lore) (Cleric,Oracle,Druid)	TAB>1 standard action	TAB>instantaneous	TAB>Close	TAB>V,S,M/DF; see text	TAB>cause bleeding as bleed
1<TAB>Transmutation<TAB>Abundant Ammunition<TAB>Hunter,Magus,Cavalier,Bloodrager,Summoner,Cleric,Oracle,Druid,Inquisitor	TAB>1 standard action	TAB>1 minute/level (D)	TAB>Touch	TAB>V,S,DF	M/DF	TAB>ammunition summons itself
2<TAB>Abjuration	TAB>Barrow's Edge<TAB>Cavalier,Fighter,Inquisitor,Sorcerer,Wizard,Bloodrager,Magus	Witch,Cleric,Oracle,Bayna's Healer	Bloodrager (Reign of Lore)	Sorcerer (Tiefling),Wizard (Abjuration)	Aldhem,Angel,Aspirant (Pious Templar)	TAB>1 swift action	TAB>1 minute/level (D)	TAB>Personal	TAB>V	Sorcerer (Fiend-Touched)	Aldhem,Angel,Busyon (TBA),Howler (TBA)	inscribed weapon gain +1 enhancement
2<TAB>Transmutation	Adhesive Spittle	Alchemist,Bloodrager,Investigator,Magus,Witch	Bard,Cleric,Druid,Inquisitor,Sorcerer,Wizard,Mercenary,Mountain Warchief	Sorcerer (Feystones Demon-Touched),Wizard (Conjuration)	DC 16 Reflex halves	TAB>1 standard	TAB>1 round/level	TAB>Close (15ft+5ft/2 levels beyond)	V,S	Aldhem,Angel,Aspirant (Pious Templar),Howler (TBA)	sticky spittle weapon, +1 save vs repel

# === operator-replace point ===
