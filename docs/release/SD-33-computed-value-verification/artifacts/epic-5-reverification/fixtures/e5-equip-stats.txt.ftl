<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 AT-33-E5-001 -- minimal BatchExporter template, modeled on
     AT-33-E2-002's computed-values.txt.ftl. Emits all six ability scores
     (name + total score + modifier) so the harness can identify which
     ability an equipped item's enhancement bonus landed on, without
     guessing the STAT-index-to-ability mapping. -->
<#list 0..5 as i>
STAT.${i}.NAME=${pcstring('STAT.${i}.NAME')}
STAT.${i}.SCORE=${pcstring('STAT.${i}')}
STAT.${i}.MOD=${pcstring('STAT.${i}.MOD')}
</#list>
