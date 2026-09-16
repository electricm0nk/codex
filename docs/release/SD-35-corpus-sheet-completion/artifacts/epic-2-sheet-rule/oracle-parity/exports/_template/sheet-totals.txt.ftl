<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-35 AT-35-E2-005 (blockers.md B1) -- the sheet-totals BatchExporter template.
     Emits PCGen's own COMPUTED totals for one character as KEY=VALUE lines, one
     token per line, in the same machine-readable shape as SD-33's
     computed-values.txt.ftl, widened with the totals that template lacked and
     that the sheet-rule evaluator's Number lines feed: skills (total and the
     misc component the rules contribute to), initiative, speed, vision, DR,
     spell DCs / slots / known per class and level, per-weapon attack and
     damage, and every visible special ability and feat with its substituted
     DESCRIPTION (the words PCGen prints, numbers already filled in).
     Token names are the pcgen.io.exporttoken / plugin.exporttokens vocabulary
     the stock outputsheets/base.xml.ftl uses at the pinned oracle SHA. -->
NAME=${pcstring('NAME')}
RACE=${pcstring('RACE')}
TOTALLEVELS=${pcstring('TOTALLEVELS')}
CLASS.COUNT=${pcvar('COUNT[CLASSES]')?c}
<@loop from=0 to=pcvar('COUNT[CLASSES]-1') ; cls , cls_has_next>
CLASS.${cls}.NAME=${pcstring('CLASS.${cls}')}
CLASS.${cls}.LEVEL=${pcstring('CLASS.${cls}.LEVEL')}
</@loop>
<#list 0..5 as i>
STAT.${i}.NAME=${pcstring('STAT.${i}.NAME')}
STAT.${i}.SCORE=${pcstring('STAT.${i}')}
STAT.${i}.MOD=${pcstring('STAT.${i}.MOD')}
</#list>
HP=${pcstring('HP')}
AC.TOTAL=${pcstring('AC.Total')}
AC.TOUCH=${pcstring('AC.Touch')}
AC.FLATFOOTED=${pcstring('AC.Flatfooted')}
AC.BASE=${pcstring('AC.Base')}
AC.ARMOR=${pcstring('AC.Armor')}
AC.SHIELD=${pcstring('AC.Shield')}
AC.ABILITY=${pcstring('AC.Ability')}
AC.SIZE=${pcstring('AC.Size')}
AC.NATURALARMOR=${pcstring('AC.NaturalArmor')}
AC.DEFLECTION=${pcstring('AC.Deflection')}
AC.DODGE=${pcstring('AC.Dodge')}
AC.MISC=${pcstring('AC.Misc')}
ACCHECK=${pcstring('ACCHECK')}
BAB=${pcstring('ATTACK.MELEE.BASE')}
ATTACK.MELEE.TOTAL=${pcstring('ATTACK.MELEE.TOTAL')}
ATTACK.MELEE.MISC=${pcstring('ATTACK.MELEE.MISC')}
ATTACK.RANGED.TOTAL=${pcstring('ATTACK.RANGED.TOTAL')}
ATTACK.RANGED.MISC=${pcstring('ATTACK.RANGED.MISC')}
VAR.CMB=${pcstring('VAR.CMB.INTVAL')}
VAR.CMD=${pcstring('VAR.CMD.INTVAL')}
INITIATIVEMOD=${pcstring('INITIATIVEMOD')}
INITIATIVEMISC=${pcstring('INITIATIVEMISC')}
INITIATIVEBONUS=${pcstring('INITIATIVEBONUS')}
<#list pc.checks as check>
<#assign checknum = check?index />
CHECK.${checknum}.NAME=${pcstring('CHECK.${checknum}.NAME')}
CHECK.${checknum}.TOTAL=${pcstring('CHECK.${checknum}.TOTAL')}
CHECK.${checknum}.BASE=${pcstring('CHECK.${checknum}.BASE')}
CHECK.${checknum}.MISC=${pcstring('CHECK.${checknum}.MISC')}
</#list>
SKILL.COUNT=${pcvar('COUNT[SKILLS]')?c}
<@loop from=0 to=pcvar('COUNT[SKILLS]-1') ; skill , skill_has_next>
SKILL.${skill}.NAME=${pcstring('SKILL.${skill}')}
SKILL.${skill}.TOTAL=${pcstring('SKILL.${skill}.TOTAL')}
SKILL.${skill}.RANK=${pcstring('SKILL.${skill}.RANK')}
SKILL.${skill}.ABMOD=${pcstring('SKILL.${skill}.ABMOD')}
SKILL.${skill}.MISC=${pcstring('SKILL.${skill}.MISC')}
SKILL.${skill}.ACHECK=${pcstring('SKILL.${skill}.ACPNONE,YES,NONPROF,DOUBLE,WEIGHT')}
</@loop>
MOVE.COUNT=${pcvar('COUNT[MOVE]')?c}
<@loop from=0 to=pcvar('COUNT[MOVE]-1') ; move , move_has_next>
MOVE.${move}.NAME=${pcstring('MOVE.${move}.NAME')}
MOVE.${move}.RATE=${pcstring('MOVE.${move}.RATE')}
</@loop>
VISION=${pcstring('VISION')}
DR=${pcstring('DR')}
SR=${pcstring('SR')}
SPELLCLASS.COUNT=${pcvar('COUNT[SPELLCLASSES]')?c}
<@loop from=0 to=pcvar('COUNT[SPELLCLASSES]-1') ; sc , sc_has_next>
SPELLLISTCLASS.${sc}=${pcstring('SPELLLISTCLASS.${sc}')}
SPELLLISTCLASS.${sc}.LEVEL=${pcstring('SPELLLISTCLASS.${sc}.LEVEL')}
SPELLLISTCLASS.${sc}.CASTERLEVEL=${pcstring('SPELLLISTCLASS.${sc}.CASTERLEVEL')}
SPELLLISTCLASS.${sc}.CONCENTRATION=${pcstring('SPELLLISTCLASS.${sc}.CONCENTRATION')}
<#list 0..9 as lvl>
SPELLLISTCAST.${sc}.${lvl}=${pcstring('SPELLLISTCAST.${sc}.${lvl}')}
SPELLLISTKNOWN.${sc}.${lvl}=${pcstring('SPELLLISTKNOWN.${sc}.${lvl}')}
SPELLLISTDC.${sc}.${lvl}=${pcstring('SPELLLISTDC.${sc}.${lvl}')}
</#list>
</@loop>
WEAPON.COUNT=${pcvar('COUNT[EQTYPE.WEAPON]')?c}
<@loop from=0 to=pcvar('COUNT[EQTYPE.WEAPON]-1') ; weap , weap_has_next>
WEAPON.${weap}.NAME=${pcstring('WEAPON.${weap}.NAME')}
WEAPON.${weap}.TOTALHIT=${pcstring('WEAPON.${weap}.TOTALHIT')}
WEAPON.${weap}.DAMAGE=${pcstring('WEAPON.${weap}.DAMAGE')}
WEAPON.${weap}.CRIT=${pcstring('WEAPON.${weap}.CRIT')}
WEAPON.${weap}.MULT=${pcstring('WEAPON.${weap}.MULT')}
</@loop>
<#-- Ability-category pools (cycle 3): PCGen has no export token for a category's pool, so
     the JEP `charbonusto("ABILITYPOOL", <category>)` total is emitted for every category the
     pinned Core Rulebook chain declares. `sheet_parity.py export` fills the list below from
     the data (`ABILITYCATEGORY:` rows) into a generated copy of this template; the marker
     line must stay exactly as written. -->
