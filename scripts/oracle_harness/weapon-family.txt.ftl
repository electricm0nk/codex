<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 remediation wave 6 (sd33-r6-weapon) -- WEAPON.n batch dump, reusable
     for any real wielded-weapon shape in the WEAPONPROF=<x>/WEAPON-enhancement
     family (see AT-33-E5-last67-weapon_cycle_receipt.md). Loop syntax copied
     verbatim from the shipped csheet's own equipped-weapon loop
     (outputsheets/base.xml.ftl, "Equipped weapon loop",
     `<@loop from=0 to=pcvar('COUNT[EQTYPE.WEAPON]-1') ; weap , weap_has_next>`).

     Known gap (this cycle, sd33-r6-weapon): a NATURALATTACKS-token-granted
     weapon (self-granted by an Equipment record, or granted by a
     TEMPLATESAPPLIED template) does NOT reliably appear under
     COUNT[EQTYPE.WEAPON] via this direct-java BatchExporter path -- confirmed
     empirically (see AT-33-E5-last39-weapon_cycle_receipt.md). This template
     works cleanly for any weapon the character actually wields/EQUIPSETs
     (real weapons, or a standalone-equipment natural-typed item like
     "Unarmed Strike"), but not for a bare NATURALATTACKS grant. -->
WEAPON.COUNT=${pcvar('COUNT[EQTYPE.WEAPON]')?c}
<@loop from=0 to=pcvar('COUNT[EQTYPE.WEAPON]-1') ; weap , weap_has_next>
WEAPON.${weap}.OUTPUTNAME=${pcstring('WEAPON.${weap}.OUTPUTNAME')}
WEAPON.${weap}.MAGICHIT=${pcstring('WEAPON.${weap}.MAGICHIT')}
WEAPON.${weap}.MAGICDAMAGE=${pcstring('WEAPON.${weap}.MAGICDAMAGE')}
WEAPON.${weap}.MULT=${pcstring('WEAPON.${weap}.MULT')}
WEAPON.${weap}.NUMATTACKS=${pcstring('WEAPON.${weap}.NUMATTACKS')}
</@loop>
