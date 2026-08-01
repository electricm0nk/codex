# SD-13 Core Class/Race Roster and Level-10 Progression Matrix Epic Breakdown

## Breakdown rule
This file decomposes the SD-13 source STC into implementation-facing epics and feature seeds without becoming an execution handoff.

## Epic SD13-E1 — Support-state taxonomy, matrix schema, and seeded current truth
**Objective:** Define and later implement the control-plane surfaces that make bounded breadth claims honest before any broader roster code claim is attempted.

**Derived from:**
- SD-13 README: Objective / Readiness / Acceptance Summary
- `technical-requirements.md`: Support-state taxonomy requirements; Matrix row and evidence requirements; Breadth-claim gating requirements
- `artifacts/core-roster-and-support-state-matrix.md`
- `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`

**Depends on:**
- GE-06 current pilot proof ceiling
- SD-11 visibility boundary

### Feature seed SD13-F1 — Matrix schema and seeded current-state rows
**Outcome:** The exact core roster, support-state taxonomy, evidence tiers, and initial truthful rows are represented in a durable machine-usable surface.

**Acceptance signals:**
- exact seven-race and eleven-class roster encoded without ambiguity
- current Human Fighter level-1 ceiling and known blocked/unverified rows seeded truthfully

**Notes:**
- the first execution slice should start here rather than in a fake breadth implementation sprint

### Feature seed SD13-F2 — Breadth-claim composition policy
**Outcome:** User-facing or operator-facing breadth claims can be assembled from matrix truth instead of ad hoc language.

**Acceptance signals:**
- claim composition depends on race row, class row, and interaction rows where required
- partial/lossy/blocked/unverified states remain visible

**Notes:**
- this slice may couple to SD-11 later, but SD-11 does not own the taxonomy

## Epic SD13-E2 — Core race semantic coverage
**Objective:** Implement the bounded race-semantic support surface for the seven PF1 Core Rulebook core races without pretending every race/class combination is complete.

**Derived from:**
- `technical-requirements.md`: Race-semantic requirements
- `artifacts/core-roster-and-support-state-matrix.md`
- `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`

**Depends on:**
- SD13-E1
- GE-06 pilot input/provenance conventions where relevant

### Feature seed SD13-F3 — Baseline racial trait and modifier support
**Outcome:** Each core race can be classified against the named semantic families rather than merely listed.

**Acceptance signals:**
- every core race has explicit state for ability-modifier, trait, and bounded derived-output impact surfaces
- missing or deferred race traits remain visible in the ledger

### Feature seed SD13-F4 — Named interaction seam map
**Outcome:** Material race/class interaction seams are named explicitly rather than buried inside broad combination claims.

**Acceptance signals:**
- at least the obvious high-pressure seams are represented as interaction rows
- interaction rows explain why separate race and class rows are insufficient in those cases

## Epic SD13-E3 — Martial and skill-driven class progression through level 10
**Objective:** Implement the bounded level-10 progression surface for the non-full-caster core classes without conflating that work with spellcasting depth.

**Derived from:**
- `technical-requirements.md`: Class-progression requirements; Level-10 progression requirements
- `artifacts/level-10-progression-validation-contract.md`

**Depends on:**
- SD13-E1
- SD13-E2 where race interactions materially affect progression claims

### Feature seed SD13-F5 — Fighter, Barbarian, Monk, and Rogue level-10 burden slices
**Outcome:** The first non-full-caster classes can move upward from today’s narrow Fighter-only ceiling through named level-10 milestones and feature burdens.

**Acceptance signals:**
- class-feature milestones are named and tested/classified through level 10
- blocked or partial states remain explicit where the burden is not yet satisfied

### Feature seed SD13-F6 — Paladin and Ranger hybrid chassis baseline
**Outcome:** The hybrid martial-plus-spell burden begins with chassis/class-feature truth before later spell burden closes the class.

**Acceptance signals:**
- non-spell chassis and class-feature obligations are separated from the later spell burden
- the classes do not falsely become `supported` simply because the non-spell half improved

## Epic SD13-E4 — Spellcasting and hybrid class progression through level 10
**Objective:** Implement the full spell burden surface for the core spell-bearing classes.

**Derived from:**
- `technical-requirements.md`: Spellcasting-specific requirements; Level-10 progression requirements
- `artifacts/level-10-progression-validation-contract.md`

**Depends on:**
- SD13-E1
- SD13-E3 for hybrid chassis classes where non-spell posture must already be visible

### Feature seed SD13-F7 — Bard, Sorcerer, and Wizard spellcasting burden
**Outcome:** The spontaneous and arcane-prepared spell classes can be classified honestly through level-10 progression.

**Acceptance signals:**
- spell list/source lineage, slot or known/prepared posture, and class-specific choices stay visible
- lack of spell burden closure blocks `supported` claims explicitly

