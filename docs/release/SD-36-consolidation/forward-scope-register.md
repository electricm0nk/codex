---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Forward Scope Register

Planned work deferred from SD-36. Each row is a **capability deferral** (not a blocker) — scoped work that was out of scope for this bundle but should land in a successor.

---

## Deferrals recorded during SD-36

| ID | Work | Reason | Target bundle | Notes |
|---|---|---|---|---|
| FS-1 | Semantic dedups in pilot_compute (fable review PC5-3/PC6-1/PC2-3) | Design work, requires adversarial review. Deferred for correctness focus bundle. | SD-36.5 or later (correctness phase) | Logged in SD-36 retrospective; blocks some compute optimizations but does not block release. |
| FS-2 | SD-34 fable-review P1 correctness rows (C2.5) | Separate correctness review bundle. 8 open P1s from fable review (doc link: `docs/release/SD-34-corpus-sheet-completion/references/README.md`). | Correctness bundle (post-SD-36) | Independent scope; should land before Starfinder bundle. |
| FS-3 | rules_tables → data package (181,797 lines) | Starfinder data-driven architecture requires rules as JSON/YAML, not Rust. Burn-down tied to `.lst` citation de-duplication (D6). | Starfinder data-package bundle | 8,592 `.lst` citations (provenance strings in rules_tables) burned down when Rust data becomes external package. Work inventory already frozen; this unblocks that transition. |
| FS-4 | pf1e_dashboard_producer.py extraction (if incomplete during B7) | Extract doneness.py library (~150 lines) from the producer if the extraction is clean; else defer whole producer deletion. | Immediate post-SD-36 fix cycle or next bundle | Recorded in Epic B7 deferral decision if not complete. |
| FS-5 | CI oracle fetch for converter test (optional per D1 Phase 0b) | Sparse clone via `scripts/fetch-pcgen-oracle.sh` in CI test job so gate proves something in CI. Currently: gate skips without checkout. | Next CI infrastructure bundle | Orthogonal to SD-36 scope; improves test coverage but not required for ship. |

---

## No deferrals for SD-36 blockers

Every acceptance criterion in `epic-breakdown.md` is scoped to this bundle. No work falls into the "open blocker" category (see `docs/governance/blocker-closure-doctrine.md`). The bundle is 100% scoped or explicitly deferred.

---

