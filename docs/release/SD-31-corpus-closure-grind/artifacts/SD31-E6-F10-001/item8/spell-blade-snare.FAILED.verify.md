# item-8 on-screen verification — FAILED

- verdict: **FAILED** — record is rendered but expected value(s) missing from screen: 'Druid' 
- family: `spell` · record: `Blade Snare`
- expected on screen: `invisible magic field`
- expected on screen: `Druid`
- agent: `sd31equipclass` · date: 2026-08-17T01:30:19Z
- HEAD: `a9426b760`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Spell Catalog
Back
Every real corpus record the engine knows about — 1937 spells across the Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Advanced Race Guide, Ultimate Intrigue, Ultimate Magic, Occult Adventures, Ultimate Combat and Inner Sea Gods. Not what any one character has selected. 27 carry no school in the corpus and appear only under “All”. Level shown is each record’s lowest class level, not any one class’s level — e.g. Hideous Laughter is Bard 1 but Sorcerer/Wizard 2, and lists as Level 1.

All books (1937)
CRB (652)
APG (297)
ACG (144)
ARG (92)
UI (101)
UM (269)
OA (144)
UC (146)
ISG (92)
All (1937)
Abjuration (197)
Conjuration (323)
Divination (151)
Enchantment (212)
Evocation (227)
Illusion (134)
Necromancy (173)
Transmutation (487)
Universal (6)

1 matching spell.

Blade SnareISGAbjuration
Level 3
This spell creates an invisible magic field that does not stop weapons (whether manufactured or natural) from moving toward you, but impedes their motion when they are retracted. When you are hit with a melee attack, attempt a caster level check against your opponent's CMD. If your check succeeds, your opponent's attacking weapon or body part becomes caught in the field, as if magically affixed to your body. If your check fails, your opponent may retract its weapon. If your opponent's melee weapon becomes trapped in the field, the opponent may release the weapon and move away from you. If your opponent attacked with a part of its body (such as a fist, a horn, a tail, etc.) or it attacked with a weapon but refuses to release it, your opponent gains the grappled condition. Because you are not using any part of your body to maintain control over your opponent, you do not gain the grappled condition. Once an opponent's weapon is snared, you may attempt a new caster level check against that opponent's CMD on each of your subsequent turns to maintain the grapple. This is a standard action, during which you may make any of the usual grappling actions, but if you choose to pin the opponent, you gain the grappled condition as well. On your opponent's turn, it may try to retract its weapon or limb by attempting a combat maneuver check or Escape Artist check, the DC of which is equal to the spell's saving throw. While blade snare is active, you may make melee attacks and cast spells as normal, though you may not make ranged weapon attacks; the magic field thwarts such attacks. You may not snare more than one limb or weapon in the field at a time. If you already have an opponent's weapon or limb stuck in the field and you are hit by a second melee attack, you must choose which weapon or limb to snare. The other limb or weapon is unaffected by the spell.```
