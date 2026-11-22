# GE-07 Upstream Dependency Contract

## Purpose
This contract records what upstream artifacts GE-07 may rely on and what they still do not authorize.

## Upstream dependency map
| Upstream artifact | What GE-07 may rely on | What it does not authorize |
|---|---|---|
| `plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md` | GE-07 objective, in-scope/out-of-scope boundaries, required documents, required output classes, exit gates, and open questions | code authority, repo write scope, final framework choice, final command transport |
| `plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` | Stage D position, Tauri/TypeScript shell posture, and the strategic rule that GE-07 depends on GE-06 domain confidence | real GE-06 proof, narrow first shell slice, or any execution handoff |
| `requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` | the integrated pilot truth boundary and the rule that minimal UI truth consumes real outputs | broad GE-07 implementation authority or final shell architecture |
| `requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md` | explanation payload duties, invalid-choice visibility, and the rule that computation remains headless | UI-side recomputation, UX layout, or final explanation presentation |
| `requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md` | importer diagnostics, unsupported-token visibility, and provenance obligations | UI suppression of warnings or any guess about final import-surface transport |
| `doctrine/program-doctrine-and-scope-charter.md` | headless-core-first rule, anti-UI-first drift rule, and local-first product identity | permission to start coding without a bounded handoff |
| `doctrine/quality-gate-policy.md` | documentation gate and UI-truth gate criteria | a claim that those gates are already passed by the existence of this STC alone |
| `research/codex-reference-architecture-2026-06-17.md` | current Tauri/TypeScript/React preference, surface taxonomy, and shell-over-core composition model | final frontend binding, repo structure, or packaging implementation details |

## Downstream obligations imposed on GE-07 work
1. every later GE-07 slice must treat the UI as a consumer boundary over upstream domain truth
2. no later GE-07 handoff may compute authoritative values or explanation logic in the shell
3. any later UI slice that exposes parity, diagnostics, or provenance must preserve the upstream wording/truth instead of inventing softer product phrasing
4. any newly discovered command-boundary, model-home, or packaging authority change must propagate back into this source STC or a linked decision surface

## Non-authorizations
This source STC does not prove:
- GE-06 viability
- final desktop framework choice
- final repo directory layout for the UI shell
- exact transport shape for UI commands
- packaging/signing readiness
- code readiness for any broad desktop implementation lane
