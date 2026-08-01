# Intake-to-Triage Mapping

## Purpose
Define the operator-side bridge from SD-11 GitHub intake into the SD-15 triage taxonomy without rewriting tester-facing issue UX or inventing a second evidence schema.

## Boundary rule
- SD-11 remains the authoritative tester-facing intake contract and evidence-capture vocabulary.
- This document maps SD-11 intake outputs into SD-15 operator classification fields.
- The operator must point back to `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md` for intake field meaning rather than restating a new issue form here.
- Enhancement requests stay in the SD-11 enhancement lane unless the intake evidence proves a present-tense defect, unsupported path, or status/documentation drift condition that SD-15 must classify.

## Evidence partitions
| Evidence bucket | Source of truth | Typical fields | SD-15 use |
|---|---|---|---|
| Tester-supplied evidence | SD-11 intake payload | observed behavior, expected behavior, reproduction steps, tester goal, current friction, requested capability, redaction declaration, optional attachment notes | establishes what the tester claims happened and what they were trying to do |
| Auto-captured evidence | SD-11 evidence-capture matrix | build label/version, tester-facing channel/support label, platform / OS, current bounded workflow, current data-source identity, diagnostics/blocked claims when present, explanation/provenance refs when present | anchors the report to a concrete runtime context before operator judgment starts |
| Operator-added classification data | SD-15 triage lane | primary triage class, outcome state, adjacent-authority references, package/install context when relevant, operator provenance handle when available, support-state note, persistence/migration note, evidence sufficiency note, next required artifact or escalation path | turns intake into a durable triage record without changing the intake schema |

## Minimal operator-added fields
Every SD-15 triage record should add, at minimum:
1. primary SD-15 class from `triage-class-dictionary.md`
2. outcome state: `defect`, `unsupported`, `partial`, `not-yet-verified`, `blocked`, or `status-drift`
3. adjacent-authority reference used to justify the classification
4. evidence sufficiency note naming what is still missing when the outcome is `blocked` or `not-yet-verified`
5. escalation target or next artifact when more proof is required

## Routing sequence
1. intake arrives through the existing SD-11 bug or enhancement channel
2. operator checks that the mandatory SD-11 fields for that intake type are present
3. operator preserves the auto-captured runtime context exactly as received
4. operator selects the primary SD-15 class using `triage-class-dictionary.md`
5. operator records the adjacent authority that governs the classification
6. operator assigns the visible outcome state
7. operator points the report toward the next proof surface when needed: regression receipt, install/use matrix row, clean-machine report, external-test artifact, or reconciliation checklist

## Initial mapping matrix
| Intake shape or dominant symptom | Tester-supplied evidence to rely on | Auto-captured evidence to preserve | Operator-added classification data | Primary route |
|---|---|---|---|---|
| UI copy, rendering, navigation, status-surface, or explanation mismatch | observed behavior, expected behavior, reproduction steps, optional screenshot note | build label/version, channel/support label, platform/OS, current workflow, explanation/provenance refs when present | primary class `UI or presentation defect`; note whether contradiction is ordinary UI failure or status drift | SD-15 UI/presentation defect, or `status-drift` if the visible claim contradicts accepted authority |
| Wrong calculation, invalid gating, bad progression, or rules outcome | observed behavior, expected behavior, reproduction steps | build/channel/platform/workflow, diagnostics when present, current data-source identity | primary class `rules-engine defect`; affected class/race/level/choice; SD-13 support-state reference when relevant | SD-15 rules-engine defect unless SD-13 already marks the path unsupported or partial |
| Missing or incorrect content/data | observed incorrect content, expected source truth, reproduction steps | build/channel/platform/workflow, current data-source identity, diagnostics/provenance when present | primary class `content or data defect`; exact content identity when known; SD-13 breadth/support note when relevant | SD-15 content/data defect, or `partial`/`unsupported` if the gap is already an accepted breadth bound |
| Tester clearly hit a known unsupported, bounded, or partially supported path | observed behavior or blocked path, tester goal, reproduction steps | build/channel/platform/workflow, any diagnostics/warnings captured automatically | primary class `unsupported semantics`; adjacent authority proving the bound; visible outcome `unsupported`, `partial`, or `not-yet-verified` | SD-15 unsupported semantics / known unsupported paths |
| Artifact missing, wrong channel, broken publication, acquisition failure, rollback/withdrawal confusion | observed behavior, expected acquisition path, reproduction steps | build label/version, channel/support label, platform/OS | primary class `packaging or distribution defect`; package/install context; SD-12 authority reference; publication/provenance handle when available | SD-15 packaging/distribution defect |
| Build acquired but install, launch, workbench entry, or bounded use fails | observed behavior, expected behavior, reproduction steps | build/channel/platform/workflow, diagnostics when present | primary class `install/use defect`; exact failed step; environment note; clean-machine flag when known | SD-15 install/use defect unless the failure clearly belongs to persistence, rules, or content after entry |
| Save/load/reopen/revise/migrate/upgrade continuity failure | observed behavior, expected behavior, reproduction or migration steps, attachment/redaction declaration when save evidence exists | build/channel/platform/workflow, diagnostics when present | primary class `persistence/migration/saved-state continuity defect`; SD-14 reference; compatibility/update seam note when relevant | SD-15 persistence/migration defect |
| Repo README, program README, execution ledger, or visible status claims contradict one another or the receipts | observed contradiction, links/quotes to conflicting claims | build/channel/platform/workflow when the drift is tied to a concrete runtime receipt | primary class `status/documentation drift`; exact claim family in conflict; reconciliation target surface | SD-15 status/documentation drift |
| Enhancement request or general friction with no present-tense defect evidence yet | tester goal, current friction, requested capability | build/channel/platform/workflow, current data-source identity | keep as SD-11 enhancement unless evidence proves a current defect or unsupported path; note possible future SD-15 relevance only if the request exposes a live bound | remain SD-11 enhancement intake; do not force into SD-15 defect taxonomy |
| Mandatory SD-11 bug evidence is incomplete | whatever partial narrative exists | whatever runtime context was auto-captured successfully | candidate class if inferable, outcome `blocked` or `not-yet-verified`, explicit missing fields list | hold in SD-15 only as a blocked/unverified triage record pending evidence |

## Escalation and next-proof rules
- if the class is packaging/distribution or install/use, the next proof surface usually becomes the install/use matrix or clean-machine validation report
- if the class is rules/content/persistence, the next proof surface usually becomes a regression evidence receipt with the relevant SD-13 or SD-14 context attached
- if the class is status/documentation drift, the next proof surface becomes the reconciliation checklist and the contradicted adjacent authority references
- if the report remains `unsupported`, `partial`, `blocked`, or `not-yet-verified`, preserve that state explicitly; do not upgrade it to a defect merely to keep queues simple

## Explicit refusals
- do not introduce new tester-facing mandatory fields here; SD-11 already owns intake UX
- do not discard the SD-11 evidence split by merging tester-supplied and auto-captured fields into one untraceable blob
- do not treat missing operator-added classification data as proof that the intake itself was bad
- do not relabel enhancement intake as a bug merely because the desired feature is absent; only present-tense bounded failures belong in SD-15 triage
