# SD-13 Core Class/Race Roster and Level-10 Progression Matrix Risks and Open Questions

## Purpose
This document isolates the remaining uncertainty in the SD-13 breadth lane so the main packet can stay normative without pretending unresolved proof burdens are already settled.

## Active risks

### Risk 1 — Spellcasting breadth becomes counterfeit support theater
- why it matters:
  - spellcasting classes can look “present” long before slots, prepared/known posture, and class-specific spell semantics are real
- current exposure:
  - the repo proof ceiling is still Human Fighter level 1; no accepted spellcasting breadth lane exists yet
- preferred resolver:
  - keep spellcasting classes in a dedicated same-domain execution family and refuse blanket roster-complete language until the spell burden table is satisfied

### Risk 2 — The matrix collapses into fake binary badges
- why it matters:
  - a simple yes/no support badge would hide partial, lossy, blocked, and unverified states and recreate folklore
- current exposure:
  - the program already has a narrow pilot proof, so expansion pressure will naturally favor optimistic shorthand
- preferred resolver:
  - preserve separate support-state and evidence-tier axes and require the debt ledger everywhere support is summarized

### Risk 3 — Interaction pressure explodes into combinatorial scope
- why it matters:
  - a naive 7 x 11 race/class demand can turn one bounded lane into a fake “do everything” epic
- current exposure:
  - the tranche names both classes and races, so readers may infer all combinations are equal first-wave execution targets
- preferred resolver:
  - keep separate race rows, class rows, and named interaction rows; promote interaction rows only when they materially change support truth

### Risk 4 — Tester-facing wording drifts into a second authority surface
- why it matters:
  - if SD-11 invents its own “supported enough” language independent of SD-13, breadth truth will fork
- current exposure:
  - tester-facing workbench and issue flows are already defined in SD-11, but not yet coupled to an SD-13 matrix
- preferred resolver:
  - later coupling slices must consume SD-13 matrix output rather than invent new breadth labels locally

### Risk 5 — Later distribution or persistence work is mistaken for breadth proof
- why it matters:
  - broader distro/update or save/load behavior can create a false sense that class/race support is complete
- current exposure:
  - SD-12 and SD-14 sit immediately adjacent and are likely to consume SD-13 outputs later
- preferred resolver:
  - preserve the seam contract and require any downstream consumer to cite matrix rows rather than substituting adjacent-lane success for breadth truth

## Open questions
- What is the minimum truthful evidence floor required before a race or class may appear as “supported” in a tester-visible surface: `Computed`, `Oracle-checked`, or another explicitly named threshold per dimension?
- Which race/class seams deserve first-class interaction rows immediately, beyond the already obvious Human bonus feat/ability-bonus pressure?
- What minimum spell breadth is required before Cleric, Druid, Bard, Ranger, Paladin, Sorcerer, and Wizard may each move from `partial` or `blocked` into `supported` for level-10 claims?
- Which feat, equipment, and class-feature pressure points are mandatory in the first cross-cutting validation slice to keep the level-10 claim honest without turning the tranche into every possible build combination?
- Which later operational or audit surface should own periodic evidence refresh once tester-visible breadth claims exist in shipped builds?

## Explicitly deferred decisions
- multiclassing admission: deferred out of scope for this tranche
- non-core books, archetypes, prestige classes, and alternate racial traits: deferred out of scope for this tranche
- final UI wording and rendering surface for breadth states: deferred to later SD-11-coupled execution slices
- final persistence compatibility rules for broader class/race coverage: deferred to later SD-14-coupled work

## Forbidden shortcuts while questions remain open
- do not promote a class or race to `supported` by default because the open question feels administrative
- do not hide unresolved spellcasting burden inside a general “core roster” label
- do not let downstream UI, release, or persistence lanes answer these questions implicitly by implementation convenience
