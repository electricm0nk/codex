---
title: GE06-E5-F2 Narrow-vs-Expand Decision
artifact_type: decision-trigger-review
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_route_surface: ../execution-handoff.md
selected_slice: GE06-E5-F2 — Narrow-vs-expand decision trigger
workflow_route: review
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
source_artifacts:
  - ./ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - ./ge06-post-e5-f1-decision-rack-2026-06-22.md
  - ../../GE-05-oracle-validation-and-parity-harness/README.md
  - ../../GE-05-oracle-validation-and-parity-harness/execution-handoff.md
  - ../../GE-05-oracle-validation-and-parity-harness/epic-breakdown.md
related_artifacts:
  - ./ge06-post-e5-f2-decision-rack-2026-06-22.md
  - ../execution-handoff.md
  - ../../GE-05-oracle-validation-and-parity-harness/execution-handoff.md
---

# GE06-E5-F2 Narrow-vs-Expand Decision

## Decision
Choose:

```text
narrow the pilot
```

Do not expand upstream requirements now.
Do not stop for architectural failure now.

## Why this is the correct branch
GE06-E5-F1 already fixed the current truthful posture at `computed-but-not-oracle-checked`.

That prior decision settled the key architecture-level question:
- the supported deterministic headless route survives with real `Computed` evidence
- no fatal model, importer, or engine collapse is exposed on the supported route
- the primary blocker to a stronger claim is named as `OracleGap`
- the UI truth gate is still unmet, but that is a separate optional spike lane rather than the decisive blocker to the next proof move

Because of that, GE06-E5-F2 does not need to re-litigate viability.
It only needs to decide what class of next move is justified.

The answer is narrow the pilot around the named blocker.

## Consumed input from GE06-E5-F1
| GE06-E5-F1 result | Effect on this decision |
|---|---|
| Outcome class remains `computed-but-not-oracle-checked` | Prevents counterfeit expansion to `pilot-viable` or product-visible language. |
| Primary blocker is `OracleGap` | Makes parity closure the next mandatory proof burden. |
| No fatal flaw is supported by current evidence | Refuses the "stop due to architectural failure" branch. |
| GE06-E4-F1 remains a bounded pre-viability spike posture | Refuses broad UI-first expansion as the default next move. |

## What "narrow the pilot" means here
Narrowing does not mean pretending the pilot is smaller than it is.
It means spending the next unit of authority only on the minimum path that can upgrade or honestly reject the current claim tier.

For GE-06, that minimum path is:

```text
close the selected-dimension oracle gap through the GE-05 parity surface
```

The next mandatory work should therefore point at the owning GE-05 surfaces, not at a vague "continue GE-06" instruction.

## Owning surfaces for the next move
### Primary owning STC
`programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md`

Why:
- GE-05 owns the comparison standard, claim-tier promotion rules, parity report shape, and known-gap policy
- GE-06 may consume parity evidence, but it does not own the parity machinery

### Primary route surface
`programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/execution-handoff.md`

Current grounded next candidate there:

```text
GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance
```

Why this is the next truthful bounded candidate:
- the GE-05 schema foothold already exists from merged `GE05-E2-F1`
- the remaining honest gap is to instantiate the first governed pilot fixture with real oracle evidence references, unresolved Codex output posture, and blocked/known-gap truth intact
- that fixture is the narrowest credible bridge from GE-06 `Computed` evidence toward GE-05 `Oracle-checked` evidence

### Secondary GE-05 follow-on terrain
After the governed fixture instance exists, later parity closure still belongs to GE-05 epic terrain:
- `GE05-E3` — output capture and normalization
- `GE05-E4` — comparator, diff reporter, and parity report writer
- `GE05-E6-F3` — GE-06 integration evidence handoff

This sequence belongs under GE-05 unless new evidence proves an upstream STC is insufficient.

## Rejected branches
### Rejected branch: expand upstream requirements now
This is not justified by the current evidence.

Why not:
- the blocker is not presently "we do not know what to build"
- the blocker is "the selected pilot dimensions are not yet oracle-checked"
- broadening GE-01, GE-03, GE-04, GE-06, or GE-07 now would spend authority before the parity surface has had a chance to prove whether a narrower path is enough

Expansion should happen only if later GE-05 comparison work exposes a concrete insufficiency in one of those owning STCs.
Until then, expansion would be narration masquerading as progress.

### Rejected branch: stop due to architectural failure
This is also not justified by the current evidence.

Why not:
- GE06-E5-F1 explicitly refused `fatal-flaw`
- the supported deterministic route computes on real code and tests
- the current limiting fact is missing parity evidence, not demonstrated impossibility

Stopping now would confuse an unclosed evidence gate with an architecture collapse.

## Queue and governance consequences
1. GE-06 remains at `computed-but-not-oracle-checked`.
2. GE06-E4-F1 remains optional and spike-only unless Todd explicitly wants UI-side evidence before parity closure.
3. GE06-E5-F3 becomes the next ready GE-06 documentary move so the no-change / delta propagation can be recorded explicitly.
4. The next mandatory implementation-facing burden moves to GE-05, starting from its route surface and bounded next candidate rather than from a new broad GE-06 coding brief.

## Explicit recommendation
The next mandatory bounded move should be framed as:

```text
Continue by narrowing through GE-05 parity closure, beginning with the GE-05 governed fixture-instance lane and only expanding upstream requirements if that parity work produces concrete evidence that an owning STC is insufficient.
```

## Non-authorizations
This decision does not authorize:
- broad UI-first expansion under GE-06
- declaring GE-06 `pilot-viable`
- declaring any selected dimension `Oracle-checked`
- declaring a new GE-03, GE-04, GE-06, or GE-07 requirements expansion already necessary
- treating the current lack of parity evidence as architectural failure

## Completion rule
This decision artifact is complete only if it leaves no ambiguity about five facts:

1. the chosen branch is `narrow the pilot`
2. GE06-E5-F1 was consumed rather than re-litigated
3. the next mandatory proof burden is routed to GE-05 parity ownership
4. broad upstream expansion is not yet justified
5. architectural stop is not yet justified
