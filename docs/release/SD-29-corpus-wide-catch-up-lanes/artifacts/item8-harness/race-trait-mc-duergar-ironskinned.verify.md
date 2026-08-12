# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `race_trait` · record: `Ironskinned`
- expected on screen: `ironskin once per day`
- expected on screen: `Duergar`
- agent: `item8-harness` · date: 2026-08-11T22:30:26Z
- HEAD: `8b621552`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/item8-harness/race-trait-mc-duergar-ironskinned.png`
- rendered lines containing the record/expectations:
```
8:Corpus finding: 2 standard trait row(s) declare a multi-flag `!PREFACT` gate whose trailing flags the single-valued `suppressed_by_flag` field cannot hold; the resolver suppresses on the first flag only: Duergar ~ Spell-Like Ability ~ Enlarge Person (Duergar_ReplaceSLAEnlargePerson); Duergar ~ Spell-Like Ability ~ Invisibility (Duergar_ReplaceSLAInvisibility)
25:Duergar (7)
33:Duergar — standard traits
37:Duergar are hearty and observant, but also belligerent.
38:Duergar ImmunitiesB1
39:Duergar are immune to paralysis, phantasms, and poison. They gain a +2 racial bonus on saves against spells and spell-like abilities.
41:Duergar begin play speaking Common, Dwarven, and Undercommon. Duergar with high Intelligence scores can choose from the following languages: Aklo, Draconic, Giant, Goblin, Orc, and Terran.
43:Duergar are dazzled in areas of bright light.
45:Duergar are Medium creatures and have no bonuses or penalties due to their size.
47:Duergar have a base speed of 20 feet, but their speed is never modified by armor or encumbrance.
51:Duergar receive a +4 racial bonus to their Combat Maneuver Defense against bull rush or trip attempts while standing on the ground.
53:Duergar are humanoids with the dwarf subtype.
55:Duergar can see in the dark up to 120 feet.
62:Duergar have long warred against their dwarven cousins and the hated drow. Duergar with this racial trait receive a +1 racial bonus on attack rolls against humanoid creatures of the dwarf or elf subtypes. This racial trait replaces the invisibility spell-like ability.
70:Duergar spellcasters labor long to overcome the inborn spell resistance held by so many of their underground foes. Duergar with this racial trait receive a +2 racial bonus on caster level checks made to overcome spell resistance and a +2 racial bonus on dispel checks. This racial trait replaces the enlarge person and invisibility spell-like abilities.
72:Dwarf Traits (Replaces Duergar Immunities)ARG p.186
73:Replaces Duergar Immunities
74:Duergar can select any dwarf racial trait that replaces stability. They can select dwarf racial traits that replace the hardy dwarf racial trait by giving up duergar immunities instead.
78:Duergar can select any dwarf racial trait that replaces stability. They can select dwarf racial traits that replace the hardy dwarf racial trait by giving up duergar immunities instead.
80:IronskinnedMC
82:The soul of the earth infuses some duergar, giving them the ability to harden their skin. Duergar with this racial trait can use ironskin once per day as a spell-like ability, using their character level as their caster level. This racial trait replaces the enlarge person spell-like ability.
86:While most duergar have the power to become invisible, a few are instead able to extinguish the light around them. Duergar with this racial trait can use dust of twilight (Pathfinder RPG Advanced Player's Guide) once per day as a spell-like ability, using their character level as their caster level. This racial trait replaces the invisibility spell-like ability.
```
