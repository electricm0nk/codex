# Level-10 Progression Validation Contract

## Objective
Define what later execution must prove before any SD-13 class can claim truthful support through level 10.

## Universal progression dimensions
Every class-level support claim through level 10 must classify these dimensions where they matter to the class:
1. class identity and level progression posture
2. base attack bonus progression
3. saving-throw progression
4. class-feature progression through the relevant level milestones
5. feat, talent, power, domain, style, school, bloodline, or equivalent class-granted choice pressure
6. prerequisite and invalid-choice blocking behavior
7. skill-rank and class-skill pressure where the supported slice exposes them
8. derived-output and explanation posture for claimed supported surfaces
9. spellcasting progression where the class requires it

## Class-specific burden table
| Class | Level-10 mandatory semantics that must be classified | Why breadth is counterfeit without them |
|---|---|---|
| Barbarian | rage posture, rage-power progression, movement/defense modifiers, trap sense / uncanny-dodge family through level 10 | class identity is inseparable from rage and level-based powers |
| Bard | bardic performance progression, known-spell posture, spell slots, class-skill and support-feature burden through level 10 | a selectable Bard without performance and spell burden is not meaningful support |
| Cleric | prepared divine spell posture, domain burden, channel-energy progression, class-feature and prerequisite pressure through level 10 | Cleric cannot be reduced to a non-caster chassis |
| Druid | prepared divine spell posture, nature-bond branch, wild-shape progression, class-feature burden through level 10 | Druid support is counterfeit if wild-shape and class-specific choice burden are absent |
| Fighter | bonus-feat progression, armor/weapon training milestones, prerequisite pressure, and derived combat surfaces through level 10 | current repo truth proves only Fighter level 1 and explicitly blocks level 2 |
| Monk | flurry and combat-style burden, AC bonus, ki pressure, maneuver/bonus-feat progression through level 10 | Monk is not truthfully supported if only generic attack/save math exists |
| Paladin | smite/lay-on-hands/divine-grace burden, mercy or similar class-feature progression, plus later spell burden through level 10 | a partial martial shell does not prove Paladin support |
| Ranger | favored-enemy/combat-style burden, skill/tracking burden, later spell burden through level 10 | Ranger cannot be marked supported from a generic martial shell alone |
| Rogue | sneak-attack progression, rogue-talent burden, skill pressure, trapfinding/evasion family through level 10 | current negative evidence already shows Rogue is not simply interchangeable with Fighter |
| Sorcerer | bloodline burden, known-spell posture, spell slots, spontaneous casting progression through level 10 | Sorcerer support is counterfeit without bloodline and spells-known truth |
| Wizard | spellbook/prepared posture, school or bonded-item burden, bonus-feat burden, slot progression through level 10 | Wizard support is counterfeit without real prepared-casting and class-choice truth |

## Class-family execution grouping
To keep slices truthful and bounded, later execution should usually separate these burden families:
- martial and skill-driven classes: Barbarian, Fighter, Monk, Rogue
- hybrid martials with spell burden: Paladin, Ranger
- spontaneous arcane burden: Bard, Sorcerer
- prepared or branch-heavy spell burden: Cleric, Druid, Wizard

## Milestone rule
Later execution slices do not need one test per every level immediately, but they do need explicit milestone coverage sufficient to make a level-10 claim honest for the targeted class.

Each handoff must therefore name:
- the exact milestone levels under test or classification
- the class features unlocked by those milestones
- what remains partial, lossy, blocked, or unverified after the slice

## Spellcasting floor rule
No spellcasting or hybrid class may be promoted to `supported` unless the handoff can classify at least:
- spell-source lineage
- known versus prepared posture where applicable
- slot/per-day progression where applicable
- class-specific spell burden such as domain, school, bloodline, or equivalent bounded core choice surfaces
- blocked or partial outcomes when the spell burden is incomplete

## Cross-cutting pressure rule
A class may still remain `partial` or `blocked` through level 10 even if its class-feature table exists, when any of the following remain unproven for the claimed surface:
- prerequisite gating
- feat or bonus-choice legality
- race/class interaction seams
- derived-output explanation surfaces
- blocked-choice diagnostics

## Current truthful floor as of 2026-06-30
- Fighter: `partial` at level 1 only; level 2 is explicitly `blocked` by live GE-06 tests
- Rogue: `blocked` for the bounded compute path where Rogue level 1 replaces the current pilot chassis in live GE-06 tests
- every other class: `unverified` at the SD-13 packet level until later slices name and prove them