### Feature seed SD13-F8 — Cleric, Druid, Paladin, and Ranger spell burden
**Outcome:** The divine and hybrid spell classes can be classified honestly without flattening prepared-casting, domains, nature-bond, or partial-caster pressure.

**Acceptance signals:**
- each class preserves its class-specific spell or choice burden
- partial-caster classes do not inherit false support from the martial slice alone

## Epic SD13-E5 — Cross-cutting prerequisite, feat, skill, and derived-stat validation
**Objective:** Implement the shared validation surfaces that keep level-10 class support honest across the roster.

**Derived from:**
- `technical-requirements.md`: Prerequisite, feat, equipment, skill, and derived-stat requirements
- `artifacts/level-10-progression-validation-contract.md`

**Depends on:**
- SD13-E3
- SD13-E4

### Feature seed SD13-F9 — Prerequisite and invalid-choice blocking
**Outcome:** The system can classify class/race/feat choice pressure honestly instead of fabricating legal builds.

**Acceptance signals:**
- blocked choices remain diagnosable and claim-blocking where required
- support rows can cite real blocking evidence instead of vague “not ready” prose

### Feature seed SD13-F10 — Derived-output and explanation pressure
**Outcome:** Class/race progression claims remain tied to real computed outputs and explanations where the class burden requires them.

**Acceptance signals:**
- supported outputs cite explanation surfaces where required
- lack of explanation or derived-output truth remains visible in the matrix or ledger

## Epic SD13-E6 — Unsupported-depth diagnostics and tester-visible reporting
**Objective:** Keep roster debt visible to testers and operators without turning the UI lane into a second requirements authority.

**Derived from:**
- `technical-requirements.md`: Unsupported visibility requirements; Adjacent-lane boundary requirements
- `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md`

**Depends on:**
- SD13-E1
- SD-11 tester-workbench posture

### Feature seed SD13-F11 — Support-state and debt presentation contract
**Outcome:** Tester-facing or operator-facing roster-status surfaces can render SD-13 truth without inventing local rules.

**Acceptance signals:**
- visible states and debt language come from SD-13-controlled data/contract surfaces
- no UI-local support labels outrank the matrix

### Feature seed SD13-F12 — Evidence and issue-capture coupling
**Outcome:** Issue-reporting or audit surfaces can cite roster state, semantic debt, and blocking reason without folklore.

**Acceptance signals:**
- structured evidence payloads can reference the relevant matrix or ledger rows
- downstream reporting distinguishes blocked breadth truth from distribution or persistence failures

## Epic SD13-E7 — Breadth-claim audit and evidence-refresh posture
**Objective:** Preserve ongoing honesty once broader roster claims start appearing in live tester builds.

**Derived from:**
- SD-13 README: Acceptance Summary / Next Stage Rule
- `technical-requirements.md`: Breadth-claim gating requirements; Verification and proof obligations
- `artifacts/core-roster-and-support-state-matrix.md`

**Depends on:**
- SD13-E1
- SD13-E6

### Feature seed SD13-F13 — Evidence-refresh and stale-claim audit surface
**Outcome:** The program can tell when breadth claims have drifted away from current evidence.

**Acceptance signals:**
- claims can be checked against matrix rows and evidence timestamps/refs
- stale or unsupported product language can be flagged explicitly

### Feature seed SD13-F14 — Cross-lane propagation rules
**Outcome:** When breadth truth changes, downstream SD-11, SD-12, and SD-14 consumers can be updated without ambiguous ownership.

**Acceptance signals:**
- propagation triggers are explicit
- adjacent lanes consume updated breadth truth without seizing ownership

## Initial sequencing
1. SD13-E1 — Support-state taxonomy, matrix schema, and seeded current truth
2. SD13-E2 — Core race semantic coverage
3. SD13-E3 — Martial and skill-driven class progression through level 10
4. SD13-E4 — Spellcasting and hybrid class progression through level 10
5. SD13-E5 — Cross-cutting prerequisite, feat, skill, and derived-stat validation
6. SD13-E6 — Unsupported-depth diagnostics and tester-visible reporting
7. SD13-E7 — Breadth-claim audit and evidence-refresh posture

## Handoff boundary
No coding harness should act directly from this file. Each later execution slice must receive a dedicated handoff that names:
- exact repo paths
- exact allowed write scope
- exact required reads
- exact verification commands
- exact non-goals
- exact roster slice and semantic burden under claim

Any derived handoff file must also receive its own artifact card on the board.

## Completion gate
- [ ] every major SD-13 requirement family routes to at least one epic
- [ ] spellcasting burden is preserved as distinct from martial or simple chassis work
- [ ] matrix/taxonomy truth is first, not an afterthought
- [ ] unresolved questions stay in `risks-and-open-questions.md`, not hidden here
