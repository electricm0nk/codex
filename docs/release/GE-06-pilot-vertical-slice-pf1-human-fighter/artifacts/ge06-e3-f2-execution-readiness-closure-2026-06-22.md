---
title: GE06-E3-F2 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E3-F2 — Failure classifier and owner mapping
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - ./ge06-e3-f2-execution-handoff-2026-06-22.md
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E3-F2 Execution Readiness Closure

## Verdict
GE-06 is now grounded sufficiently to mint the next narrow code-producing handoff for the failure classifier and owner-mapping lane, and that paired artifact now exists.

The active E3-F2 code-authorizing artifact created from this readiness closure is:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-execution-handoff-2026-06-22.md
```

This readiness closure is not code authority. It records why the separate E3-F2 handoff now truthfully carries `code_authority: true` while the root `execution-handoff.md` remains a route surface.

## Core problem
GE06-E2-F3 proved one integrated headless receipt path with computed-or-blocked status and claim-blocking diagnostics, but GE-06 still lacks the next narrow classifier surface that maps those receipt facts into one primary owner category.

TR-06-012 requires GE-06 to classify integrated failures into one primary category: model flaw, importer flaw, engine flaw, oracle gap, or UI gap. The technical design is explicit that the first implementation should classify by the first broken contract, not the last visible symptom, and it must refuse `IntegrationIssue` as a terminal bucket.

The smallest honest next move is a narrow rules-core-local classifier over the merged receipt facts. It must expose the full required vocabulary while only claiming the distinctions the current receipt surface can actually support.

## Selected bounded slice

```text
GE06-E3-F2 — Failure classifier and owner mapping
```

This slice should do only four things:

1. consume the merged GE06-E2-F3 headless receipt and its structured diagnostics as read-only input
2. emit one required primary owner from the GE-06 vocabulary
3. preserve optional contributing-owner context or rationale when useful
4. refuse a terminal `IntegrationIssue` bucket

This slice does not authorize a broad incident framework, parity comparator logic, report writing, UI work, importer rewrites, or rules-core redesign.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Upstream merge truth | `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md` verifies `origin/develop` at `6977c86` and names the integrated receipt path as computed. |
| Live repo anchor | `git rev-parse origin/develop` returned `6977c862d7e0f40e105b0360ac34f36e18dccd43`; `git log -1 origin/develop` shows `6977c86 Merge pull request #13 from electricm0nk/ge06-e2-f3-headless-receipt-path`. |
| Baseline execution proof | `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet` passed (`2 passed`), and `"$HOME/.cargo/bin/cargo" test --quiet` passed in `/home/ubuntu/workspace/repos/codex`. |
| Receipt-status / diagnostic surface exists | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs` now exposes `HeadlessReceiptStatus`, `PilotHeadlessReceipt`, and claim-blocking diagnostics on the merged receipt path. |
| Rules-core lane remains narrow | `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs` currently exports only `character_input` and `pilot_compute`; there is no dedicated failure-classifier surface yet. |
| First-broken-contract doctrine exists | `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md` defines the five owner categories and the first-broken-contract rule. |
| Current receipt can support a truthful first classifier | The merged receipt can already distinguish at least two meaningful states: a computed receipt with no comparison evidence yet (`OracleGap`) and a blocked receipt with claim-blocking rules diagnostics (`EngineFlaw`). Broader categories must remain part of the stable vocabulary without being fabricated from absent signals. |
| Write-scope disjointness still holds | E3-F2 remains isolated to `src/rules_core/**` plus a new focused test, while E3-F1 remains isolated to `src/oracle_validation/**`; the first honest parallel pair is still collision-free. |

## Grounded implementation posture
Because the merged repo now has:

- one stable integrated receipt entry point
- one stable computed-versus-blocked status surface
- one stable rules-core module boundary that does not yet include classification
- passing receipt-path and full-suite baseline tests
- and explicit GE-06 doctrine for primary-owner vocabulary

...the smallest truthful implementation is:

1. add one new `pilot_failure` module under `src/rules_core/`
2. update `src/rules_core/mod.rs` only to expose that module
3. add one focused test proving the classifier maps observable receipt states into the required vocabulary without inventing broader telemetry

Anything broader would be counterfeit infrastructure.

## Expected classifier boundary
The derived handoff should require a narrow classifier surface with this stable primary-owner vocabulary:

```text
ModelFlaw
ImporterFlaw
EngineFlaw
OracleGap
UiGap
```

The first implementation must remain honest about what the merged receipt can currently prove.

At minimum, the classifier must prove:

```text
supported deterministic receipt with computed outputs but no comparison evidence -> OracleGap
blocked deterministic receipt with claim-blocking rules diagnostics -> EngineFlaw
```

Equivalent type names are acceptable if the classifier remains:

- machine-checkable in tests
- explicit about one required primary owner
- free of an `IntegrationIssue` sink
- reusable by later GE-06 / GE-05 evidence routing without becoming a program-wide incident framework

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior foothold merged | pass | GE06-E2-F3 is verified on `origin/develop` at `6977c86`. |
| Bounded implementation slice selected | pass | E3-F2 is limited to a primary-owner classifier over the merged receipt surface. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Reset to current `origin/develop`, then branch `ge06-e3-f2-failure-classifier`. |
| Allowed write scope explicit | pass | `src/rules_core/mod.rs`, `src/rules_core/pilot_failure.rs`, and `tests/ge06_failure_classifier.rs` only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD plus bounded scope. |
| Toolchain grounded | pass | Explicit cargo path works; targeted receipt test and full suite pass. |
| Verification commands known | pass | Exact RED/GREEN/VERIFY commands are named below. |
| Write scope remains parallel-safe with E3-F1 | pass | E3-F2 stays in `src/rules_core/**`; E3-F1 stays in `src/oracle_validation/**`. |
| Vocabulary honesty preserved | pass | The full five-owner vocabulary is required, but only observable receipt states may be classified in the first implementation. |
| Non-goals explicit | pass | No incident framework, comparator, UI, importer rewrite, or oracle-validation edits are allowed. |
| Harness route explicit | pass | E3-F2 now has its own stage-specific execution handoff; the root route surface remains non-authorizing. |

## Authorized write scope for the derived handoff
The derived handoff may authorize writes only to:

```text
src/rules_core/mod.rs
src/rules_core/pilot_failure.rs
tests/ge06_failure_classifier.rs
```

It may read but must not modify these grounded surfaces:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_headless_receipt.rs
src/oracle_validation/golden_fixture.rs
tests/golden_case_fixture_schema.rs
Cargo.toml
Cargo.lock
AGENTS.md
CLAUDE.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md
programs/codex/doctrine/quality-gate-policy.md
```

If a compile break proves another file is required, stop and report the blocker rather than widening silently.

## Required TDD posture
The coding harness must:

1. create the failing `tests/ge06_failure_classifier.rs` test first
2. run the specific test and capture RED
3. implement the smallest classifier surface inside `src/rules_core/pilot_failure.rs`
4. update `src/rules_core/mod.rs` only to expose the new module
5. run the specific test and capture GREEN
6. re-run the receipt-path proof plus full suite verification
7. run a file-granular scope audit

## Explicit non-goals
The derived handoff must not authorize:

- edits to `src/oracle_validation/**`
- edits to `src/rules_core/pilot_compute.rs`
- a generic cross-program incident-management framework
- parity comparator or report-writer behavior
- UI implementation or GE-07 work
- importer or rules-engine rewrites disguised as classification work
- Cargo dependency changes
- changes to `tests/ge06_pilot_headless_receipt.rs` or `tests/golden_case_fixture_schema.rs`

## Claim tier after this slice
If the later E3-F2 handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
integrated headless receipt path: computed
narrow primary-owner classifier over the merged receipt surface: computed
oracle parity / UI truth / broad incident framework: not yet
```

## Completion rule
This readiness closure is complete when the package truthfully records all of the following:

- E2-F3 remains the most recently merged GE-06 coding slice
- E3-F2 is now grounded enough for a code-authorizing handoff
- the paired `ge06-e3-f2-execution-handoff-2026-06-22.md` artifact exists and is awaiting Todd launch
- the root `execution-handoff.md` points at the live E3-F1/E3-F2 pair without becoming code authority itself
- any later E3-F2 implementation run must keep the full five-owner vocabulary, classify only what the merged receipt can actually support, and refuse counterfeit incident-framework expansion
