# SD15-E4-F8-D2 — Truthful launchability and blocker state

Date: 2026-07-01
Task: `t_35ac5a9b`
Scope: determine the real tranche-2 external-cycle state from the current evidence set, without inventing launch, participation, or closure meaning.

## Decision

The first bounded tranche-2 external cycle had one launchable row (`LNX-A`) with a governed build and clean-machine basis, but the cycle was not launched. The blocking condition is explicit: tranche-2 coding was still intentionally incomplete for external issuance, so the row-specific `LNX-A` operator packet was not issued and no external participant evidence exists yet.

## Evidence-backed answer by acceptance criterion

| Question | Answer | Source evidence |
|---|---|---|
| Is there at least one governed build? | Yes. `LNX-A` is tied to governed Linux alpha build `alpha-v0.0.0-c2cea5c6` from `origin/develop` commit `c2cea5c6baeb3ca34077b85331214c4b42a4809c`, with publication/provenance/checksum evidence. | `artifacts/tranche-2-install-and-use-matrix.md:53`; `artifacts/tranche-2-clean-machine-validation-report.md:66-75`; `artifacts/sd15-e4-f8-d1-tranche-2-external-cycle-evidence-inventory-2026-07-01.md:14-17` |
| Is there at least one launchable row? | Yes. Only `LNX-A` is grounded enough to launch. `LNX-S`, `MAC-A`, `MAC-S`, `WIN-A`, and `WIN-S` remain not ready. | `artifacts/tranche-2-external-test-cycle-plan.md:42-48`; `artifacts/tranche-2-external-test-cycle-plan.md:66-76`; `artifacts/tranche-2-external-test-cycle-report.md:15-20`; `artifacts/tranche-2-install-and-use-matrix.md:53-60`; `artifacts/sd15-e4-f8-d1-tranche-2-external-cycle-evidence-inventory-2026-07-01.md:13-15` |
| Is there at least one real participant or participant evidence packet? | No. Todd Hintzmann is named only as the intended first `LNX-A` tester. Actual participation remains zero, no row-specific operator-packet issuance event is recorded, and no external-tester attachment bundle exists. | `artifacts/tranche-2-external-test-cycle-plan.md:47-48`; `artifacts/tranche-2-external-test-cycle-plan.md:52-55`; `artifacts/tranche-2-external-test-cycle-report.md:20`; `artifacts/tranche-2-external-test-cycle-report.md:25-36`; `artifacts/tranche-2-external-test-cycle-report.md:61-65`; `artifacts/sd15-e4-f8-d1-tranche-2-external-cycle-evidence-inventory-2026-07-01.md:18-21`; `artifacts/sd15-e4-f8-d1-tranche-2-external-cycle-evidence-inventory-2026-07-01.md:39-44` |
| Is there enough evidence to name a real cycle state? | Yes. The current evidence names the bounded attempt as pre-launch and not launched: one grounded row existed, but the cycle remained deferred and external participation stayed at zero. | `artifacts/tranche-2-external-test-cycle-plan.md:42-48`; `artifacts/tranche-2-external-test-cycle-report.md:10-20`; `artifacts/sd15-e4-f8-d1-tranche-2-external-cycle-evidence-inventory-2026-07-01.md:13-20`; kanban comment on `t_d3eb4d73` from `default` at 2026-07-01 13:49 and repeated at 13:50 |
| What exact blocker prevents calling the cycle launched? | The governed Linux alpha path is ready on build/provenance/clean-machine basis, but the cycle was intentionally deferred until tranche-2 coding completion; because of that, the `LNX-A` operator packet was not issued and no participant evidence packet could exist yet. | `artifacts/tranche-2-external-test-cycle-plan.md:47-48`; `artifacts/tranche-2-external-test-cycle-report.md:16-20`; `artifacts/tranche-2-external-test-cycle-report.md:34-36`; kanban comment on `t_d3eb4d73` from `default` at 2026-07-01 13:49 and repeated at 13:50 |

## Minimal factual statement for downstream use

- cycle identifier: `SD15-EXT-2026-07-01-001`
- grounded launch row set: `LNX-A` only
- grounded governed build for that row: `alpha-v0.0.0-c2cea5c6`
- actual external participants: `0`
- current cycle state: `not launched`
- blocking condition: tranche-2 coding completion and subsequent row-specific `LNX-A` operator-packet issuance had not yet occurred at evidence capture time

## Explicit refusals

- Do not treat the existence of a plan, clean-machine receipt, or intended tester name as proof that the external cycle launched.
- Do not count pre-launch clean-machine evidence as external participation.
- Do not claim stable Linux, macOS, or Windows launchability from the grounded `LNX-A` row.
