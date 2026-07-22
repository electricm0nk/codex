# SD-24 Epic 5 — Criterion 5.5: APG/ACG-Class Multiclass Deferral (Epic-5-side echo)

## Decision

**APG/ACG-class multiclass remains out of scope for SD-24 Epic 5.** Epic 5 ("Multiclass Stacking Real and Full") is delivered scoped to **Fighter + Wizard only**, level 1 → 10 advancement, per operator directive 2026-07-21 (`decisions.md §4`). No APG or ACG class participates in any of Epic 5's multiclass dispatch, test surface, or integration test.

This is not a new decision. Per `acceptance-and-verification.md`'s row for criterion 5.5 (verification method: "n/a (deferral artifact)"), this criterion's role is documentary: confirm, now that Epic 5 (criteria 5.1–5.4) has actually landed, that the deferral held throughout delivery and to record the closing cross-reference to Epic 4's own decision artifact.

**Source decision:** `../epic_4/apg-acg-multiclass-deferred.md` (criterion 4.5) is the canonical decision record and evidence base. This document does not re-derive that evidence — it confirms the decision was honored across Epic 5's actual delivery and closes the loop criterion 4.5 anticipated ("Criterion 5.5 ... should reference this document as its source decision rather than re-deriving it").

## Confirmation against Epic 5's landed work

Reviewed each of Epic 5's landed criteria (5.1–5.4) for APG/ACG-class touch:

| Criterion | Commit | APG/ACG touch? |
|---|---|---|
| 5.1 Fighter+Wizard multiclass dispatch | `0068818` | None. Touches only `pilot_compute.rs`, `level_up/fighter.rs`, `level_up/wizard.rs`. |
| 5.2 Deterministic 30-cycle test surface | (fighter-wizard-multiclass-deterministic-cycle) | None. `tests/sd24_multiclass_deterministic.rs` walks Fighter-solo, Wizard-solo, and Fighter+Wizard split-advance only. |
| 5.3 Integration test consumes ingested content | `b503c47` | None. `tests/sd24_multiclass_integration.rs` derives its oracle from `class_tables::good_saves_for`, exercised only for `ClassId::Fighter`/`ClassId::Wizard`. |
| 5.4 Multiclass dispatch four-check audit | `79162c1` | None. `tests/sd24_multiclass_dispatch_audit.rs` scopes the four-check audit to exactly `pilot_compute.rs`, `level_up/fighter.rs`, `level_up/wizard.rs`, `class_tables.rs` — the Fighter+Wizard dispatch surface. |

No cycle in Epic 5 added an `ApgClassId`/`AcgClassId` branch to `compute_class_chassis`, no `level_up::apg_*`/`level_up::acg_*` module was created, and no APG/ACG class appears in any Epic 5 test fixture. The deferral held for the entirety of Epic 5's delivery — confirmed by inspection of the diff for each of the four landed commits above (`git show --stat`), not merely by absence of a contrary claim.

## Scope consequence

Epic 5 is now fully delivered (criteria 5.1–5.5 complete) for its actual scope: Fighter + Wizard multiclass, level 1 → 10, with BAB best-progression stacking, best-fractional-progression saves, a 30-cycle deterministic test surface, an ingested-content-backed integration test, and a standing four-check dispatch-audit regression test. APG/ACG-class multiclass carries no SD-24 cycle-id and is not part of this bundle's closure scope.

## Follow-on delivery vehicle

Unchanged from criterion 4.5's decision: per `risks-and-open-questions.md §4 Q1`, the default follow-on vehicle is **SD-25, immediately following SD-24 closure**; the operator may pin a different bundle. `../epic_4/remediation-plan.md §5` remains the record of the 16 APG/ACG classes' unassigned named-feature gaps; extending Fighter/Wizard-style multiclass dispatch to those classes is out of scope for any SD-24 follow-on work and belongs with that same follow-on bundle.

## Override

`risks-and-open-questions.md §3` Flag `FLAG-B: APG-MULTICLASS-DEFER` remains **unset**. No cycle in Epic 5 required or requested the flag be set; the cycle picker correctly refused to dispatch any APG/ACG-class multiclass criterion throughout Epic 5's run, matching this document's confirmation above.

## Cross-references

- `../epic_4/apg-acg-multiclass-deferred.md` — criterion 4.5's canonical decision + evidence (this document's source)
- `../epic_4/remediation-plan.md` — the 16 APG/ACG classes' unassigned named-feature gaps
- `../epic_4/per-class-coverage-matrix.md` — the underlying coverage data
- `../../risks-and-open-questions.md §5 Deferrals` — bundle-level deferral ledger entry
- `../../risks-and-open-questions.md §3` — `FLAG-B` override flag
- `../../decisions.md §4` — Epic 5's Fighter+Wizard-only multiclass scope ADR
- `./fighter-wizard-multiclass-dispatch-cycle_cycle_receipt.md` (5.1), `./fighter-wizard-multiclass-deterministic-cycle_cycle_receipt.md` (5.2), `./integration-test-cycle_receipt.md` (5.3), `./multiclass-dispatch-four-check-audit_cycle_receipt.md` (5.4) — Epic 5's own landed cycle receipts, reviewed above for APG/ACG touch
