<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 AT-33-E5-001 remediation -- generic SPELLMEM batch export.
     spellbook=0 ("Known Spells", the default bare-SPELLNAME book) is
     constant, but the PCGen `class` index a character's ONE class lands on
     is NOT always 0 -- empirically found this cycle: Wizard/Cleric/Druid/
     Bard/Ranger's own single class landed on index 0, but Paladin's
     landed on index 1 (both `.pcg`s carry exactly one `CLASS:` line; PCGen
     evidently orders `SPELLLISTCLASS` by something other than declaration
     order this cycle did not chase down further). Looping `class` 0..2
     covers every case observed and costs nothing extra for the classes
     that land on 0 -- the higher indices simply report `count=0`. -->
<#assign spellbook = 0 />
<#list 0..2 as class>
<#list 0..9 as level>
<#assign n = pcvar('COUNT[SPELLSINBOOK.${class}.${spellbook}.${level}]') />
<#if (n > 0)>
<#list 0..(n-1) as spell>
SPELL.${class}.${level}.${spell}.NAME=${pcstring('SPELLMEM.${class}.${spellbook}.${level}.${spell}.NAME')}
SPELL.${class}.${level}.${spell}.DC=${pcstring('SPELLMEM.${class}.${spellbook}.${level}.${spell}.DC')}
</#list>
</#if>
</#list>
</#list>
