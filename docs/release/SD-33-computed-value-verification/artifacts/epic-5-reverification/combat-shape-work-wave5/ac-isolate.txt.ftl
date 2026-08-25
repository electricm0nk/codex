<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 remediation wave 5 (AT-33-E5-003) -- absolute-method AC
     isolator. AC.ISOLATED reads the real PCGen bonus TOTAL to
     COMBAT|AC, negating the BASE/Ability/Size buckets, which leaves
     exactly the item's own Armor/Shield/enhancement/Deflection/
     NaturalArmor/Insight/untyped contribution -- no baseline character
     needed, and structurally immune to a MAXDEX cap or a co-located
     Dex-enhancement chain (both of those flow through the Ability
     bucket this token explicitly subtracts out).
     The per-type breakdown lines are cross-check-only (not consumed by
     the isolator's own comparison), proving the composite token's
     arithmetic against a real per-type sum before it is trusted. -->
AC.TOTAL=${pcstring('AC.Total')}
AC.ISOLATED=${pcstring('BONUS.COMBAT.AC.TOTAL.!BASE.!Ability.!Size')}
AC.BASE=${pcstring('BONUS.COMBAT.AC.BASE')}
AC.ABILITY=${pcstring('BONUS.COMBAT.AC.Ability')}
AC.SIZE=${pcstring('BONUS.COMBAT.AC.Size')}
AC.ARMOR=${pcstring('BONUS.COMBAT.AC.Armor')}
AC.ARMORENH=${pcstring('BONUS.COMBAT.AC.ArmorEnhancement')}
AC.SHIELD=${pcstring('BONUS.COMBAT.AC.Shield')}
AC.SHIELDENH=${pcstring('BONUS.COMBAT.AC.ShieldEnhancement')}
AC.NATURALARMOR=${pcstring('BONUS.COMBAT.AC.NaturalArmor')}
AC.NATURALARMORENH=${pcstring('BONUS.COMBAT.AC.NaturalArmorEnhancement')}
AC.DEFLECTION=${pcstring('BONUS.COMBAT.AC.Deflection')}
AC.INSIGHT=${pcstring('BONUS.COMBAT.AC.Insight')}