<#assign pool_categories = ["Special Ability", "Class Skill", "Equipment", "Choice", "Ability Bonus", "Gnome Obsessive Skill Bonus", "Adaptability Bonus Feat", "Favored Class", "Favored Class Bonus", "Favored Class Bonus ~ Skill Rank", "Barbarian Class Feature", "Bard Class Feature", "Cleric Class Feature", "Druid Class Feature", "Fighter Class Feature", "Monk Class Feature", "Paladin Class Feature", "Ranger Class Feature", "Rogue Class Feature", "Sorcerer Class Feature", "Wizard Class Feature", "Assassin Class Feature", "Dragon Disciple Class Feature", "Duelist Class Feature", "Eldritch Knight Class Feature", "Loremaster Class Feature", "Mystic Theurge Class Feature", "Pathfinder Chronicler Class Feature", "Shadowdancer Class Feature", "Companion Class Feature", "Traits", "Archetype", "ACF", "Alternative Class Features", "Barbarian Archetype", "Bard Archetype", "Cleric Archetype", "Druid Archetype", "Fighter Archetype", "Monk Archetype", "Paladin Archetype", "Ranger Archetype", "Rogue Archetype", "Sorcerer Archetype", "Wizard Archetype", "Racial Trait", "Giant Spider Type", "Fighter Bonus Feat", "Monk Bonus Feat", "Combat Trick Feat", "Weapon Training", "Ranger Bonus Feat", "Wizard Bonus Feat", "Armor Training Choice", "Sorcerer Bloodline", "Sorcerer Bloodline Power", "Sorcerer Bloodline Feat", "Crossblooded Sorcerer Bloodline", "Crossblooded Bloodline", "Eldritch Heritage Bloodline", "Rage Power", "Versatile Performance", "Channel Energy", "Weapon Training I", "Weapon Training II", "Weapon Training III", "Weapon Training IV", "Mercy", "Divine Bond", "Favored Enemy", "Favored Enemy Bonus", "Favored Terrain", "Favored Terrain Bonus", "Ranger Combat Style Feat", "Combat Style", "Hunter's Bond", "Rogue Talent", "Advanced Talent", "Bloodline", "Arcane Bond", "New Arcana", "Arcane Opposition School", "Expert Class Skills", "Nature Bond", "Druid Domain", "Weapon Mastery", "Simple Weapon Proficiency Choice", "Ki Pool Stat Choice", "Arcane School Specialization", "Necromancer Channeling", "Knowledge Class Skill", "Arcane Bloodline School Power", "Animal Companion Feat", "Companion Level Increase Choice", "Companion Stat Increase", "Companion Skill", "Animal Trick", "Loremaster Secret", "Add Spoken Language", "Cosmopolitan Spoken Language", "Intelligent Item", "Intelligent Item Alignment", "Intelligent Item Languages", "Intelligent Item Power", "Intelligent Item Purpose", "Intelligent Item Purpose Power", "Afflictions", "GM Awards", "Extra Channel", "Weapon Focus", "Greater Weapon Focus", "Improved Critical", "Greater Weapon Specialization", "Weapon Specialization", "Exotic Weapon Proficiency", "Martial Weapon Proficiency", "Skill Focus", "Spell Focus", "Teamwork Feat", "Class Feature", "Class Ability", "Special Mount Choice", "Permanency Applied Effects", "Optional Rules Systems", "Pact", "Background", "Human Ethnicity", "Region of Origin", "Headband of Intellect Skill Choice", "Scarlet and blue Sphere Ioun Stone Skill Choice", "Intelligence-Based Skill", "Raging Selection", "Sorcerer Draconic Bloodline", "Sorcerer Elemental Bloodline", "Draconic Bloodline Type", "Elemental Bloodline Type", "Blood of Dragons Bloodline", "Dragon Disciple Bloodline", "Temp Feat", "Temp Combat Feat", "Barbarian Class Selection", "Monk Class Selection", "Rogue Class Selection", "GM Bonus Spell Known", "GM Bonus Spell Cast", "GM Penalized Spell Known", "GM Penalized Spell Cast", "Human Subrace", "Racial Size", "Race Size Selection", "Class Level +4 Ability Adjustment", "Class Level +2 Ability Adjustment", "Class Level -2 Ability Adjustment", "Emotive Duality", "Ability Focus", "Spell-Like Ability", "Giant Creature Option", "Setting Specific Language", "Disable First Level Domain Power", "Class", "Size", "Save Bonus", "Condition", "Conditions", "Race", "Familiar Class Feature", "Familiar Alertness Choice", "Imp Consular Choice", "Aasimar Racial Trait", "Aasimar Race Trait", "Aasimar Subrace", "Android Racial Trait", "Aquatic Elf Racial Trait", "Catfolk Racial Trait", "Changeling Racial Trait", "Changeling Hag Racial Trait", "Dhampir Racial Trait", "Dhampir Subrace", "Drow Racial Trait", "Duergar Racial Trait", "Dwarf Racial Trait", "Elf Racial Trait", "Fetchling Racial Trait", "Gathlain Racial Trait", "Ghoran Racial Trait", "Gillman Racial Trait", "Gnome Racial Trait", "Goblin Racial Trait", "Grippli Racial Trait", "Half-Elf Racial Trait", "Half-Orc Racial Trait", "Halfling Racial Trait", "Hobgoblin Racial Trait", "Human Racial Trait", "Ifrit Racial Trait", "Kasatha Racial Trait", "Kitsune Racial Trait", "Kobold Racial Trait", "Kobold Scale Color", "Lashunta Racial Trait", "Lashunta Knowledgeable Selection", "Merfolk Racial Trait", "Monkey Goblin Racial Trait", "Nagaji Racial Trait", "Orc Racial Trait", "Oread Racial Trait", "Ratfolk Racial Trait", "Rougarou Racial Trait", "Samsaran Racial Trait", "Samsaran Shards of the Past Skills", "Skinwalker Racial Trait", "Skinwalker Heritage", "Strix Racial Trait", "Suli Racial Trait", "Suli Language Choice", "Svirfneblin Racial Trait", "Sylph Racial Trait", "Syrinx Racial Trait", "Tengu Racial Trait", "Tiefling Racial Trait", "Tiefling Language Choice", "Tiefling Subrace", "Triaxian Racial Trait", "Trox Racial Trait", "Undine Racial Trait", "Vanara Racial Trait", "Vishkanya Racial Trait", "Wayang Racial Trait", "Wyrwood Racial Trait", "Wyvaran Racial Trait"] />
POOL.COUNT=${pool_categories?size?c}
<#list pool_categories as cat>
POOL.${cat?index}.NAME=${cat}
POOL.${cat?index}.SIZE=${pcvar("charbonusto(\"ABILITYPOOL\",\"${cat}\")")?c}
</#list>
<#-- Spell-like abilities (cycle 3): a `SPELLS:` token's spells sit in the spellbook the token
     names (Racial, Class, ...) under a class index the stock sheet walks from 0 to
     COUNT[SPELLRACE]+COUNT[CLASSES]-1; every book, class index and level is walked so the
     export is generic. -->
