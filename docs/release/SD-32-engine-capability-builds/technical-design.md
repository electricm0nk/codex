# SD-32 Technical Design

## Architectural surface

- `RaceCorpus` (data model + `resolve`) — Epic 1's chassis build lands here; the exact module path is
  determined by Epic 1-F1's design decision (cite the current implementation location at cycle-0 rather
  than assuming a path — this file does not hand-pin it, per the "generated/derived facts, never
  hand-maintained" discipline this program applies to file-location claims as much as figures).
- The wiring-class classifier (`ambiguous`/`display`+`grounded` resolution) — Epic 2's build. Location
  determined by Epic 2-F1/F2; likely sits alongside `scripts/observer/pf1e_dashboard_producer.py`'s
  `_doneness_verdict_uncapped()` table or as a sibling Python/Rust module, decided at design time.

## What this package does not touch

- `src/rules_core/rules_tables/<book>/` — no corpus content lands here; that is
  `SD-31-corpus-closure-grind`'s surface, consuming this package's chassis/classifier.
- The dashboard producer's `doneness_verdict()` table itself — SD-30's Epic 0 surface (though this
  package's classifier output feeds into what that table reads, the table's own schema/mapping stays
  SD-30's).
- `pi_screening.rs` and the PI-screening ingest path — SD-30's Epic 3 surface, not applicable here since
  this package writes no corpus content.

File-disjointness from the sibling packages is by construction: this package touches race-chassis data
model and classifier engine code; `SD-31-corpus-closure-grind` touches `rules_tables/`/
`pilot_compute.rs`/`archetype_resolver.rs`; SD-30's remaining epics touch dashboard-producer/instrument/
PI-screening code. A cycle that finds itself editing a file outside this list should stop and check
whether it has drifted into a sibling package's scope.
