---
title: GE-01 Acceptance and Verification
stc_id: STC-CODEX-GE-01
artifact_type: acceptance-and-verification
status: active
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix
source_stc: ./README.md
last_reviewed_at: 2026-06-19
---

# GE-01 Acceptance and Verification

These checks prove that the Codex GE-01 source STC is complete enough to govern downstream GE-02/GE-03 planning and to block premature coding execution.

## Verification snapshot

Closure pass date: `2026-06-19`

Verified live artifact counts:

| Artifact | Rows |
|---|---:|
| `artifacts/pilot-corpus-inventory.csv` | 66 |
| `artifacts/pilot-token-taxonomy.csv` | 26 |
| `artifacts/conversion-matrix.csv` | 29 |
| `artifacts/unsupported-token-ledger.csv` | 13 |

Closure finding:
- GE-01 documentary deliverables are complete for the PF1 Core Rulebook Human Fighter level 1 pilot boundary.
- Every pilot-critical token family in the taxonomy has an explicit conversion-matrix route.
- Deferred/lossy behavior remains visible in the unsupported-token ledger where relevant.
- The bundle remains non-code-authorizing.

## AT-01-001 — Source STC bundle completeness
**Given** the canonical Codex GE-01 source STC directory  
**When** the bundle is reviewed  
**Then** it contains `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, `epic-breakdown.md`, `references/oracle-surface-inventory.md`, `artifacts/pilot-corpus-inventory.csv`, `artifacts/pilot-token-taxonomy.csv`, `artifacts/conversion-matrix.csv`, and `artifacts/unsupported-token-ledger.csv`.

Evidence:
- source-STC directory listing or direct file reads
- parent index link from `programs/codex/requirements/README.md`

Status: **accepted**.

## AT-01-001a — Documentary artifact destinations are explicit
**Given** the GE-01 spec domain and technical requirements  
**When** produced artifacts are reviewed  
**Then** the epic and source STC name the exact files to generate and the exact bundle-relative destinations for inventory, taxonomy, matrix, ledger, and oracle-surface artifacts.

Evidence:
- `../../plans/spec-domains/GE-01-legacy-corpus-and-conversion-matrix.md`
- `technical-requirements.md` section TR-01-014

Status: **accepted**.

## AT-01-002 — Objective and boundary truth
**Given** the GE-01 README and technical requirements  
**When** the STC is inspected  
**Then** the objective, in-scope items, out-of-scope items, next-stage rule, and implementation block posture are explicit and consistent.

Evidence:
- `README.md`
- `technical-requirements.md`
- `risks-and-open-questions.md` closure findings

Status: **accepted**.

## AT-01-003 — Pilot corpus inventory exists and is bounded
**Given** the live PCGen corpus and GE-01 artifact directory  
**When** `artifacts/pilot-corpus-inventory.csv` is reviewed  
**Then** pilot-critical, supporting, adjacent, and candidate surfaces are separated, and the Human Fighter level 1 pilot files/entities are grounded with source evidence.

Evidence:
- `artifacts/pilot-corpus-inventory.csv`
- `risks-and-open-questions.md` OQ-01-001 and OQ-01-002 resolutions

Status: **accepted**.

## AT-01-004 — Token-family taxonomy and matrix coverage exists
**Given** `artifacts/pilot-token-taxonomy.csv`  
**When** rows with `pilot_criticality: critical` are compared against `artifacts/conversion-matrix.csv`  
**Then** every pilot-critical token family has an explicit conversion-matrix route.

Evidence:
- `artifacts/pilot-token-taxonomy.csv`
- `artifacts/conversion-matrix.csv`
- closure verification script output from this pass

Status: **accepted**.

## AT-01-005 — Unsupported behavior cannot disappear silently
**Given** the conversion matrix and unsupported-token ledger  
**When** deferred, lossy, formula-bearing, choice-bearing, or predicate-gated behavior is reviewed  
**Then** the behavior is visible in a matrix row and/or `artifacts/unsupported-token-ledger.csv` with owner, severity, mitigation, and evidence.

Evidence:
- `artifacts/conversion-matrix.csv`
- `artifacts/unsupported-token-ledger.csv`
- `risks-and-open-questions.md` downstream-deferred decisions

Status: **accepted**.

## AT-01-006 — Provenance obligations are explicit
**Given** the technical requirements, design, inventory, and matrix  
**When** provenance expectations are reviewed  
**Then** source lineage from PCC include through LST file/entity and source-span evidence is required for pilot-critical rows; downgrade paths remain explicit for future parser implementation.

Evidence:
- `technical-requirements.md` section TR-01-009
- `technical-design.md` Provenance Contract and schema notes
- `artifacts/pilot-corpus-inventory.csv`
- `artifacts/conversion-matrix.csv`
- `risks-and-open-questions.md` OQ-01-005 resolution

Status: **accepted**.

## AT-01-007 — Oracle discovery is defined, not assumed
**Given** the PCGen repository and GE-01 oracle inventory  
**When** oracle surfaces are reviewed  
**Then** PCGen comparison surfaces are treated as discovered evidence with explicit limits, not as invented guarantees.

Evidence:
- `references/oracle-surface-inventory.md`
- `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` lines 76-88 and 216-218
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java` lines 98-101
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/CommandLineArguments.java` lines 62-70 and following command-line argument definitions

Status: **accepted** for GE-01 discovery; GE-05 still owns implementation of an automated oracle runner.

## AT-01-008 — Downstream work is bounded
**Given** the epic breakdown  
**When** later work is routed  
**Then** the STC decomposes corpus inventory, taxonomy, matrix/ledger, oracle, and provenance-preserving import concerns into bounded downstream epics without becoming an execution prompt.

Evidence:
- `epic-breakdown.md`
- `README.md` next-stage rule

Status: **accepted**.

## AT-01-009 — Future runtime facts are not fabricated
**Given** the canonical README  
**When** target-runtime fields are reviewed  
**Then** known facts are named, unknown facts remain explicit, and code-authorizing execution-handoff derivation is blocked until repo/workdir/branch/write-scope facts are re-grounded.

Evidence:
- `README.md` Target Runtime section
- `README.md` Blockers / Forbidden Assumptions section
- superseded `execution-handoff.md`

Status: **accepted**.

## Exit gate checklist
- [x] Full rich source-STC bundle exists.
- [x] Documentary inventory/matrix/ledger/oracle artifact files exist at the expected paths.
- [x] Parent requirements index links to GE-01.
- [x] Objective and boundaries are explicit.
- [x] Pilot corpus inventory requirements are explicit.
- [x] Pilot corpus inventory artifact is populated and bounded for the Human Fighter level 1 slice.
- [x] Token taxonomy and conversion-matrix schema requirements are explicit.
- [x] Every pilot-critical token family has a conversion-matrix route.
- [x] Unsupported-token ledger requirements are explicit.
- [x] Deferred/lossy pilot semantics are visible in the ledger where relevant.
- [x] Provenance obligations are explicit.
- [x] Oracle discovery requirements are explicit and grounded in discovered PCGen surfaces.
- [x] Downstream epic routing exists.
- [x] No code-authorizing implementation handoff has been derived prematurely; the old execution handoff is superseded and inactive.

## Closure verdict
GE-01 is accepted as complete for its documentary pilot-boundary deliverables. It authorizes downstream requirements/modeling work, not code execution.
