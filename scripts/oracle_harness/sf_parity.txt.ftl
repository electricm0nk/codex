name=${pcstring('NAME')}
race=${pcstring('RACE')}
class=${pcstring('CLASS.0')}
level=${pcvar('TL')}
<#list 0..5 as stat>
stat.${pcstring('STAT.${stat}.NAME')}=${pcstring('STAT.${stat}')}
</#list>
hp=${pcstring('HP')}
stamina=${pcstring('ALTHP')}
resolve=${pcvar('VAR.Resolve')}
eac=${pcstring('AC.EAC')}
kac=${pcstring('AC.KAC')}
initiative=${pcstring('INITIATIVEMOD')}
bab=${pcstring('ATTACK.MELEE.BASE')}
<#list 0..2 as c>
save.${pcstring('CHECK.${c}.NAME')}=${pcstring('CHECK.${c}.TOTAL')}
</#list>
attack.melee=${pcstring('ATTACK.MELEE.TOTAL')}
attack.ranged=${pcstring('ATTACK.RANGED.TOTAL')}
<@loop from=0 to=pcvar('count("SKILLSIT", "VIEW=VISIBLE_EXPORT")')-1 ; skill , skill_has_next>
skill.${pcstring('SKILLSIT.${skill}')}=${pcstring('SKILLSIT.${skill}.TOTAL')}|ranks=${pcstring('SKILLSIT.${skill}.RANK')}|untrained=${pcstring('SKILLSIT.${skill}.UNTRAINED')}
</@loop>
<@loop from=0 to=pcvar('COUNT[EQTYPE.WEAPON]-1') ; weap , weap_has_next>
weapon.${pcstring('WEAPON.${weap}.NAME')}=${pcstring('WEAPON.${weap}.TOTALHIT')}|damage=${pcstring('WEAPON.${weap}.DAMAGE')}|category=${pcstring('WEAPON.${weap}.CATEGORY')}
</@loop>
<@loop from=pcvar('COUNT[SPELLRACE]') to=pcvar('COUNT[SPELLRACE]+COUNT[CLASSES]-1') ; class , class_has_next>
<#if (pcstring("SPELLLISTCLASS.${class}") != '') >
<#list 0..6 as lvl>
spells.${pcstring('SPELLLISTCLASS.${class}')}.${lvl}=per_day:${pcstring('SPELLLISTCAST.${class}.${lvl}')}|known:${pcstring('SPELLLISTKNOWN.${class}.${lvl}')}<#if (pcvar("COUNT[SPELLSINBOOK.${class}.0.${lvl}]") > 0)>|dcs:<@loop from=0 to=pcvar('COUNT[SPELLSINBOOK.${class}.0.${lvl}]-1') ; spell , spell_has_next>${pcstring('SPELLMEM.${class}.0.${lvl}.${spell}.DC')}<#if spell_has_next>,</#if></@loop>|in_book:${pcvar("COUNT[SPELLSINBOOK.${class}.0.${lvl}]")}</#if>
</#list>
</#if>
</@loop>
bulk=${pcstring('TOTAL.WEIGHT')}
credits=${pcstring('GOLD')}
credits.spent=${pcstring('TOTAL.VALUE')}
acp=${pcstring('ACCHECK')}
abilities.feat=${pcstring('ABILITYALLLIST.FEAT')}
abilities.theme=${pcstring('ABILITYALLLIST.Theme')}
abilities.class_feature=${pcstring('ABILITYALLLIST.Class Feature')}
abilities.racial_trait=${pcstring('ABILITYALLLIST.Racial Trait')}
