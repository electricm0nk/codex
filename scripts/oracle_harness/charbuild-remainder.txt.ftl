<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 AT-33-E5-remainder-charbuild -- BatchExporter template for the
     "full character build" remainder: class_feature (fixture+literal) and
     race units whose engine-side "ours" value needs a real L20-per-source-
     class or race-only PCGen build, not the narrow spell/equipment library
     seam AT-33-E5-001/002 used.

     Proven live against a real level-20 Rogue this cycle (SA count 11,
     Sneak Attack/Trap Sense/Trapfinding/Master Strike all rendered with
     real, substituted PF1 magnitudes in DESCRIPTION) -- COUNT[SA] /
     SPECIALABILITY.${sa} (used by base.xml.ftl) evaluate to 0 for
     PF1/Pathfinder_RPG content; ABILITYALL.<CATEGORY> is the real,
     modern-pipeline mechanism this gamemode's own shipped csheet
     (d20/fantasy/htmlxml/csheet_fantasy_std.htm.ftl) uses instead.

     Emits, per character:
       - all six ability scores (name/score) -- for `race` ability-
         adjustment units (base scores are fixed at generation time so the
         delta from base is the racial bonus/penalty).
       - every "Special Ability"-category ability, name + fully formatted,
         magnitude-substituted DESCRIPTION -- the generic mechanism a real
         player reads on the sheet, and the one that covers every
         class_feature magnitude family (DR, dice, uses/day, flat bonus,
         temp HP, save DC, ...) without a per-family export token. -->
<#list 0..5 as i>
STAT.${i}.NAME=${pcstring('STAT.${i}.NAME')}
STAT.${i}.SCORE=${pcstring('STAT.${i}')}
</#list>
<#assign sacount = pcvar('countdistinct("ABILITIES","CATEGORY=Special Ability","VISIBILITY=DEFAULT[or]VISIBILITY=OUTPUT_ONLY")')?number>
SA.COUNT=${sacount?c}
<#list 0..(sacount-1) as sa>
SA.${sa}.NAME=${pcstring('ABILITYALL.Special Ability.VISIBLE.${sa}')}
SA.${sa}.DESC=${pcstring('ABILITYALL.Special Ability.VISIBLE.${sa}.DESC')}
</#list>
