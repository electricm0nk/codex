<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-35 AT-35-E4-002 -- the bucket-V BatchExporter template.

     For one carrier character it emits, per ability category the carrier's units
     live in, every visible ability PCGen actually granted, as

         ABILITY|<KEY>|<substituted DESCRIPTION on one line>

     `.KEY` is the ability's own KEY token (pcgen/io/exporttoken/AbilityToken.java
     at the pinned SHA handles `.KEY`), so the join to our corpus_key is exact and
     never goes through a display name. `.DESC` is the description PCGen printed
     with its formulas already substituted -- the numbers the ORACLE'S ENGINE
     computed for this character, which is what the comparison needs.

     `bucket_v_parity.py carriers` fills the category list below from the units the
     carrier carries; the marker line is replaced, never edited by hand. -->
NAME=${pcstring('NAME')}
RACE=${pcstring('RACE')}
TOTALLEVELS=${pcstring('TOTALLEVELS')}
<#assign categories = [] />
<#list categories as cat>
<#assign n = pcvar('countdistinct("ABILITIES","CATEGORY=${cat}","VISIBILITY=DEFAULT[or]VISIBILITY=OUTPUT_ONLY")')?number>
CATEGORY|${cat}|${n?c}
<#if (n > 0)>
<#list 0..(n-1) as i>
ABILITY|${pcstring('ABILITYALL.${cat}.VISIBLE.${i}.KEY')}|${pcstring('ABILITYALL.${cat}.VISIBLE.${i}.DESC')?replace("\n", " ")}
</#list>
</#if>
</#list>
