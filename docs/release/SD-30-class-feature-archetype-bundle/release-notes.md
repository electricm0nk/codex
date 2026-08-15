# SD-30 Release Notes

**Populated `SD30-E9-F1-001`, 2026-08-14.** This bundle's scope was narrowed mid-run
(`decisions.md §51`, 2026-08-14): Epics 4/5/6/10/11 (per-class measurement, archetype mechanism,
chassis-sweep ingest, corpus-wide ingest lanes, book onboarding) moved to
`SD-31-corpus-closure-grind/`; Epics 12/13 (race chassis, verdict-path capability) moved to
`SD-32-engine-capability-builds/`. What follows describes what **this** package, narrowed to Epics 0,
1, 2, 3, 7, 8, 9, actually delivered — not the wider sixteen-book/all-kinds charter this package
carried at points in its own history. The per-cycle receipts in `progress.md` are the per-record
evidence; this document summarizes the release.

**Status at population: NOT CLOSED.** `epic-8-code-review` has not been claimed or started
(confirmed by content: no `SD30-E8` receipt exists anywhere in `progress.md`, `kanban.md`'s
`epic-8-code-review` row is still `READY`). Per this package's own hard rule ("If ANY live card is not
complete by content, DO NOT open the promotion PR"), the tranche-promotion PR is **not** opened by this
cycle. This document is populated now so the closing cycle does not have to re-derive it from scratch;
the "Summary"/"tranche promotion PR" lines below are written as of this cycle and must be re-confirmed,
not re-derived, once Epic 8 actually lands and the PR is opened.

## Summary

- **Bundle:** SD-30 — Class-Feature/Archetype Bundle (narrowed 2026-08-14 to the gate/process epics).
- **Branch:** `tranche/10`.
- **Version:** `0.10.0` committed at HEAD (`apps/desktop/package.json`, `tauri.conf.json`,
  `src-tauri/Cargo.toml`, re-derived this cycle: `grep -h '"version"'
  apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json` → `"0.10.0"` in both; `grep
  'version = ' apps/desktop/src-tauri/Cargo.toml | head -1` → `"0.10.0"`). The build counter
  (`GITHUB_RUN_NUMBER`) substitutes for the final digit at publish time; the repo carries the base
  triple only.
- **HEAD at population:** `33ef64fe` (`git rev-parse HEAD`).

## What SD-30 delivered

- **Epic 0 — instrument application to `held` (`decisions.md §43`/`§46`, Job 1).** Landed two new
  `done` rungs — `literal-verified` (corpus-literal byte-equality sweep) and `fixture-verified`
  (derived-evaluator-vs-fixture check) — closing the static/derived "no `done` rung" measurement gap
  SD-32 had left open. Board `done` moved **3,464 → 5,837** (+2,373), re-confirmed this cycle by
  importing the live dashboard producer's own `doneness_verdict()` (not transcribed) and replaying it
  over `docs/work-inventory.json`:
  ```
  python3 -c "
  import json, importlib.util, collections
  spec = importlib.util.spec_from_file_location('m', 'scripts/observer/pf1e_dashboard_producer.py')
  mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
  d = json.load(open('docs/work-inventory.json'))['units']
  c = collections.Counter()
  for u in d:
      if u.get('book') == 'beginner_box': continue
      c[mod.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))] += 1
  print(c)"
  ```
  → `done=5837` (unchanged since F0's close; this cycle's own re-run produced the identical figure —
  no code or corpus content changed this cycle). Also removed a stale `NO_GROUNDING_PROBE` exemption
  for `companion`/`spell` (both kinds independently confirmed to reach a nonzero `grounded` count) and
  characterized `feat`'s 367-unit `unknown` residue into three buckets (100 option-pool, 217
  genuinely-unreachable, 50 unclustered-remainder), landed at
  `artifacts/sd30-e0-f3-unknown-residue/`.
- **Epic 1 — code-side identifier cleanup.** Audit pass confirming no `sd30_*`/`SD30_*` leakage in
  shipping code; `epic-1-identifier` `COMPLETE`.
- **Epic 2 — operator pre-launch.** Confirmed `kanban.md`/`epic-breakdown.md` post-split agreement,
  re-derived the 23-book `class_feature` roster (15,472 units) against the guarded work-inventory regen
  with zero verification-stamp loss.
- **Epic 3 — PI-screening provenance gate (`decisions.md §39`, `§52`-`§54`).** Four sub-cards, all
  confirmed on `tranche/10` by content:
  - **F1** — the per-class PI-blacklist sweep (`pi_table_sweep::screen_generated_table`) was already
    built and wired by SD-29; this epic added two permanent regression tests
    (`tests/pi_table_sweep.rs`) proving it against real shipped `class_feature` content.
  - **F2** — wired the shared `pi_screening::{declared_product_identity,
    classify_optional_field_declared}` reader into `src/bin/ingest_pu_classes.rs`'s `class_feature`
    write loop: `NAMEISPI:YES` rows now drop before any other processing, `DESCISPI:YES` descriptions
    redact through the shared reader, and `license`/`pi_field`/`pi_marker` are now genuinely populated
    instead of a hardcoded stand-in.
  - **F3** — corpus-wide declared-PI backfill: `transcribe_monster_tables.py` and
    `transcribe_companion_tables.py` both gained `NAMEISPI`-drop/`DESCISPI`-redact; zero live corpus
    exposure re-confirmed.
  - **F4** — a permanent regression gate
    (`tests/sd30_declared_product_identity_in_shipped_class_features.rs`) proven RED-then-GREEN against
    a planted leak, so a future ingest cannot silently reintroduce a declared-PI leak into shipped
    `class_feature` content.