SPELLBOOK.COUNT=${pcvar('COUNT[SPELLBOOKS]')?c}
<@loop from=0 to=pcvar('COUNT[SPELLBOOKS]-1') ; book , book_has_next>
SPELLBOOK.${book}.NAME=${pcstring('SPELLBOOKNAME.${book}')}
<@loop from=0 to=pcvar('COUNT[SPELLRACE]+COUNT[CLASSES]-1') ; sc , sc_has_next>
<#list 0..9 as lvl>
<#assign nsp = pcvar('COUNT[SPELLSINBOOK.${sc}.${book}.${lvl}]')?number>
<#if (nsp > 0)>
<#list 0..(nsp-1) as sp>
SPELLMEM.${sc}.${book}.${lvl}.${sp}.NAME=${pcstring('SPELLMEM.${sc}.${book}.${lvl}.${sp}.NAME')}
SPELLMEM.${sc}.${book}.${lvl}.${sp}.TIMES=${pcstring('SPELLMEM.${sc}.${book}.${lvl}.${sp}.TIMES')}
SPELLMEM.${sc}.${book}.${lvl}.${sp}.TIMEUNIT=${pcstring('SPELLMEM.${sc}.${book}.${lvl}.${sp}.TIMEUNIT')}
SPELLMEM.${sc}.${book}.${lvl}.${sp}.CASTERLEVEL=${pcstring('SPELLMEM.${sc}.${book}.${lvl}.${sp}.CASTERLEVEL')}
SPELLMEM.${sc}.${book}.${lvl}.${sp}.DC=${pcstring('SPELLMEM.${sc}.${book}.${lvl}.${sp}.DC')}
</#list>
</#if>
</#list>
</@loop>
</@loop>
<#assign sacount = pcvar('countdistinct("ABILITIES","CATEGORY=Special Ability","VISIBILITY=DEFAULT[or]VISIBILITY=OUTPUT_ONLY")')?number>
SA.COUNT=${sacount?c}
<#list 0..(sacount-1) as sa>
SA.${sa}.NAME=${pcstring('ABILITYALL.Special Ability.VISIBLE.${sa}')}
SA.${sa}.DESC=${pcstring('ABILITYALL.Special Ability.VISIBLE.${sa}.DESC')}
</#list>
<#assign featcount = pcvar('countdistinct("ABILITIES","CATEGORY=FEAT","VISIBILITY=DEFAULT[or]VISIBILITY=OUTPUT_ONLY")')?number>
FEAT.COUNT=${featcount?c}
<#list 0..(featcount-1) as ft>
FEAT.${ft}.NAME=${pcstring('ABILITYALL.FEAT.VISIBLE.${ft}')}
FEAT.${ft}.DESC=${pcstring('ABILITYALL.FEAT.VISIBLE.${ft}.DESC')}
</#list>
