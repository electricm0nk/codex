# SD-31 Technical Design

## Architectural surface

Unchanged from `SD-30-class-feature-archetype-bundle/technical-design.md`'s `class_feature`-ingestion
and archetype-wiring surface, extended to this package's additional kind lanes:

- `src/rules_core/rules_tables/<book>/` — per-class `class_feature` records (Epic 3); per-kind records
  for `monster`/`spell`/`race`/`race_trait` (Epic 4) and the 7 onboarded books (Epic 5).
- `archetype_resolver.rs` — supersession wiring (`archetype_claims_slot`) for measured classes (Epic 2);
  the chooser-interaction primitive once Epic 1-F3 designs it.
- `pilot_compute.rs` — per-class base computation, where Epic 1's measurement is performed and Epic 2's
  supersession branches land.
- `apps/desktop/src-tauri/src/reach_gate.rs` — the IPC builder every ingested record must reach through.
- `scripts/classify_race_trait_rows.py`, `scripts/classify_companion_rows.py`,
  `scripts/screen_pcc_load_gates.py` — the SD-29-inherited pre-cycle screening tools Epic 4 runs before
  any book claim.

## What this package does not touch

- The dashboard producer (`scripts/observer/pf1e_dashboard_producer.py`) and its `doneness_verdict()`
  table — SD-30's Epic 0 surface.
- `pi_screening.rs` and the declared-PI reader wiring into the ingest path — SD-30's Epic 3 surface,
  consumed (via the cross-SD gate) not modified here.
- Any race-chassis or verdict-path-classifier engine code — `SD-32-engine-capability-builds`'s surface.

File-disjointness between the three split packages is by construction: SD-30's remaining epics touch
dashboard-producer/instrument/PI-screening code; this package touches `rules_tables/`/`pilot_compute.rs`/
`archetype_resolver.rs` for its five kinds; SD-32 touches race-chassis and classifier engine code. A
cycle that finds itself editing a file outside this list should stop and check whether it has drifted
into a sibling package's scope.
