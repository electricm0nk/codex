# SD-31 Technical Design

## Architectural surface

Unchanged from `SD-30-class-feature-archetype-bundle/technical-design.md`'s `class_feature`-ingestion
and archetype-wiring surface, extended to this package's additional kind lanes:

- `src/rules_core/rules_tables/<book>/` — per-class `class_feature` records (Epic 5); per-kind records
  for `monster`/`spell`/`race`/`race_trait` (Epic 6) and the 7 onboarded books (Epic 7).
- `archetype_resolver.rs` — supersession wiring (`archetype_claims_slot`) for measured classes (Epic 4);
  the chooser-interaction primitive once Epic 3-F3 designs it.
- `pilot_compute.rs` — per-class base computation, where Epic 3's measurement is performed and Epic 4's
  supersession branches land.
- `RaceCorpus` and its `resolve` path — the race chassis Epic 1 builds (the surface that currently
  returns `None` for ~2,894 `race_trait` units' races).
- The wiring-class classifier surface — Epic 2's verdict paths, bound by the accuracy-not-movement rule.
- `scripts/reachability_audit.py` — Epic 0's standing gate; imports the dashboard producer's own
  `doneness_verdict()` rather than reimplementing its table.
- `apps/desktop/src-tauri/src/reach_gate.rs` — the IPC builder every ingested record must reach through.
- `scripts/classify_race_trait_rows.py`, `scripts/classify_companion_rows.py`,
  `scripts/screen_pcc_load_gates.py` — the SD-29-inherited pre-cycle screening tools Epic 6 runs before
  any book claim.

## What this package does not touch

- The dashboard producer (`scripts/observer/pf1e_dashboard_producer.py`) and its `doneness_verdict()`
  table — SD-30's Epic 0 surface.
- `pi_screening.rs` and the declared-PI reader wiring into the ingest path — SD-30's Epic 3 surface,
  consumed (via the cross-SD gate) not modified here.
- The dashboard producer's `doneness_verdict()` table itself — read by Epic 0's audit, never rewritten
  by it. A new status word requires the generator change and the producer rule in the same commit, or
  the cron dashboard crashes rather than degrading (`SD-30 state-goals-and-lessons.md §1.3` hazard 4).

File-disjointness between the three split packages is by construction: SD-30's remaining epics touch
dashboard-producer/instrument/PI-screening code; this package touches `rules_tables/`/`pilot_compute.rs`/
`archetype_resolver.rs` for its five kinds; SD-32 touches race-chassis and classifier engine code. A
cycle that finds itself editing a file outside this list should stop and check whether it has drifted
into a sibling package's scope.
