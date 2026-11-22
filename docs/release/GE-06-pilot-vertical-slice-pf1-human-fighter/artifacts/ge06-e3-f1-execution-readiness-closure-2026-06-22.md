---
title: GE06-E3-F1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E3-F1 — Selected parity-dimension adapter
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - ./ge06-e3-f1-execution-handoff-2026-06-22.md
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E3-F1 Execution Readiness Closure

## Verdict
GE-06 is now grounded sufficiently to mint the next narrow code-producing handoff for the selected parity-dimension adapter, and that paired artifact now exists.

The active E3-F1 code-authorizing artifact created from this readiness closure is:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-execution-handoff-2026-06-22.md
```

This readiness closure is not code authority. It records why the separate E3-F1 handoff now truthfully carries `code_authority: true` while the root `execution-handoff.md` remains a route surface.

## Core problem
GE06-E2-F3 proved one integrated headless receipt path for the accepted deterministic pilot, but GE-06 still lacks the next narrow bridge that turns that real receipt into a machine-checkable selected parity-dimension carrier for later GE-05 comparison work.

TR-06-010 requires GE-06 to define which outputs are mandatory comparison targets for viability, which may remain known gaps, and what evidence is required before anything can be called `Oracle-checked`. The smallest honest next move is not comparator logic or report writing. It is a bounded adapter that projects the merged receipt into the selected-dimension surface without broadening the claim tier.

## Selected bounded slice

```text
GE06-E3-F1 — Selected parity-dimension adapter
```

This slice should do only four things:

1. consume the merged GE06-E2-F3 headless receipt as read-only input
2. emit one narrow selected-dimension carrier for the mandatory pilot dimensions only
3. preserve current new-system values or references for those dimensions without inventing old-system comparison results
4. keep every emitted dimension below `Oracle-checked` until GE-05 comparison evidence exists

This slice does not authorize comparator logic, normalization, parity-report writing, PCGen execution, UI work, or rules-core rewrites.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Upstream merge truth | `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md` verifies `origin/develop` at `6977c86` and names the integrated headless receipt path as computed. |
| Live repo anchor | `git rev-parse origin/develop` returned `6977c862d7e0f40e105b0360ac34f36e18dccd43`; `git log -1 origin/develop` shows `6977c86 Merge pull request #13 from electricm0nk/ge06-e2-f3-headless-receipt-path`. |
| Baseline execution proof | `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet` passed (`2 passed`), and `"$HOME/.cargo/bin/cargo" test --quiet` passed in `/home/ubuntu/workspace/repos/codex`. |
| Receipt surface exists | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs` now exposes `HeadlessReceiptStatus`, `PilotHeadlessReceipt`, and `build_pilot_headless_receipt`. |
| Oracle-validation lane remains narrow | `/home/ubuntu/workspace/repos/codex/src/oracle_validation/mod.rs` currently exports only `golden_fixture`; there is no selected-dimension adapter yet. |
| GE-05 comparison carrier posture exists | `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs` and `tests/golden_case_fixture_schema.rs` already prove a bounded oracle-validation fixture lane that can consume stable dimension IDs without claiming parity passed. |
| Claim-tier / known-gap doctrine exists | `programs/codex/doctrine/quality-gate-policy.md`, `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md`, and `.../known-gap-policy.md` define `Computed` vs `Oracle-checked`, report status vocabulary, and known-gap rules. |
| Write-scope disjointness still holds | E3-F1 remains isolated to `src/oracle_validation/**` plus a new focused test, while E3-F2 remains isolated to `src/rules_core/**`; the first honest parallel pair is still collision-free. |

## Grounded implementation posture
Because the merged repo now has:

- one stable integrated headless receipt entry point
- one stable oracle-validation module boundary
- one stable golden-fixture schema lane for downstream comparison work
- passing receipt-path and full-suite baseline tests
- and no existing selected-dimension adapter surface

...the smallest truthful implementation is:

1. add one new `selected_parity_dimensions` module under `src/oracle_validation/`
2. update `src/oracle_validation/mod.rs` only to expose that module
3. add one focused test proving the selected-dimension carrier is derived from the merged receipt without promoting any claim above `Computed`

Anything broader would be counterfeit expansion.

## Expected selected-dimension boundary
The derived handoff should require a narrow carrier over exactly these mandatory pilot dimensions:

```text
character.identity
combat.baseline_melee_attack_bonus
defense.baseline_armor_class
defense.total_save.fortitude
defense.total_save.reflex
defense.total_save.will
skill.selected_modifier.climb
skill.selected_modifier.intimidate
skill.selected_modifier.swim
```

For the accepted deterministic fixture, the adapter must preserve the currently grounded new-system values or references behind those dimensions:

```text
combat.baseline_melee_attack_bonus = 5
defense.baseline_armor_class = 17
defense.total_save.fortitude = 4
defense.total_save.reflex = 2
defense.total_save.will = 1
skill.selected_modifier.climb = 5
skill.selected_modifier.intimidate = 3
skill.selected_modifier.swim = 5
```

`character.identity` must preserve the pilot identity carried by the merged receipt (`pf1-crb-human-fighter-level1` and `pf1.core_rulebook`).

Equivalent field names are acceptable if the emitted surface remains:

- machine-checkable in tests
- bounded to the selected dimensions above
- explicit that the claim-tier floor is `Computed`
- reusable by later GE-05 comparison work

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior foothold merged | pass | GE06-E2-F3 is verified on `origin/develop` at `6977c86`. |
| Bounded implementation slice selected | pass | E3-F1 is limited to a selected-dimension adapter over the merged receipt. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Reset to current `origin/develop`, then branch `ge06-e3-f1-selected-parity-dimensions`. |
| Allowed write scope explicit | pass | `src/oracle_validation/mod.rs`, `src/oracle_validation/selected_parity_dimensions.rs`, and `tests/ge06_selected_parity_dimensions.rs` only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD plus bounded scope. |
| Toolchain grounded | pass | Explicit cargo path works; targeted receipt test and full suite pass. |
| Verification commands known | pass | Exact RED/GREEN/VERIFY commands are named below. |
| Write scope remains parallel-safe with E3-F2 | pass | E3-F1 stays in `src/oracle_validation/**`; E3-F2 stays in `src/rules_core/**`. |
| Claim-tier boundary explicit | pass | E3-F1 may emit `Computed`-tier new-system carriers only; `Oracle-checked` remains forbidden. |
| Non-goals explicit | pass | Comparator, normalization, report-writer, PCGen, UI, and rules-core edits are all excluded. |
| Harness route explicit | pass | E3-F1 now has its own stage-specific execution handoff; the root route surface remains non-authorizing. |

## Authorized write scope for the derived handoff
The derived handoff may authorize writes only to:

```text
src/oracle_validation/mod.rs
src/oracle_validation/selected_parity_dimensions.rs
tests/ge06_selected_parity_dimensions.rs
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
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
programs/codex/doctrine/quality-gate-policy.md
```

If a compile break proves another file is required, stop and report the blocker rather than widening silently.

## Required TDD posture
The coding harness must:

1. create the failing `tests/ge06_selected_parity_dimensions.rs` test first
2. run the specific test and capture RED
3. implement the smallest selected-dimension adapter inside `src/oracle_validation/selected_parity_dimensions.rs`
4. update `src/oracle_validation/mod.rs` only to expose the new module
5. run the specific test and capture GREEN
6. re-run the receipt-path proof plus the existing golden-fixture schema proof
7. run full `"$HOME/.cargo/bin/cargo" test --quiet`
8. run a file-granular scope audit

## Explicit non-goals
The derived handoff must not authorize:

- edits to `src/rules_core/**`
- edits to `src/oracle_validation/golden_fixture.rs`
- parity comparator or pass/fail verdict logic
- normalization-rule behavior
- parity-report writer behavior
- PCGen execution or exporter capture
- `Oracle-checked` or broader compatibility claims
- UI/view-model or GE-07 work
- Cargo dependency changes
- changes to `tests/ge06_pilot_headless_receipt.rs` or `tests/golden_case_fixture_schema.rs`

## Claim tier after this slice
If the later E3-F1 handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
integrated headless receipt path: computed
selected parity-dimension carrier for the mandatory pilot dimensions: computed
oracle parity / pass-fail comparison / normalization / report-writer / UI truth: not yet
```

## Completion rule
This readiness closure is complete when the package truthfully records all of the following:

- E2-F3 remains the most recently merged GE-06 coding slice
- E3-F1 is now grounded enough for a code-authorizing handoff
- the paired `ge06-e3-f1-execution-handoff-2026-06-22.md` artifact exists and is awaiting Todd launch
- the root `execution-handoff.md` points at the live E3-F1/E3-F2 pair without becoming code authority itself
- any later E3-F1 implementation run must stay inside the oracle-validation lane, preserve the `Computed` claim-tier floor, and refuse comparator/report-writer expansion
