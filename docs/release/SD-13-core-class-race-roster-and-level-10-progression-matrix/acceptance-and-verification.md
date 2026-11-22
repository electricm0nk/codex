# SD-13 Core Class/Race Roster and Level-10 Progression Matrix Acceptance and Verification

## Acceptance posture
This is a planning-ready documentary gate. The immediate proof burden is that the SD-13 packet defines bounded breadth truth concretely enough for later execution-story minting without inventing current implementation support.

## Documentation gate checks

### Gate A — Source STC shape exists
Acceptance:
- `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md` exist
- `references/upstream-dependency-contract.md` exists
- all named same-epic output artifacts exist under `artifacts/`

Verification:
- verify the file set exists under `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/`

### Gate B — Exact roster identity is explicit
Acceptance:
- the packet names the exact PF1 Core Rulebook core-race roster
- the packet names the exact PF1 Core Rulebook core-class roster
- the packet preserves PF1 Core Rulebook-only scope and single-class-through-level-10 posture

Verification:
- confirm `README.md`, `technical-requirements.md`, and `artifacts/core-roster-and-support-state-matrix.md` all name the same seven races and eleven classes
- confirm `Out of Scope` sections reject non-core books, archetypes, and multiclassing

### Gate C — Support-state taxonomy and debt visibility are concrete
Acceptance:
- the packet defines `supported`, `partial`, `lossy`, `blocked`, and `unverified`
- the packet separates support state from evidence tier
- the packet defines a visible debt ledger rather than hiding unsupported semantics

Verification:
- confirm `technical-requirements.md` and `artifacts/core-roster-and-support-state-matrix.md` define the taxonomy
- confirm `technical-design.md` preserves separate support-state and evidence-tier axes
- confirm `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md` defines row shape and seeded current-state entries

### Gate D — Current repo reality is grounded honestly
Acceptance:
- the packet states clearly that current accepted repo truth is still the Human Fighter level-1 pilot ceiling
- the packet records negative evidence that Rogue level 1 and Fighter level 2 are not presently supported by the bounded compute path
- the packet does not claim current core-roster or level-10 runtime support already exists

Verification:
- confirm README `Readiness`, `Closure State`, and `Target Runtime` mention the current branch/commit observations and GE-06 ceiling
- confirm `technical-requirements.md` `Current-state grounding` cites the deterministic fixture and the two GE-06 test files
- confirm `artifacts/core-roster-and-support-state-matrix.md` seeds current rows accordingly

### Gate E — Level-10 proof burden is class-aware
Acceptance:
- the packet defines universal progression dimensions through level 10
- the packet preserves a stricter burden for spellcasting classes
- the packet does not collapse level-10 support into “class selectable” or “class parsed”

Verification:
- confirm `technical-requirements.md` and `artifacts/level-10-progression-validation-contract.md` define universal and class-specific burden tables
- confirm Bard, Cleric, Druid, Paladin, Ranger, Sorcerer, and Wizard carry spellcasting-specific proof obligations

### Gate F — Adjacent authority surfaces remain separate
Acceptance:
- SD-13 does not counterfeit SD-11 tester-workbench authority
- SD-13 does not counterfeit SD-12 distribution/update authority
- SD-13 does not counterfeit SD-14 persistence/lifecycle authority

Verification:
- confirm README `Authority and Scope`, `In Scope`, and `Out of Scope` preserve the lane split
- confirm `artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md` names what each adjacent lane owns and what it does not inherit from SD-13 automatically
- confirm `references/upstream-dependency-contract.md` names what upstream surfaces do and do not authorize

### Gate G — Epic decomposition is truthful and bounded
Acceptance:
- every major requirement family routes into at least one bounded epic
- the epic list preserves separate slices for taxonomy/matrix, race semantics, spellcasting burden, cross-cutting validation, and visible debt/reporting
- no epic is itself an execution handoff

Verification:
- confirm `epic-breakdown.md` includes bounded epics and feature seeds rather than a fake “implement all breadth” sprint
- confirm `epic-breakdown.md` ends with an explicit handoff boundary rule

## Future implementation proof obligations
A later SD-13 execution handoff is acceptable only when it names exact repo paths, exact allowed write scope, and exact commands, and proves at least the relevant subset of these obligations:

### Matrix truth proof
- seed or update the matrix from real repo/runtime evidence
- preserve the taxonomy and evidence-tier separation
- refuse unsupported state promotion without named evidence

### Race-semantic proof
- exercise the named race semantic families for the targeted race slice
- verify blocked or partial semantics remain visible
- verify the race can change the relevant bounded outputs honestly

### Class progression proof
- exercise the named class through the required level milestones for the targeted slice
- verify class features, prerequisites, and derived outputs behave or classify honestly
- verify unsupported levels remain blocked or partial rather than silently computed

### Spellcasting proof
- exercise the named spellcasting or hybrid class burden for the targeted slice
- verify spells/slots/known/prepared posture and class-specific spellcasting choices remain visible
- verify non-spell chassis proof alone does not promote the class to `supported`

### Cross-cutting validation proof
- exercise prerequisite, feat, skill, and derived-stat pressures relevant to the targeted slice
- verify explanations and diagnostics remain structured and visible

### Breadth-claim and debt-visibility proof
- verify user-facing or operator-facing claim surfaces consume the matrix truth rather than inventing new labels
- verify partial, lossy, blocked, or unverified states remain visible where required

## Anti-counterfeit rules
- roster names alone do not prove breadth
- parsed or converted content alone does not prove class or race support
- a selectable UI option does not prove level-10 support
- a build artifact or shipped release does not promote a breadth row automatically
- a lossy path must not be reported as `supported`
