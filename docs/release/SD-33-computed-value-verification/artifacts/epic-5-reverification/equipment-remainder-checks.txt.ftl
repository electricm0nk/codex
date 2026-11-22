<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 equipment-remainder lane (AT-33-E5-001/002 remainder) -- minimal
     BatchExporter template for the equipment `other_bonus_shape` SKILL
     sub-population. Reuses AT-33-E2-002's own proven `<#list pc.checks as
     check>` loop verbatim (`epic-2-oracle-harness/computed-values.txt.ftl`)
     -- the same CHECK.<i>.NAME / CHECK.<i>.TOTAL export token family every
     prior AT-33-E5-00x cycle's own receipts named as the cheapest lever
     for this shape ("the existing computed-values.txt.ftl template already
     emits CHECK.<i>.TOTAL/.NAME, so the remaining work is a skill-name-to-
     CHECK-index lookup, not a new export mechanism"). -->
<#list pc.checks as check>
<#assign checknum = check?index />
CHECK.${checknum}.NAME=${pcstring('CHECK.${checknum}.NAME')}
CHECK.${checknum}.TOTAL=${pcstring('CHECK.${checknum}.TOTAL')}
</#list>
