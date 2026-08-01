---
title: GE-06 Pilot Charter Alignment
stc_id: STC-CODEX-GE-06
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts
source_stc: ../README.md
related:
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../../plans/spec-domains/GE-06-pilot-vertical-slice-pf1-human-fighter.md
  - ./ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
---

# GE-06 Pilot Charter Alignment

## Purpose
Record how GE-06 consumes the pilot charter, what the charter already settles, and when later GE-06 work must patch the charter or escalate to an ADR.

## Current alignment map
| Charter surface | GE-06 obligation | Current disposition |
|---|---|---|
| Slice identity: PF1 Core Rulebook Human Fighter level 1 | GE-06 must not broaden the first case. | Accepted and inherited unchanged. |
| Source content: Core Rulebook slice only | GE-06 must not import broader content as “just fixture setup.” | Accepted and inherited unchanged. |
| Character path: Human / Fighter / level 1 | GE-06 must keep the integrated fixture within this path. | Accepted and inherited unchanged. |
| Initial ability scores and named `power_attack` feat | GE-06 may treat these as grounded defaults. | Accepted and inherited; first-pilot additional feat/choice, skill, equipment, and active-state debt is now closed by `ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`. |
| Required evidence layers | GE-06 must bind source discovery, import, canonical model, rules computation, explainability, oracle comparison, and UI workflow into one proof contract. | Expanded into GE-06 acceptance and viability criteria. |
| Non-expansion rule | GE-06 must route scope growth into charter patch or ADR rather than hiding it in implementation. | Preserved as hard gate. |

## Current no-change result
As of this source-STC generation pass, GE-06 does **not** require a direct text patch to `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md`.

Reason:
- the charter already defines the correct pilot identity, initial acceptance target, and non-expansion rule
- the final deterministic input contract clarifies first-pilot selections inside the existing Human/Fighter/Core Rulebook boundary rather than expanding the pilot
- remaining GE-06 questions are proof-path, runtime evidence, parity, UI-minimum, and implementation routing questions that belong in this STC until new pilot scope is actually proposed

## Triggers that require a charter patch
A later GE-06 pass MUST patch the pilot charter if it changes any of the following:
- the source book boundary
- the pilot class/race/level identity
- the minimum output categories required by the pilot
- the statement of what the pilot is trying to prove
- the non-expansion rule itself

## Triggers that likely require an ADR
A later GE-06 pass SHOULD escalate to `programs/codex/doctrine/decisions/` when it:
- broadens the UI scope beyond the current minimum proof surface
- reclassifies the pilot as no longer requiring oracle evidence for selected outputs
- changes the ownership split between GE-06 and GE-07
- changes the meaning of viability in a way that affects roadmap or doctrine

## Final rule
GE-06 may clarify the pilot. It may not silently replace the pilot.
