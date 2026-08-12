---
title: GE-01 Risks and Open Questions
stc_id: STC-CODEX-GE-01
artifact_type: risks-and-open-questions
status: active
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix
source_stc: ./README.md
last_reviewed_at: 2026-06-19
---

# GE-01 Risks and Open Questions

## Closure finding
The 2026-06-19 closure pass resolved the GE-01 pilot-boundary questions that were recoverable from the live PCGen corpus and the existing Codex planning surface. GE-01 is complete for the PF1 Core Rulebook Human Fighter level 1 documentary boundary.

This file now preserves residual risks and downstream-deferred modeling decisions. Those deferred decisions do **not** block GE-01 closure because they belong to GE-02, GE-03, GE-04, or GE-05 as named below.

## Primary risks

| ID | Risk | Consequence | Mitigation | Closure disposition |
|---|---|---|---|---|
| R-01-001 | Pilot corpus boundary drift | The legacy inventory expands from a bounded PF1 pilot into broad Pathfinder archaeology. | Require pilot criticality on every inventory/taxonomy record and route non-critical discovery into deferred owners. | Mitigated for pilot closure through inventory role classification: `pilot-critical`, `supporting`, `adjacent`, and candidate variants. |
| R-01-002 | Unsupported-token silence | Import appears healthier than it is because unsupported or lossy behavior disappears into prose or code comments. | Require both conversion-matrix visibility and unsupported-token ledger capture. | Mitigated for pilot closure; deferred/lossy semantics are recorded in `artifacts/unsupported-token-ledger.csv`. |
| R-01-003 | Provenance collapse | Later debugging and parity review become impossible because lineage is not preserved at design time. | Require provenance obligations before parser work begins and forbid silent downgrade. | Mitigated for GE-01; artifacts preserve file paths, include edges, and source-span precision where recovered. |
| R-01-004 | Oracle folklore | Teams assume PCGen can be validated headlessly without proving which entry surfaces actually exist. | Treat oracle discovery as a first-class requirement with explicit trust limits. | Partially mitigated; a batch export path is documented and tested, but a later GE-05 runner must still implement validation. |
| R-01-005 | Counterfeit coding readiness | A coding harness receives this STC or a collection bridge artifact as if it were already a code-authorizing execution brief. | Keep this STC on the collection route; the superseded `execution-handoff.md` remains inactive; require later coding-route runtime grounding before code. | Mitigated for GE-01 closure. No code-authorizing handoff is active. |
| R-01-006 | Governance gap between legacy PCGen and Codex | Codex inherits useful doctrine from PCGen artifacts but lacks a fully native GE-00 governance surface. | Record the dependency honestly; if Codex governance diverges, derive a Codex-native GE-00 or equivalent doctrine surface upstream. | Not a GE-01 blocker; monitor at program governance layer. |

## Resolved pilot-boundary questions

### OQ-01-001 — Which exact PCC include path is the minimum pilot entry surface?
Answer:
The pilot entry surface is bounded by `core_rulebook.pcc` as the Core Rulebook campaign root plus the Human race package included through the Core Essentials race path.

Minimum documented chain for this GE-01 closure boundary:
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/_core_essentials.pcc`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/_race.pcc`

Disposition:
Resolved for GE-01 documentary closure. Later parser work may refine mechanical load ordering, but it must not reopen the basic pilot entry boundary without new evidence.

### OQ-01-002 — Which exact LST files are required for the pilot?
Answer:
For the GE-01 Human Fighter level 1 documentary slice, the pilot-critical files are recorded in `artifacts/pilot-corpus-inventory.csv` with `pilot_role: pilot-critical`. Supporting and adjacent surfaces are separated rather than collapsed into the critical set.

Pilot-critical file classes include:
- Core Rulebook PCC root and Human race PCC package
- base stat, save, variable, class, skill, feat, equipment, proficiency, and Human race/trait files
- entity-specific rows for Human, Fighter, Fighter class skills, Fighter proficiency grants, Human default traits, Human trait replacement gates, and armor/shield proficiency references

