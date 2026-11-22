# SD-30 Release Notes

**Populated `SD30-E9-F1-001`, 2026-08-14; updated CLOSED `SD30-E9-F2-001`, 2026-08-15.** This
bundle's scope was narrowed mid-run (`decisions.md §51`, 2026-08-14): Epics 4/5/6/10/11 (per-class
measurement, archetype mechanism, chassis-sweep ingest, corpus-wide ingest lanes, book onboarding)
moved to `SD-31-corpus-closure-grind/`; Epics 12/13 (race chassis, verdict-path capability) moved to
`SD-32-engine-capability-builds/`. What follows describes what **this** package, narrowed to Epics 0,
1, 2, 3, 7, 8, 9, actually delivered — not the wider sixteen-book/all-kinds charter this package
carried at points in its own history. The per-cycle receipts in `progress.md` are the per-record
evidence; this document summarizes the release.

**Status: CLOSED, `SD30-E9-F2-001`, 2026-08-15.** Both blockers the first closure attempt
(`SD30-E9-F1-001`) correctly refused to paper over are resolved and re-confirmed by content this
cycle: `epic-7-version`'s own gate is green (`SD30-E7-F1-001` re-dispatch, `VERIFY_EXIT=0` at
`4630fec2`, the first confirmed-green full gate at any `0.10.0` tip), and `epic-8-code-review` ran
to completion (`SD30-E8-F3-001`, three real findings fixed in bundle, `VERIFY_EXIT=0` at `fc461781a`).
Epics 0/1/2/3/7/8 are all `complete` in `progress.md` **and** re-confirmed on `tranche/10` by content
this cycle (symbol grep against the actual `HEAD`, not a card status word — see `progress.md`
`SD30-E9-F2-001` §2). The tranche-promotion PR `tranche/10 → develop` is opened by this cycle; the
operator merges it personally.

## Summary

- **Bundle:** SD-30 — Class-Feature/Archetype Bundle (narrowed 2026-08-14 to the gate/process epics).
- **Branch:** `tranche/10`.
- **Version:** `0.10.0` committed at HEAD (`apps/desktop/package.json`, `tauri.conf.json`,
  `src-tauri/Cargo.toml`, re-derived this cycle: `grep -h '"version"'
  apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json` → `"0.10.0"` in both; `grep
  'version = ' apps/desktop/src-tauri/Cargo.toml | head -1` → `"0.10.0"`). The build counter
  (`GITHUB_RUN_NUMBER`) substitutes for the final digit at publish time; the repo carries the base
  triple only.
- **HEAD at population (`SD30-E9-F1-001`):** `33ef64fe`. **HEAD at closure (`SD30-E9-F2-001`,
  before this cycle's own doc-only commits):** `44497b67e` (`git rev-parse HEAD`).

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
  files; test anchors updated in both `buildVersionTriple.test.ts` locations. **Closed for real,
  `SD30-E7-F1-001` re-dispatch, 2026-08-14/15.** The first version-bump commit missed a build-label
  string fixture (`src/testerWorkbench/loadTesterWorkbenchSurface.test.ts`, asserted fresh by
  `src/releaseChecks/buildLabelFixtureFreshness.test.ts`) and a CI publish-workflow version stamp; both
  fixed, and the re-dispatch cycle polled a full `verify.sh` inline to a captured exit code
  (`VERIFY_EXIT=0`, 16/16 stages, at `4630fec2`) — the first confirmed-green full gate at any tip
  carrying `0.10.0`.
- **Epic 8 — bundle code review.** `SD30-E8-F3-001`, 2026-08-15. Reviewed the full bundle diff against
  `origin/develop` (3,146 files, +23,214/-7,389). Three real defects found and fixed in bundle: (1) a
  dangling PI-dropped grant reference in `ingest_pu_classes.rs` (a future `NAMEISPI:YES` feature row
  would have shipped an orphan grant in class-variant JSON — 0-record no-op on today's corpus, fixed
  before it could bite); (2) the DoD-3 trap-audit self-check's own bare-basename citation bug, which
  masked (3) the identical bug already live in `gen_book_cache.rs`'s generator, which had already
  shipped 3 wrong `wiring_class` stamps into production `inner_sea_gods` monster data — fixed and the
  3 records regenerated (verified byte-for-byte: only the 3 expected `wiring_class` field-pairs
  changed). `VERIFY_EXIT=0`, 16/16 stages, at `fc461781a`. One finding (wiring the trap-audit into
  `scripts/verify.sh` itself) deferred to `SD-31-corpus-closure-grind` (`forward-scope-register.md`
  C1.8) as a repo-wide gate-shape decision outside this card's remit.

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

## Known issues / open items at closure

Both open items named at population time (`epic-8-code-review` not started; `epic-7-version`'s gate
not confirmed) are **resolved** — see the Epic 7/Epic 8 bullets above. Remaining open items at actual
closure:

1. **The `v06_corpus_trap_report -- --audit` gate is not wired into `scripts/verify.sh`.**
   (`epic-8-code-review` finding 2c, `forward-scope-register.md` C1.8.) The audit is a real,
   non-vacuous gate as of this bundle (`epic-8-code-review` fixed its own bare-basename bug and the
   generator bug it was masking), and re-derived independently by this closing cycle:
   `cargo run --locked --bin v06_corpus_trap_report -- --audit` → `TRAP_AUDIT_EXIT=0` (`259 0
   mod-record`, no defects). It is **not currently a `scripts/verify.sh` stage**; wiring it in is a
   repo-wide gate-shape decision (which books/kinds it covers corpus-wide, how a legitimate
   `no_corpus_line` record is told apart from a future real regression) beyond any one card's remit,
   deferred to `SD-31-corpus-closure-grind`.
2. **`v06_work_inventory.rs`'s `enumerate_file` shares the identical bare-basename citation bug**
   that `epic-8-code-review` found and fixed in two other call sites (`corpus_traps.rs`'s
   self-check, `gen_book_cache.rs`'s generator) — self-found during that cycle's investigation, not
   in the reviewer's original finding set, and **not itself fixed** (unconfirmed board effect;
   `forward-scope-register.md` C1.9, owner the next measurement-touching bundle, likely
   `SD-31-corpus-closure-grind`).
3. **The 100% dashboard mandate** (`decisions.md §45`) remains unreached corpus-wide (15.2% `done`,
   table above) and is not claimed as reached by this bundle — it is the joint SD-30 → SD-31 → SD-32
   exit criterion. SD-30's own contribution is Epic 0's rung-application gain (`done` 3,464 → 5,837);
   the remainder is SD-31's (per-kind ingest, per-class chassis sweep) and SD-32's (race chassis,
   verdict-path capability) to move.

## Verification evidence

Per-cycle receipts, with the command behind every figure, are in `progress.md`. The full gate is
green at both epics that carried code changes after population: `epic-7-version`
(`VERIFY_EXIT=0`, 16/16 stages, `4630fec2`) and `epic-8-code-review` (`VERIFY_EXIT=0`, 16/16 stages,
`fc461781a`). This closing cycle (`SD30-E9-F2-001`) launched its own full `verify.sh` at the closing
tip; see `progress.md`'s `SD30-E9-F2-001` receipt for the captured exit code and log path (launched
in background per this package's own gate-sequencing doctrine — doc-only cycles may run `--only`
stages, but this cycle chose the full sweep for the strongest possible closing-tip evidence).

## Update eligibility

No operator-on-file override recorded for this closure pass. The operator personally merges the
tranche-promotion PR (`tranche/10 → develop`) once opened; no cycle in this bundle is authorized to
merge it.
