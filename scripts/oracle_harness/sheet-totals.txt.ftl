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