Disposition:
Resolved for GE-01 documentary closure. Full Pathfinder coverage remains intentionally out of scope.

### OQ-01-003 — Which token families are pilot-critical?
Answer:
Pilot-critical token families are recorded in `artifacts/pilot-token-taxonomy.csv` with `pilot_criticality: critical`. The closure pass verified that each critical family has an explicit route in `artifacts/conversion-matrix.csv`, and deferred/lossy semantics are additionally visible in `artifacts/unsupported-token-ledger.csv` where relevant.

Critical families include PCC include directives, `CLASS`, `RACE`, `SKILL`, `EQUIPMENT`, `ABILITY`, `AUTO`, `BONUS`, prerequisite guards, `CSKILL`, `PROFICIENCY`, `STARTFEATS`, `CHOOSE`, `KEYSTAT`, `STARTSKILLPTS`, `STATMOD / MODIFY`, `PREVARGTEQ / PREVAREQ / PREFACT`, `ABILITYPOOL`, and `PROFICIENCY:ARMOR / PROFICIENCY:SHIELD`.

Disposition:
Resolved for GE-01 documentary closure. GE-02 owns canonical model decisions for these families.

### OQ-01-004 — Which legacy oracle entry points are actually usable?
Answer:
A usable non-GUI oracle surface is documented and tested in the PCGen repository:
- `pcgen.system.Main` supports command-line flags including `--character`, `--exportsheet`, `--outputfile`, and `--configfilename`.
- `PcgenFtlTestCase` demonstrates batch export through `Main.main(...)` and XML comparison.
- `AGENTS.md` documents the same batch export path and example invocation.

Disposition:
Resolved enough for GE-01. GE-05 or a later validation-harness epic must still build the actual automated runner and fixtures.

### OQ-01-005 — What is the minimum acceptable source-span precision?
Answer:
For the first downstream implementation slice, file-level provenance alone is insufficient for pilot-critical semantics. GE-01 establishes this threshold:
- pilot-critical entity/token rows should preserve file path plus line or narrow line-range evidence where recovered;
- file-level evidence is acceptable only for broad include/catalog surfaces;
- if later parser work cannot preserve token spans immediately, it must record an explicit downgrade and retain file/line evidence from the GE-01 matrix.

Disposition:
Resolved for GE-01 documentary closure. Parser-level span fidelity remains GE-03 implementation work.

### OQ-01-006 — When should Codex derive a native GE-00 governance surface?
Answer:
Not required to close GE-01. A Codex-native governance surface may be warranted when program doctrine diverges from the inherited PCGen planning assumptions, but GE-01 has enough governance context to close the pilot corpus/matrix deliverable.

Disposition:
Not a GE-01 blocker. Track at program-governance level.

## Downstream-deferred decisions

The following remain intentionally deferred and do not block GE-01 closure:

- final canonical rules-model schema — owner: GE-02
- parser implementation details — owner: GE-03
- formula and prerequisite expression language — owner: GE-02 + GE-04
- choice/pool runtime behavior — owner: GE-02 + GE-04
- automated oracle runner implementation — owner: GE-05
- broad Pathfinder or multi-system coverage — owner: future expansion epics
- repo-local source layout inside a future Codex implementation checkout — owner: later coding-route handoff

## Forbidden assumptions

- GE-01 closure does not authorize repository scaffolding or import implementation.
- Local repo path, branch, or worktree facts for future coding work must still be grounded at execution time.
- Legacy token presence does not equal pilot importance unless the artifact taxonomy marks it as such.
- Unsupported or deferred behavior may not be ignored; it must remain visible through matrix or ledger surfaces.
- The batch export oracle surface is not the same as an implemented Codex validation harness.

## Review trigger

Reopen this file when any of the following occurs:

- the PF1 Human Fighter pilot scope changes
- later corpus scanning contradicts a pilot-critical classification
- GE-02 changes the canonical model boundary in a way that invalidates GE-01 mapping assumptions
- GE-03 parser work cannot preserve the required provenance evidence
- GE-05 oracle work proves the batch export path unusable for the intended fixtures
- a future code-authorizing execution handoff is being proposed
