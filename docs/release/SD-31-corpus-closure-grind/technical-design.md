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

**Corrected 2026-08-15 (launch-readiness remediation Step 5, drift D1).** The first bullet below was
true at split time and is no longer true: this package's own launch-readiness remediation (Step 4,
`progress.md` S4-dashboard) landed a narrow, one-time change to the dashboard producer. Left visible
per this program's doc convention; read it as "does not touch, except the one narrow correction
below."

- ~~The dashboard producer (`scripts/observer/pf1e_dashboard_producer.py`) and its `doneness_verdict()`
  table — SD-30's Epic 0 surface.~~ **Corrected:** this package's Step 4 remediation cycle
  (`195b237d3`) mapped the two previously-unmapped `('ambiguous', literal-/fixture-verified)` cells to
  `held` in `_doneness_verdict_uncapped`, and imported `scripts/observer/PF1e-dashboard.html` into
  version control (`2b232fe1d`) — both one-time, both landed, both closed. This package does not have
  a standing epic that owns the producer; SD-30's Epic 0 remains the surface's origin and the
  generator/producer same-commit discipline (below) still binds any future touch.
- `pi_screening.rs` and the declared-PI reader wiring into the ingest path — SD-30's Epic 3 surface,
  consumed (via the cross-SD gate) not modified here.
- The dashboard producer's `doneness_verdict()` table itself, beyond the one closed correction above —
  read by Epic 0's audit, never rewritten wholesale. A new status word requires the generator change
  and the producer rule in the same commit, or the cron dashboard crashes rather than degrading
  (`SD-30 state-goals-and-lessons.md §1.3` hazard 4).

**Corrected 2026-08-15 (launch-readiness remediation Step 5, drift D1).** The paragraph below
originally described a three-package split ("SD-32 touches race-chassis and classifier engine code").
`decisions.md §2` absorbed SD-32 into this package the same day the split-time text was written —
there is no sibling package to be disjoint from. Race-chassis (`RaceCorpus::resolve`) and classifier
engine code are this package's own Epics 1-2 surface now, listed in "Architectural surface" above.

File-disjointness is between this package and SD-30 by construction: SD-30's remaining epics (all
`COMPLETE`, closed 2026-08-14) touch dashboard-producer/instrument/PI-screening code; this package
touches `rules_tables/`/`pilot_compute.rs`/`archetype_resolver.rs`/`RaceCorpus`/the wiring-class
classifier for its own eight kinds and two capability tracks, **plus** (as of this package's own
launch-readiness remediation) `scripts/observer/pf1e_dashboard_producer.py`'s
`_doneness_verdict_uncapped` table and `scripts/observer/PF1e-dashboard.html` — that surface was
SD-30's original Epic 0 deliverable and this package touched it once, narrowly, to close the two
previously-unmapped `('ambiguous', literal-/fixture-verified)` cells and land the strict mandate
headline (`decisions.md §5`/§6, `progress.md`'s S4-dashboard receipt), not to re-open Epic 0's
charter generally. A cycle that finds itself editing a file outside this list should stop and check
whether it has drifted out of scope.
