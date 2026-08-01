---
title: GE-09 Expansion Scope Selection Policy
stc_id: STC-CODEX-GE-09
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts
source_stc: ../README.md
related:
  - ../../../doctrine/quality-gate-policy.md
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - ../../GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md
  - ../../GE-08-homebrew-authoring-and-rules-studio/README.md
---

# GE-09 Expansion Scope Selection Policy

## Purpose
Define how Codex decides what, if anything, may expand next without outrunning current evidence, known-gap truth, or the pilot's explicit claim ceiling.

## Current gate posture
GE-09 now has the upstream facts it was previously waiting on:
- GE-06 has an explicit verdict: `computed-but-not-oracle-checked`, not `pilot-viable`.
- GE06-E5-F2 and GE06-E5-F3 make the routing consequence explicit: the next mandatory proof burden belongs to GE-05 parity ownership, not to broadening GE-06 or GE-09 scope.
- GE-08 has a planning-ready source STC, but no code-authorizing handoff or grounded contributor workflow.

Therefore the top-ranked outcome today is not “expand now.”
It is:

```text
hold scope-broadening expansion and keep GE-09 documentary until GE-05 parity evidence changes the claim ceiling
```

## Evidence basis available for ranking
Current grounded inputs are still pilot-bounded:
- GE-01 conversion matrix: 29 rows.
- Support disposition mix: 15 `partial`, 14 `deferred`.
- Lossiness mix: 12 `high-risk`, 8 `medium-risk`, 5 `none-expected`, 4 `unknown-risk`.
- GE-01 unsupported-token ledger: 13 entries total, with 2 `high`, 9 `medium`, and 2 `low` severity items.
- The only explicitly inventoried non-pilot broadening lane in GE-01 today is the adjacent Core Rulebook domain cluster named from `core_rulebook.pcc`: deities, domains, spells, companions, and magic-item surfaces.

This means GE-09 may rank candidate bands, but it may not pretend the current evidence supports a fine-grained book-by-book expansion backlog.

## Ranking rule
Rank candidate bands by this order of precedence:
1. claim-safety impact — does the candidate reduce the risk of counterfeit compatibility or scope claims?
2. leverage over multiple current pilot rows or known gaps — does the work unlock many later truths rather than one flashy lane?
3. dependency readiness — can the candidate proceed from existing GE-02 / GE-03 / GE-04 / GE-05 / GE-08 authority without inventing missing workflow?
4. scope-broadening cost — how much new unsupported surface would the candidate force the program to narrate before it can verify it?
5. operator value — only after the above truth conditions are satisfied.

Popularity, nostalgia, book order, and vague “support more Pathfinder” language are forbidden ranking inputs.

## Current ranked candidate bands

| Rank | Candidate band | Evidence basis | Why it ranks here now | Current action |
|---|---|---|---|---|
| 0 | No scope-broadening launch while parity remains open | GE-06 explicit verdict plus GE06-E5-F2 / E5-F3 propagation truth | The current claim ceiling is still `computed-but-not-oracle-checked`; broadening scope now would spend authority before the decisive parity gate closes | Keep GE-09 documentary and rerank only when trigger conditions fire |
| 1 | Formula / stat / save / class-initialization mechanics foundation | Conversion-matrix rows covering Fighter formulas, base stats, save progression, and STARTSKILLPTS; unsupported-ledger entries for formula-bearing progression, base-stat formulas, and Fighter skill-point chain | This cluster carries the highest-severity open mechanics debt and touches many future claim surfaces at once | Treat as the first scope-deepening preparation band after parity improves; do not market it as broad expansion |
| 2 | Predicate / choice / proficiency-link semantics | Conversion-matrix rows for PRE* guards, CHOOSE+MULT, AUTO grants, equipment proficiency references, and class-skill carrier/type-target surfaces; unsupported-ledger entries for prerequisite chains, martial proficiency choice semantics, class-skill type targets, and equipment-proficiency references | Cross-cutting relational semantics must be honest before later packages can claim stable compatibility | Keep as the next documentary/stabilization band after rank 1 |
| 3 | Human race-trait and choice composition surfaces | Conversion-matrix rows for Human carrier indirection, STARTFEATS, ABILITYPOOL, replacement flags, and template identity; unsupported-ledger entries for Human trait indirection, replacement gates, and choice pools | Still pilot-adjacent and medium-risk, but narrower in leverage than the mechanics foundation above | Preserve as a bounded later band, not as a first broadening move |
| 4 | Adjacent Core Rulebook non-pilot domains already inventoried in GE-01 | Unsupported-ledger row naming deities, domains, spells, companions, and magic-item surfaces from `core_rulebook.pcc` | This is the first truly scope-broadening candidate family grounded today, but GE-01 only inventories it at cluster level and GE-06/GE-05 have not yet lifted the current evidence ceiling | Documentary planning only until ranks 0-3 improve and a narrower expansion STC/update names exact scope |
| 5 | Human biosettings and authoring-linked adjuncts | Unsupported-ledger row for Human BIOSET support surface plus GE-08 source-STC posture | Lowest readiness: limited evidence value today and explicit dependence on later authoring/character-editing truth | Keep deferred; do not let it distort near-term expansion sequencing |

## Band interpretation rule
Ranks 1 through 3 are not book-expansion campaigns.
They are scope-deepening preparation bands that strengthen truth surfaces inside or immediately adjacent to the current pilot boundary.

Rank 4 is the first honest broadening band.
It stays documentary until GE-05 parity evidence, GE-06 claim posture, and a narrower scope declaration say otherwise.

## Required candidate packet for any future rerank
Every future candidate or candidate band MUST name:
- candidate or band ID
- exact source package or book scope
- affected conversion-matrix rows or an explicit statement that new rows must be added first
- known-gap and unsupported-ledger references
- current compatibility-language ceiling
- dependency owners across GE-02 / GE-03 / GE-04 / GE-05 / GE-08
- review trigger that would promote, block, or downgrade the candidate
- whether the candidate is scope-deepening or scope-broadening

## Review cadence
Primary cadence is event-driven, not calendar theater.

Immediate rerank triggers:
- a GE-05 parity artifact changes the gap class or raises the evidence ceiling for the pilot
- GE-06 publishes a stronger or weaker propagated claim posture than `computed-but-not-oracle-checked`
- GE-02 / GE-03 / GE-04 close one of the current high-leverage mechanics clusters above
- GE-08 publishes a narrower authoring readiness closure or an accepted contributor-path decision
- a doctrine decision changes compatibility-language, release-authority, divergence, signing, or distribution policy

Fallback cadence:
- if none of the above triggers fire, perform a monthly GE-09 documentary review to confirm that the ranked bands still match the latest matrix and ledger truth

## Explicit non-authorizations
This policy does not authorize:
- launching a broad expansion campaign now
- claiming support for a new book or package family without new conversion-matrix rows and a parity/validation plan
- translating the existence of a ranked band into code authority
- treating GE-08's planning-ready STC as a settled contributor workflow

## Completion rule
This policy is complete for GE09-E1 only if a future session can answer three questions without improvising:
1. Why is broadening scope not the first move today?
2. Which candidate bands are ranked next once the gate changes?
3. What exact events force a rerank instead of letting the ranking silently drift?