- **Epic 7 — build version numbering.** Version triple set to `0.10.0` across the three build-config
  files; test anchors updated in both `buildVersionTriple.test.ts` locations. **Open at population
  time** (see "Known issues" below) — the version-bump commits required an unlocked `cargo test` to
  re-resolve `apps/desktop/src-tauri/Cargo.lock`, and the retry gate launched afterward has not yet
  returned an exit code as of this document's population.

## What SD-30 handed to SD-31 and SD-32

- **SD-31-corpus-closure-grind** inherits: Epic 4 (per-class `class_feature` measurement, `decisions.md
  §38`'s `unknown`-bucket characterization for `class_feature`'s 3,622-unit residue), Epic 5 (archetype
  mechanism — supersession + chooser shapes), Epic 6 (per-class chassis-sweep ingest across the 23
  in-scope books), Epic 10 (corpus-wide ingest lanes for `monster`/`spell`/`race`/`race_trait`), Epic 11
  (7-book onboarding under the 100% mandate). It also inherits Epic 3's PI-screening gate as a
  cross-package standing dependency (the gate itself did not move) and the F1/F2/F3/F4 invocation
  contracts this bundle documented for it (`decisions.md §52.3`, `§53.5`,
  `forward-scope-register.md` C1.4/C1.7).
- **SD-32-engine-capability-builds** inherits: Epic 12 (race chassis, closing the ~2,894
  chassis-blocked `race_trait` units plus the `race` kind's 0.0% floor) and Epic 13 (verdict-path
  capability for the ~3,547 unmeasurable units, classifier work bound by this package's own
  accuracy-not-movement rule, `decisions.md §50(a)`/`§50(c)`).

## The honest board position, per kind (re-derived `SD30-E9-F1-001`, unchanged since Epic 0's close)

Command: as above (`doneness_verdict()` replay, grouped by `kind`).

| kind | done | total | % |
|---|---:|---:|---:|
| equipment_modifier | 911 | 1,580 | 57.7% |
| feat | 1,178 | 2,610 | 45.1% |
| equipment | 2,626 | 6,208 | 42.3% |
| companion | 416 | 1,696 | 24.5% |
| class | 27 | 185 | 14.6% |
| monster_ability | 334 | 3,107 | 10.7% |
| race_trait | 266 | 3,447 | 7.7% |
| spell | 47 | 2,843 | 1.7% |
| monster | 7 | 1,270 | 0.6% |
| class_feature | 25 | 15,472 | 0.2% |
| race | 0 | 103 | 0.0% |
| **total** | **5,837** | **38,521** | **15.2%** |

**This is a corpus-wide figure, not an SD-30-scope figure** — SD-30's own narrowed scope (Epics
0/1/2/3/7/8/9) owns none of the per-kind `done` movement directly except Epic 0's rung-application gain
(+2,373, already reflected in the table above, landed before this document's population). The
per-kind remainder is SD-31's and SD-32's to move; the 100% mandate (`decisions.md §45`) is the
**joint** SD-30 → SD-31 → SD-32 exit criterion, not something this package achieved or claims to have
achieved alone.

## Operational changes

- Hermes board retired; work queue is `kanban.md` (local-file Markdown), paired with `progress.md`.
- Concurrency budget re-derived twice this bundle as the box was resized (4 cores/2026-08-11 → 8
  cores/`decisions.md §47` → 24 cores/`SD30-PRELAUNCH-002`) — re-derive `nproc`/`df` at every
  pre-launch, never carry a prior figure forward as a constant.
- The table-sheet doneness doctrine (`decisions.md §49`) ratified the `literal-verified`/
  `fixture-verified` rungs: `done` means the character sheet exposes the end rule with a true resolved
  value, not that the engine merely simulates the mechanism.

## Known issues / open items at population time

1. **`epic-8-code-review` not started.** Blocks this package's own closure (`decisions.md §51`'s
   narrowed exit criterion). No successor package owns it — it is this package's own remaining scope.
2. **`epic-7-version`'s gate exit code not confirmed at HEAD.** `kanban.md` marked the card `COMPLETE`
   at 20:18 on 2026-08-14, before its own gate returned; the version-bump commits required an unlocked
   `cargo test` to fix `Cargo.lock`, and the subsequent retry gate (PID `663386` as observed by this
   cycle, launched ~21:40) was still running (in the `desktop` stage) as of this cycle's own
   observation and has not yet appended a `VERIFY_EXIT=` line to any log. The last **confirmed** green
   full-gate run on this branch is `VERIFY_EXIT=0` at commit `b472aec2` (`epic-3-pi-gate`'s own close),
   which predates the version-bump commits — so no confirmed-green run exists yet at any tip that
   includes `0.10.0`. This is a real open item for whichever cycle claims closure next, not fabricated
   or inferred away by this document.
3. **The 100% dashboard mandate** (`decisions.md §45`) remains unreached corpus-wide (15.2% `done`,
   table above) and is not claimed as reached by this bundle — it is the joint SD-30 → SD-31 → SD-32
   exit criterion.

## Verification evidence

Per-cycle receipts, with the command behind every figure, are in `progress.md`. The last confirmed
`VERIFY_EXIT=0` full-gate run on `tranche/10` prior to this document's population is recorded at
`SD30-E3-F4-001`'s receipt (commit `b472aec2`, 16/16 stages PASS, `reach` 27 matched tests).

## Update eligibility

No operator-on-file override recorded for this closure pass. The operator personally merges the
tranche-promotion PR (`tranche/10 → develop`) once opened; no cycle in this bundle is authorized to
merge it.
