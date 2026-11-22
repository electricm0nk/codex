---
title: GE-00 Program Governance and Scope Source STC
status: draft
scope: programs/codex/requirements/GE-00-program-governance-and-scope
artifact_type: source-stc
grand_epic: ../../plans/spec-domains/GE-00-program-governance-and-scope.md
roadmap: ../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
sources:
  - ../../research/pcgen-port-findings-2026-06-17.md
  - ../../research/codex-reference-architecture-2026-06-17.md
---

# GE-00 Program Governance and Scope Source STC

## Purpose

This source STC converts [GE-00 — Program Governance and Scope](../../plans/spec-domains/GE-00-program-governance-and-scope.md) into actionable technical requirements.

GE-00 exists to prevent Codex from becoming any of the following:

- a cosmetic PCGen UI port
- an unbounded Pathfinder migration
- a generic character-builder fantasy
- a rewrite that hides unsupported legacy semantics
- an implementation effort without source STCs

The program must first establish its doctrine, scope boundaries, artifact flow, decision rules, pilot slice, and quality gates.

## Source STC contents

- [technical-requirements.md](technical-requirements.md) — normative requirements for doctrine, scope, artifact flow, decision records, pilot-slice charter, and unsupported-token policy.
- [acceptance-and-verification.md](acceptance-and-verification.md) — testable checks that prove GE-00 is complete enough to unblock later spec domains.
- [risks-and-open-questions.md](risks-and-open-questions.md) — risks, unresolved decisions, and questions that must be answered or consciously deferred.
- [epic-breakdown.md](epic-breakdown.md) — implementation epics and feature seeds derived from this governance source STC.

## Authority surface

Canonical location:

```text
programs/codex/requirements/GE-00-program-governance-and-scope/
```

This is program-owned workspace documentation. It belongs under `programs/codex/requirements/` because it governs the Codex program and does not define global workspace doctrine.

## Program doctrine statement

Codex is not a port of PCGen. It is a rules platform with a modern character-builder UI that can ingest PCGen PCC/LST content as legacy source material while using the existing PCGen runtime as a migration oracle.

The decisive substrate is not the desktop shell. The decisive substrate is PCGen's PCC/LST content model, token semantics, loader behavior, runtime rules evaluation, and the ability to prove that imported behavior matches the old system where parity is claimed.

## Non-negotiables

1. **Headless core first** — domain capabilities must be runnable through CLI/tests before broad UI work.
2. **PCGen as oracle, not architecture** — PCGen may define expected behavior, but not the new internal model.
3. **Canonical rules model** — the target model must be clean, versioned, provenance-preserving, and not a direct LST syntax clone.
4. **Conversion matrix as control plane** — import coverage must be visible by token family and validation state.
5. **No unsupported-token silence** — unsupported, lossy, approximated, or intentionally ignored legacy semantics must be recorded explicitly.
6. **Vertical slice before breadth** — first proof is Pathfinder 1e Core Rulebook Human Fighter level 1, not broad Pathfinder support.
7. **Explainability is product behavior** — derived values and unavailable choices must be explainable from sources and rules.
8. **Source STCs before implementation** — spec domains do not become implementation work until their source STCs exist.

## Initial pilot slice

The initial pilot slice is:

> Pathfinder 1e Core Rulebook Human Fighter level 1, including race, class, ability scores, skills, feats, equipment, basic combat stats, saving throws, source lineage, import diagnostics, oracle comparison, and one exportable character summary.

This slice is narrow enough to finish but broad enough to test PCC loading, LST object parsing, prerequisites, formulas, effects, equipment, derived stats, explanations, and UI workflow pressure.

## Dependency position

- **Depends on:** existing PCGen research and reference architecture artifacts.
- **Unblocks:** GE-01 legacy corpus/conversion matrix, GE-02 canonical rules model, and all later source STCs.
- **Blocks:** implementation work until source STCs and quality gates are established.

## Derived artifacts

This source STC has been built out into these program artifacts:

- [Program Doctrine and Scope Charter](../../doctrine/program-doctrine-and-scope-charter.md)
- [Documentation Control Plane](../../doctrine/documentation-control-plane.md)
- [Quality Gate Policy](../../doctrine/quality-gate-policy.md)
- [Decision Record Scaffold](../../doctrine/decisions/README.md)
- [Decision Record Template](../../doctrine/decisions/ADR-0000-template.md)
- [PF1 Core Rulebook Human Fighter Level 1 Pilot Slice Charter](../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md)
- [Source STC Template](../templates/GE-XX-source-stc-template.md)

## Exit statement

GE-00 is complete when the team can state the program doctrine, name the pilot slice, explain why the work is not a PCGen UI port, follow the documentation path from spec domain to source STC to implementation epic/feature, and enforce the rule that unsupported legacy semantics cannot disappear silently.
