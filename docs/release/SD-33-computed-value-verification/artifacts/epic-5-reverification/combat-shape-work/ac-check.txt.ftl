<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 Epic 5 combat/weapon lane -- minimal BatchExporter template,
     shared across every AC-shape unit (and the per-book baselines): the
     export token name (AC.Total) is fixed, not parameterized per unit,
     so no per-unit .ftl is needed (unlike the SKILL-shape lane's
     SKILL.<name>.MISC). -->
AC.TOTAL=${pcstring('AC.Total')}
