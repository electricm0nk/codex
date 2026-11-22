---
title: GE-01 Superseded Execution Handoff
stc_id: STC-CODEX-GE-01
artifact_type: execution-handoff
status: superseded
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix
source_stc: ./README.md
superseded_by: ./collection-handoff.md
---

# Superseded Artifact — Do Not Use As The Active Route

This file is retained as an audit trail because an earlier workflow pass misrouted GE-01 into downstream GE-03 source-STC generation before GE-01's own documentary deliverables were built.

## Why it was wrong
- it treated "GE-01 is not the importer" as if the next action should automatically be "generate the GE-03 STC"
- it used an `execution-handoff.md` shape for work that was not a coding-route handoff
- it failed to prioritize GE-01's own concrete documentary outputs: inventory, taxonomy, conversion matrix, unsupported-token ledger, and oracle inventory

## Correct active route
Use:
- `./collection-handoff.md`

That artifact is the authoritative downstream brief for continuing GE-01 same-epic documentary artifact generation.

## Forbidden use of this file
Do not use this file to:
- generate a GE-03 source STC by default
- infer coding authority
- skip GE-01 artifact completion because later epics eventually own code
