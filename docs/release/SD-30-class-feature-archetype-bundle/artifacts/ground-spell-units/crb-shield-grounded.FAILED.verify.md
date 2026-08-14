# item-8 on-screen verification — FAILED

- verdict: **FAILED** — search for 'Shield' still shows 13 rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport
- family: `spell` · record: `Shield`
- expected on screen: `Shield`
- agent: `probe-spell-ground` · date: 2026-08-13T21:27:39Z
- HEAD: `d1593801`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Spell Catalog
Back
Every real corpus record the engine knows about — 1286 spells across the Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Advanced Race Guide and Ultimate Intrigue. Not what any one character has selected. 3 carry no school in the corpus and appear only under “All”. Level shown is each record’s lowest class level, not any one class’s level — e.g. Hideous Laughter is Bard 1 but Sorcerer/Wizard 2, and lists as Level 1.

All books (1286)
CRB (652)
APG (297)
ACG (144)
ARG (92)
UI (101)
All (1286)
Abjuration (133)
Conjuration (231)
Divination (101)
Enchantment (128)
Evocation (161)
Illusion (94)
Necromancy (109)
Transmutation (321)
Universal (5)

13 matching spells.

Entropic ShieldCRBAbjuration
Level 1
A magical field appears around you, glowing with a chaotic blast of multicolored hues. This field deflects incoming arrows, rays, and other ranged attacks. Each ranged attack directed at you for which the attacker must make an attack roll has a 20% miss chance [similar to the effects of concealment]. Other attacks that simply work at a distance are not affected.
ShieldCRBAbjuration
Level 1
Shield creates an invisible shield of force that hovers in front of you. It negates magic missile attacks directed at you. The disk also provides a +4 shield bonus to AC. This bonus applies against incorporeal touch attacks, since it is a force effect. The shield has no armor check penalty or arcane spell failure chance.
Shield OtherCRBAbjuration
Level 2
This spell wards the subject and creates a mystic connection between you and the subject so that some of its wounds are transferred to you. The subject gains a +1 deflection bonus to AC and a +1 resistance bonus on saves. Additionally, the subject takes only half damage from all wounds and attacks [including those dealt by special abilities] that deal hit point damage. The amount of damage not taken by the warded creature is taken by you. Forms of harm that do not involve hit points, such as charm effects, temporary ability damage, level draining, and death effects, are not affected. If the subject suffers a reduction of hit points from a lowered Constitution score, the reduction is not split with you because it is not hit point damage. When the spell ends, subsequent damage is no longer divided between the subject and you, but damage already split is not reassigned to the subject. If you and the subject of the spell move out of range of each other, the spell ends.
Shield of FaithCRBAbjuration
Level 1
This spell creates a shimmering, magical field around the target that averts and deflects attacks. The spell grants the subject a +2 deflection bonus to AC, with an additional +1 to the bonus for every six levels you have [maximum +5 deflection bonus at 18th level].
Shield of LawCRBAbjuration
Level 8
A dim, blue glow surrounds the subjects, protecting them from attacks, granting them resistance to spells cast by chaotic creatures, and slowing chaotic creatures when they strike the subjects. This abjuration has four effects. First, each warded creature gains a +4 deflection bonus to AC and a +4 resistance bonus on saves. Unlike protection from chaos, this benefit applies against all attacks, not just against attacks by chaotic creatures. Second, a warded creature gains spell resistance 25 against chaotic spells and spells cast by chaotic creatures. Third, the abjuration protects you from possession and mental influence, just as protection from chaos does. Finally, if a chaotic creature succeeds on a melee attack against a warded creature, the attacker is slowed [Will save negates, as the slow spell, but against shield of law's save DC].
Fire ShieldCRBEvocation
```
