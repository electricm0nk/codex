# item-8 on-screen verification — FAILED

- verdict: **FAILED** — record 'Agathion-Blooded' is NOT rendered on the race_trait screen — this is exactly the green-gate/empty-screen defect item 8 exists to catch
- family: `race_trait` · record: `Agathion-Blooded`
- expected on screen: `Agathion-Blooded`
- expected on screen: `Idyllkin possess bestial aspects`
- expected on screen: `Asura-Spawn`
- agent: `sd29-racetrait-r4` · date: 2026-08-12T13:46:06Z
- HEAD: `9176f869`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- extraction (first 40 lines of what WAS on screen):
```
⚙
Race Traits
Back
Standard traits
Alternate racial traits
Alternate racial traits from every ingested book — 283 alternate racial traits across 18 races. Choosing one replaces the standard trait it names; the engine resolves the swap, this screen only shows it.

Corpus finding: 2 standard trait row(s) declare a multi-flag `!PREFACT` gate whose trailing flags the single-valued `suppressed_by_flag` field cannot hold; the resolver suppresses on the first flag only: Duergar ~ Spell-Like Ability ~ Enlarge Person (Duergar_ReplaceSLAEnlargePerson); Duergar ~ Spell-Like Ability ~ Invisibility (Duergar_ReplaceSLAInvisibility)

Corpus finding: 4 ARG alternate(s) write the self-exclusion guard's negated branch as `!PREABILITY` where the operand is a fact flag, not an ability key; read as a guard anyway because the operand is the very flag the row sets: Half-Elf ~ Drow Magic; Half-Elf ~ Drow-Blooded; Half-Elf ~ Wary; Tengu ~ Deft Swords

Showing numbers for

Showing each trait as the book prints it. Pick a saved character to see the numbers their feats give them.
Human (33)
Dwarf (30)
Elf (28)
Half-Orc (28)
Halfling (27)
Gnome (23)
Half-Elf (20)
Tiefling (20)
Aasimar (17)
Goblin (10)
Hobgoblin (10)
Duergar (8)
Drow (7)
Kobold (5)
Orc (5)
Tengu (5)
Merfolk (4)
Svirfneblin (3)
Human — standard traits
6 traits apply. No alternate selected, so nothing is replaced.

+2 to One Ability ScoreCRB
Human characters get a +2 bonus to one ability score of their choice at creation to represent their varied nature.
Bonus FeatCRB
Humans select one extra feat at 1st level.
LanguagesCRB
```
