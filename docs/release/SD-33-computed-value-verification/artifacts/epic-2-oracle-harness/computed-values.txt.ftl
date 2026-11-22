<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 AT-33-E2-002 — minimal BatchExporter template.
     Emits PCGen's own COMPUTED variables (not literal LST text) as plain
     key=value lines, one token per line, machine-readable by design.
     Token names are the pcgen.io.exporttoken vocabulary the stock
     outputsheets/base.xml.ftl also uses (verified against that file at
     the pinned SHA before authoring this one). -->
NAME=${pcstring('NAME')}
RACE=${pcstring('RACE')}
CLASS.0.NAME=${pcstring('CLASS.0')}
CLASS.0.LEVEL=${pcstring('CLASS.0.LEVEL')}
STAT.STR.SCORE=${pcstring('STAT.0')}
STAT.STR.MOD=${pcstring('STAT.0.MOD')}
STAT.DEX.SCORE=${pcstring('STAT.1')}
STAT.DEX.MOD=${pcstring('STAT.1.MOD')}
STAT.CON.SCORE=${pcstring('STAT.2')}
STAT.CON.MOD=${pcstring('STAT.2.MOD')}
HP=${pcstring('HP')}
AC.TOTAL=${pcstring('AC.Total')}
AC.TOUCH=${pcstring('AC.Touch')}
AC.FLATFOOTED=${pcstring('AC.Flatfooted')}
BAB=${pcstring('ATTACK.MELEE.BASE')}
VAR.CMB=${pcstring('VAR.CMB.INTVAL')}
VAR.CMD=${pcstring('VAR.CMD.INTVAL')}
<#list pc.checks as check>
<#assign checknum = check?index />
CHECK.${checknum}.NAME=${pcstring('CHECK.${checknum}.NAME')}
CHECK.${checknum}.TOTAL=${pcstring('CHECK.${checknum}.TOTAL')}
CHECK.${checknum}.BASE=${pcstring('CHECK.${checknum}.BASE')}
</#list>
