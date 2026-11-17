<#-- SD-42 E1.7: the import parity export's own lines, appended by run.sh to the game mode's totals
     template (sheet-totals.txt.ftl, Pathfinder; sf_parity.txt.ftl, Starfinder). Every class and
     level, the money PCGen holds, and every gear line merged by location (EQ.MERGELOC, the stock
     sheet's equipment table): its output name, its long name (the
     .pcg's EQUIPNAME, the name Codex keys a record by), pieces, location. Totals only; no PCGen prose. -->
pcg.classes=${pcvar('COUNT[CLASSES]')?c}
<@loop from=0 to=pcvar('COUNT[CLASSES]-1') ; cls , cls_has_next>
pcg.class.${pcstring('CLASS.${cls}')}=${pcstring('CLASS.${cls}.LEVEL')}
</@loop>
pcg.gold=${pcstring('GOLD')}
pcg.gear=${pcvar('COUNT[EQUIPMENT.MERGELOC]')?c}
<@loop from=0 to=pcvar('COUNT[EQUIPMENT.MERGELOC]')-1 ; eq , eq_has_next>
pcg.gear.${eq}=${pcstring('EQ.MERGELOC.${eq}.NAME')}|long=${pcstring('EQ.MERGELOC.${eq}.LONGNAME')}|qty=${pcstring('EQ.MERGELOC.${eq}.QTY')}|location=${pcstring('EQ.MERGELOC.${eq}.LOCATION')}
</@loop>
