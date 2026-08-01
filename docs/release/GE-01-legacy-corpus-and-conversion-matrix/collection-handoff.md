---
title: GE-01 Collection Handoff — Populate and Expand Documentary Artifact Set
stc_id: STC-CODEX-GE-01
artifact_type: agent-handoff
status: active
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix
source_stc: ./README.md
target_agent: god-emporer
work_type: data-collection
workflow_route: collection
readiness: collection-ready
---

# Agent Handoff: GE-01 Documentary Artifact Expansion

## Objective
Expand the GE-01 documentary artifact set toward pilot-complete coverage by collecting additional grounded facts from the live PCGen corpus and writing them into the existing GE-01 inventory, taxonomy, matrix, ledger, and oracle artifacts.

## Work Type
`data-collection`

## Workflow Route
`collection`

## Readiness
`collection-ready`

## Source STC
- path: `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md`
- source readiness: `collection-ready`
- authoritative for: GE-01 scope, artifact set, pilot boundary, provenance expectations, and documentary completion rules

## Downstream Target
- harness or workflow: `fresh god-emporer / Hermes session`
- invocation mode: `direct session`

## Required Reads
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md` — authoritative scope, artifact paths, and next-stage rule
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/technical-requirements.md` — exact schema and evidence obligations for inventory, taxonomy, matrix, ledger, and oracle surfaces
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/acceptance-and-verification.md` — what counts as documentary completion for this phase
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv` — current inventory baseline that must be extended, not replaced abstractly
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv` — current taxonomy baseline
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv` — current conversion baseline
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv` — current unresolved/deferred baseline
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md` — current oracle baseline

## Conditional Reads
- `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html` — if token meaning or token-family grouping is ambiguous
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/GenericLoader.java` — if loader behavior needs grounding for taxonomy or oracle entries
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/CampaignSourceEntry.java` — if campaign/include semantics need stronger grounding
- `programs/codex/plans/spec-domains/GE-03-pcgen-import-pipeline-and-provenance.md` — only if a GE-01 row needs a downstream owner and the owner is unclear

## Source Universe / Inputs
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/_race.pcc`
- pilot-relevant LST files referenced by those PCC files
- listfile docs and loader code only when needed to ground meaning or behavior

## Required Output Artifacts
Update these existing files in place:
- `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md`

## Route-Specific Constraints
- remain inside the PF1 Core Rulebook Human Fighter level 1 pilot boundary unless a row is explicitly marked adjacent/deferred
- every new row or note must be grounded by live source inspection
- preserve unknowns explicitly rather than filling them with plausible Codex-side guesses
- use GE-02 concept names only where they are already grounded by the GE-02 spec domain; otherwise mark the target concept as pending GE-02 refinement honestly
- do not pivot into GE-03 source-STC generation during this run

## Non-Goals
- do not write parser, importer, token-handler, CLI, or repo code
- do not create or update a GE-03 source STC in this run
- do not broaden the pilot to full Pathfinder coverage
- do not replace grounded rows with generic summaries

## Acceptance Criteria
- the five GE-01 artifact files above are updated with additional grounded coverage
- inventory coverage grows beyond the initial root/include skeleton toward more pilot-relevant file/entity visibility
- taxonomy, matrix, and ledger rows cite specific source files and, where available, source lines or bounded source spans
- oracle inventory distinguishes grounded oracle surfaces from merely candidate ones
- no row claims support, parity, or semantic certainty that the source evidence does not justify

## Verification
- read back representative slices from all updated artifacts
- verify that newly added rows contain concrete source paths and evidence notes
- verify that no output artifact path drift occurred
- summarize grounded additions separately from still-open documentary debt

## Allowed Assumptions
- `/home/ubuntu/workspace/repos/pcgen` remains read-only reference material
- the existing GE-01 artifact files are the canonical destination paths for this phase
- adjacent but non-pilot files may be inventoried if they are clearly labeled as adjacent/deferred

## Blockers / Forbidden Assumptions
- stop if the live PCGen source files needed for grounding are missing
- do not assume a token's Codex-side semantic home if GE-02 has not established it
- do not claim collection completeness merely because file existence was confirmed
- do not jump to the next spec domain just because the current one is documentary

## Notes for the Downstream Agent
- this handoff exists because GE-01 still owes its own documentary implementation outputs
- completion means producing better artifact content, not writing a recommendation memo
- if documentary completion exposes a future GE-03 dependency, record it in GE-01 artifacts or notes without changing the route of this run
