---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01; re-scoped 2026-08-10)
date: 2026-08-10
canonical_branch: tranche/10 (operator directive 2026-08-01)
build_version_target: 0.10.<build>
---

# SD-30 — Local-file Work Queue (replaces Hermes board `codex-tranche-10`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-30's
work queue is a local-file Markdown table. The supervisor reads this
file at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

**Re-cut 2026-08-10** (`decisions.md §33-38`). The sixteen per-book cards this
file previously carried are retired — that book list dissolved into SD-29's
corpus-wide scope. Cards now match `epic-breakdown.md`'s 9 dependency-ordered
epics: measurement (epic-4) gates mechanism (epic-5) and chassis-sweep
(epic-6) **per class**, not bundle-wide.

## Status legend

- `READY` — not yet claimed. Cycle can pick up.
- `READY (gated on ...)` — not claimable until every named card is `COMPLETE`. The gate is part of the card's state: a cycle that claims a gated card while its gate is open is out of protocol.
- `READY (per-class, gated on epic-4 for the target class)` — epic-5/epic-6 specific: the card as a
  whole opens once its predecessor epic is under way, but any individual class-scoped cycle inside it
  still needs that class's own epic-4 (and, for epic-6, epic-5) clearance before it can be claimed.
- `IN-FLIGHT` — claimed by a cycle, in progress.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

## Cards (one row per epic; epic-5/epic-6 dispatch further per-class inside their own card)

Rows are in claim-priority order, top-down, matching `loop-instruction.md`'s
"Epic ordering": Epic 1 first, then Epic 2, then Epic 3, then Epic 4 (which
never fully "completes" in the sense of blocking dispatch — it clears classes
incrementally and epic-5/epic-6 cycles begin per class as soon as their class
is cleared).

**Widened 2026-08-13 (`decisions.md §43`):** `epic-0-instrument-apply` is inserted at the top of the
claim-priority order — cheapest lever in the bundle (data already in the engine, instruments already
built by the former SD-32, no new ingest required). It runs independently of the `epic-1`..`epic-9`
`class_feature` chain below (different files: dashboard-producer/instrument code and cross-kind
application vs. `class_feature` rules-tables content) and does not gate or get gated by it.

**Reordered 2026-08-14 (`decisions.md §46`, operator directive, launch session — "dashboard/reporting
is Job 1"):** `epic-0-instrument-apply` is Order 1 in the table below, ahead of every other card, not
only for its cheapest-lever rationale (`decisions.md §43`) but because it is also the reporting
surface the operator and orchestrator both read for live progress. No card outranks it.

| ID | Status | Epic | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-0-instrument-apply` | COMPLETE | **Order 1 — Apply Existing Instruments to `held` (dashboard/reporting, Job 1)** | `done`-rung build (static/derived) + computed-bucket consumer-delta probes, corpus-wide + `unknown`-residue characterization (`feat`) + re-derivation reporting | sd30-e0-f4-report | 2026-08-14T17:40:31-04:00 | `SD30-E0-F4-001` |
| `epic-1-identifier` | COMPLETE | Code-Side Identifier Cleanup | identifier-discipline audit pass | sd30-e1-identifier | 2026-08-14T21:49:20Z | `SD30-E1-F1-001` |
| `epic-2-prelaunch` | COMPLETE | Operator Pre-Launch | local-file dispatch readiness + cycle-0 trap-report + work-inventory (23-book `class_feature` re-derivation) | sd30-e2-prelaunch | 2026-08-14T22:10:00Z | `SD30-E2-F1-001` |
| `epic-3-pi-gate` | IN-FLIGHT (SD30-E3-F1/F2 sub-scopes COMPLETE; F3/F4 still open) | PI-Screening Provenance Gate | per-class PI-blacklist sweep (SD30-E3-F1, COMPLETE) + declared-PI reader wired into class_feature ingest (SD30-E3-F2, COMPLETE, `decisions.md §39`/`§53`) + corpus-wide declared-PI backfill (SD30-E3-F3) + regression gate (SD30-E3-F4) — F3/F4 remain open and still hard-block SD-31's chassis-sweep successor card, no chassis-sweep cycle may claim a class before both are COMPLETE | sd30-e3-f2-declared | 2026-08-14T22:52:00Z | `SD30-E3-F2-001` |
| `epic-4-measurement` | **MOVED to `SD-31-corpus-closure-grind/kanban.md` `epic-1-measurement` (`decisions.md §51`, 2026-08-14)** | Per-Class Archetype Measurement | class inventory + per-class hand-verification + chooser-primitive design + `unknown`-bucket characterization | — | — | — |
| `epic-5-mechanism` | **MOVED to `SD-31-corpus-closure-grind/kanban.md` `epic-2-mechanism` (`decisions.md §51`)** | Archetype Mechanism | supersession-shape wiring per cleared class; chooser-shape wiring once epic-4-F3 lands | — | — | — |
| `epic-6-chassis-sweep` | **MOVED to `SD-31-corpus-closure-grind/kanban.md` `epic-3-chassis-sweep` (`decisions.md §51`)** | Per-Class Chassis Sweep | per-class `class_feature` ingest across the 23 in-scope books, reach-gate claim per record | — | — | — |
| `epic-7-version` | READY (gated on epic-1) | Build Version Numbering | first concrete value `0.10.<build>` | — | — | — |
| `epic-8-code-review` | READY (gated on epic-1, epic-2, epic-3, epic-7 — narrowed 2026-08-14, `decisions.md §51`; no longer gated on epic-5/epic-6, moved) | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §26`) | — | — | — |
| `epic-9-closure` | READY (gated on every other card) | Closure Epilogue | tranche promotion PR | — | — | — |
| `epic-10-ingest-lanes` | **MOVED to `SD-31-corpus-closure-grind/kanban.md` `epic-4-ingest-lanes` (`decisions.md §51`)** | Corpus-Wide Ingest Lanes, folded from SD-29 | per-kind ingest: SD30-E10-F1 `monster`, F2 `spell`, F3 `race`, F4 `race_trait` — each runs the raw-vs-workable split + pre-cycle classifier screen before claiming a book (SD-29 lessons, `decisions.md §44`) | — | — | — |
| `epic-11-book-onboarding` | **MOVED to `SD-31-corpus-closure-grind/kanban.md` `epic-5-book-onboarding` (`decisions.md §51`)** | Book Onboarding, 100% mandate | onboard the 7 `future_state` books (`occult_adventures`, `adventurers_guide`, `mythic_adventures`, `inner_sea_magic`, `inner_sea_temples`, `inner_sea_taverns`, `inner_sea_faiths`) — PI screen (epic-3) clean per book before any record is written (`decisions.md §45`) | — | — | — |
| `epic-12-race-chassis` | **MOVED to `SD-32-engine-capability-builds/kanban.md` `epic-1-race-chassis` (`decisions.md §51`)** | Race Chassis, 100% mandate | build the missing race chassis closing the ~2,894 chassis-blocked `race_trait` units plus the `race` kind — DoD-8 on-screen verification mandatory (`decisions.md §45`) | — | — | — |
| `epic-13-verdict-paths` | **MOVED to `SD-32-engine-capability-builds/kanban.md` `epic-2-verdict-paths` (`decisions.md §51`)** | Verdict-Path Capability, 100% mandate | real (non-placeholder) verdict paths for the ~3,547 unmeasurable units incl. 2,109 `ambiguous`; classifier work bound by `SD-30-class-feature-archetype-bundle/decisions.md §50(c)`'s accuracy-not-movement rule (`decisions.md §45`) | — | — | — |
| `epic-14-cloud-fanout` | **SPLIT (`decisions.md §51`): grind-lane scope MOVED to `SD-31-corpus-closure-grind/kanban.md` `epic-6-cloud-fanout`; capability-build-lane scope MOVED to `SD-32-engine-capability-builds/kanban.md` `epic-3-cloud-fanout`. No `epic-14` remains in this file.** | Cloud Fan-Out Protocol | local-proof-then-cloud-scale protocol for lane B/C shapes (epic-10, epic-11); local orchestrator owns all `tranche/10` merges, DoD-8/dashboard-producer work stays local (`decisions.md §47`) | — | — | — |

## Retired cards (sixteen-book era, 2026-08-01 to 2026-08-10) — historical record, not claimable

`epic-3-oa` through `epic-18-bd2` (Occult Adventures, Horror Adventures, Mythic Adventures, Monster
Codex, Book of the Damned ×2, the ten Inner Sea modules), plus the old `epic-20-version` /
`epic-21-code-review` / `epic-19-closure` numbering, are retired by `decisions.md §35`. None of these
IDs are claimable; a cycle that finds one of them cited in prior doctrine resolves it to the current
card covering the same functional role (Build Version Numbering -> `epic-7-version`; Bundle Code
Review -> `epic-8-code-review`; Closure -> `epic-9-closure`; every per-book content card -> retired
outright, no successor card, since the underlying kinds moved to SD-29 and `class_feature` is now
tracked per-class inside `epic-6-chassis-sweep`, not per-book).

## Cycle claims (cycle-supervisor protocol)

When a cycle claims a card:

1. Edit the card's `Status` to `IN-FLIGHT`.
2. Edit `Claimed-by` to the cycle's harness identifier.
3. Edit `Claimed-at` to the cycle's ISO-8601 timestamp.
4. Edit `Cycle-id` to the cycle's audit ID (e.g., `SD30-E4-F2-<class>-001`).
5. Append the cycle's per-cycle facts to `progress.md`.
6. On cycle completion, edit `Status` to `COMPLETE` and append the
   completion receipt to `progress.md`.
7. For `epic-5-mechanism` and `epic-6-chassis-sweep`: a cycle claiming a
   per-class slice inside the card names the class explicitly in `Cycle-id`
   and confirms (cites the receipt) that class's `epic-4-measurement`
   clearance before claiming — the card-level `IN-FLIGHT`/`COMPLETE` status
   tracks the epic as a whole; individual class slices are tracked in
   `progress.md`.

## Ordering check (2026-08-13, `decisions.md §44`) — Epic 10 fold

`epic-10-ingest-lanes` and its four F-cards are hard-gated on `epic-3-pi-gate` exactly as
`epic-6-chassis-sweep` is — the fold widens which kinds' ingest is subject to the PI-screening gate,
it does not create a bypass. `epic-10` is otherwise independent of the `class_feature` E4/E5/E6 chain
(different kinds; file-disjoint in the common case) and does not gate or get gated by it, mirroring
`epic-0-instrument-apply`'s standing independence.

## Ordering check (2026-08-13, `decisions.md §41-§42`)

Re-verified: `epic-3-pi-gate` (PI-screening, including the 2026-08-13 declared-PI cards
SD30-E3-F2/F3/F4) still hard-blocks `epic-6-chassis-sweep` in the table above, and `epic-4-measurement`
still gates both `epic-5-mechanism` and `epic-6-chassis-sweep` per class. SD-32's corpus-wide
`static`/`derived` gates (`decisions.md §41`) do not change this order — they are consumed by running
`./scripts/verify.sh` per `AT-30-002`, already a standing per-cycle requirement, not a new card. No
reordering needed.

## Update (2026-08-14, `decisions.md §49`) — SD-32's E5/E6 unblocked, now part of `epic-0-instrument-apply`

SD-32 `decisions.md §2` (the `static`/`derived` "no `done` rung" measurement gate, formerly `BLOCKED
(decision)` on the SD-32 board's `e5-static-sweep`/`e6-derived-check` cards) is **ANSWERED** by
operator ruling, 2026-08-14 (the table-sheet doneness doctrine — `decisions.md §49`). The
`literal-verified`/`fixture-verified` rung and its verdict-table mapping are ratified. Those two
cards' work — the static corpus-literal byte-equality sweep and the derived evaluator-vs-fixture
check — is corpus-wide instrument-application work with no `class_feature`-only scope, so it is
claimed under **`epic-0-instrument-apply`** above (already `READY`, already Order 1) rather than as
separate cards on this board; SD-32's own kanban records their `READY` status directly for cycles
that read that package's queue.

## Update (2026-08-14, SD30-E0-F2-001) — computed-bucket consumer-delta probes: F2 sub-scope COMPLETE

Enumerated the real kind/`probe_*` list fresh rather than working from the card's own framing, and
found it wrong: `class_feature` (4,178 units, confirmed the largest `computed` population) already
has a landed, wired probe (`probe_class_feature_effect_wiring`, `src/bin/v06_work_inventory.rs:4072`)
grounding 20 units — the card's "no existing `probe_*` function" claim for it does not hold.
The 4 kinds genuinely lacking one (`companion`, `monster_ability`, `monster`, `race`) were each
investigated on their own evidence, not assumed to need a probe built: `companion`/`monster_ability`'s
only real consumer (`apps/desktop/src-tauri`'s `list_companion_catalog`/`list_monster_catalog`,
already claimed in `reach_gate.rs`) is a proven structural bijection over the exact compiled registry
the current membership check already reads (own module tests assert it), so a new probe would be
redundant, not a gap — confirmed by the fact that every `computed` unit of both kinds is already
exactly `{grounded, not-ingested}`, no third status a probe could move. `monster` is already 100%
grounded (7/7). `race`'s 4-unit `computed` population is 2 `file_kind()`-misclassified companion
records plus 2 real not-yet-ingested races (Aasimar/Tiefling) — a probe changes none of their status.
`NO_GROUNDING_PROBE = ("companion", "spell")` in `scripts/observer/pf1e_dashboard_producer.py` was
removed (both kinds re-checked and confirmed reaching a nonzero `grounded` count under `computed`
right now — companion 416/793, spell 46/210 — the card's own confirmation bar). Board effect:
`held` −132 / `in-progress` +132 (spell reclassified, more honestly), `done` unchanged at 5,837 — not
a bar lowered, a bar the excuse for exempting no longer covers. A stale `wiring-class-summary.json`
cache (hazard 5) served the OLD split after the code change until `WIRING_SUMMARY_SCHEMA` was bumped
11→12 to force recompute — caught live, fixed in the same change. Full receipt, all commands, and the
DoD table: `progress.md`, cycle `SD30-E0-F2-001`. This row stays `READY` — F3 (`unknown`-residue
characterization) and F4 (re-derivation reporting) remain open under it.

**Correction (SD30-E0-F1-001, 2026-08-14).** This section's "4,805 ceiling, 1,602 movable" (static)
and "2,674 ceiling, 535 movable" (derived) figures were pre-run planning estimates, never
re-derived against an actual sweep/check run, and both are wrong. The mechanism landed in commits
`4087f171`/`e928da8c` (static rung) and `c04eb9ef` (derived rung), already merged to `tranche/10`
before this cycle. Re-run live this cycle:
`cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep.json` → CLEAN, 3516 records
examined of 9328 read, 0 findings, joining to **2,322** `literal-verified` units (not 1,602) — the
static+held population is 4,801 (not 4,805), of which 2,322 are TOKEN-COMPARED (the actual movable
set), 138 are digest-only (not credited), 2,341 are unreached (no shipped corpus record at that
book/file/line, can never move by re-running the sweep).
`cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture.json` → **49**
of 94 covered units cleared (not 535) — the fixture covers exactly 94 of 2,879 held `derived` units
by design (every held unit whose corpus row carries an evaluated token family), not a sample that
widens; 1 of the 94 is a known, live-confirmed failure
(`advanced_players_guide:equipment:spindle_of_perfect_knowledge` — corpus states
`BONUS:STAT|INT,WIS,CHA|4|TYPE=Enhancement`, the evaluator currently produces no ability bonus — the
instrument correctly refuses to stamp it), and the other 2,785 held derived units are out of the
fixture's coverage entirely, not movable by construction. Board movement from applying both rungs,
re-derived by importing the dashboard producer's own `doneness_verdict()` and replaying it over
`git show <ref>:docs/work-inventory.json` at the pre-rung commit (`d1b29589`, the parent of
`4087f171`) versus current `HEAD`: `done` 3,464 → 5,837 (+2,373), `held` 9,455 → 7,086 (−2,369),
matching `state-goals-and-lessons.md §1.1`'s figure exactly. `retro.py correction` event id
`1786738386633-sd30-e0-f1-rung-012e3f` (`docs/retro/events/sd30-e0-f1-rung.jsonl`).

## Update (2026-08-14, SD30-E0-F3-001) — `unknown`-residue characterization: F3 sub-scope COMPLETE

Re-derived per-kind `unknown` residue fresh against the committed `docs/work-inventory.json`
(unchanged this cycle): only `class_feature` (3,622, already owned by `decisions.md #38` /
`SD-31-corpus-closure-grind epic-1-measurement`) and `feat` (367, previously uncharacterized) are
nonzero — every other kind is 0. Characterized `feat`'s residue into `decisions.md #38`'s three
buckets (option-pool / genuinely-unreachable / unclustered-remainder), read per-unit off the PCGen
`.lst` source line: **100 option-pool, 217 genuinely-unreachable (needs a probe-fixture capability
expansion, not ingest), 50 unclustered-remainder.** Durable artifact landed at
`artifacts/sd30-e0-f3-unknown-residue/` (README with an explicit SD-31 invocation contract,
per-unit JSON, reproducible classifier script). Two of this bundle's own inherited figures corrected
in place: F3's own acceptance text's "329 units" claim for feat -> re-derived **367**; `decisions.md
#38`'s "3,218" claim for class_feature -> re-derived **3,622** (not re-characterized, out of F3
scope, drift flagged for SD-31). Full receipt: `progress.md`, cycle `SD30-E0-F3-001`. This row stays
`READY` — F4 (re-derivation and reporting) remains open under it.

## Update (2026-08-14, SD30-E0-F4-001) — re-derivation and reporting: `epic-0-instrument-apply` COMPLETE

F4's own acceptance text's `artifacts/derive-movable-mass.py` re-run instrument confirmed, live, to
still raise `ValueError('static','literal-verified')` exactly as F1's prior correction found — cannot
produce a pre/post pair; used the live dashboard producer's own `doneness_verdict()` (imported, not
transcribed) instead, replayed over `git show <ref>:docs/work-inventory.json` at the true pre-Epic-0
commit (`98d98d3a`) versus current `HEAD`. Board `done` unchanged at **5,837** across Epic 0's own
three cycles (F1/F2/F3 confirmed/characterized an already-landed mechanism rather than moving new
`done` units themselves); non-`done` buckets moved `held` −38 / `unmeasurable` +443 / `not-started`
−405, entirely attributable to F3's guarded regen correctly reclassifying 38 `feat` units. All three
reporting surfaces (`docs/work-inventory.json`, the live `PF1e-dashboard.json`, this package's own
`state-goals-and-lessons.md §1.1`) cross-checked and now agree exactly — the third surface was stale
on two independent counts (a `beginner_box`-exclusion gap and a pre-F3 snapshot date), both corrected
in place with a retro correction event, not silently folded in. `AT-30-015`'s already-executed move to
`SD-31-.../acceptance-and-verification.md AT-31-005` (`decisions.md §51`) is confirmed intact by
content; that successor table's actual current per-kind figures were delivered into it per this
bundle's SCOPE NOTE, surfacing and fixing a real row-copy-paste defect in its `spell` row (`held`
1,235→**1,103**) along the way — not merely re-stating the split-time snapshot as current. Committed
inventory verification stamps confirmed intact (2,322 `literal-verified` + 49 `fixture-verified` =
2,371, matching F1 exactly). `reach` stage re-run: PASS, 27 matched tests. `v06_corpus_trap_report
--audit`: exit 2, 177 pre-existing defects byte-identical to F1/F2's own prior reproduction (33/3/141
by kind), confirmed neither caused nor worsened this cycle. Full receipt, all commands, and the DoD
table: `progress.md`, cycle `SD30-E0-F4-001`.

**`epic-0-instrument-apply` flipped to `COMPLETE`** — F1/F2/F3/F4 all confirmed on `tranche/10` **by
content** (grepped the landed symbols fresh this cycle, not read from card status):
`literal-verified`/`fixture-verified` in `pf1e_dashboard_producer.py` (F1),
`probe_class_feature_effect_wiring` in `v06_work_inventory.rs` + `NO_GROUNDING_PROBE = ()` (F2), the
`artifacts/sd30-e0-f3-unknown-residue/` artifact directory (F3), and this update plus the re-derived
figures above (F4) — all present at `HEAD` (`3a3b89d1` before this cycle's own doc commit), confirmed
an ancestor of `origin/tranche/10`.

## Update (2026-08-14, `SD30-E2-F1-001`) — `epic-2-prelaunch` COMPLETE

Re-verified every SD30-E2-F1/F2 acceptance criterion fresh rather than trusting the P0.5
(2026-08-14, pre-epic-1) and `SD30-PRELAUNCH-002` (2026-08-14, HEAD `e39a7f47`) receipts still held
after `epic-1-identifier` closed (HEAD now `aa248507`):

- **F1 — kanban.md vs `epic-breakdown.md` agreement, post-split:** confirmed by direct read of both
  files, not by re-trusting either's own claim. `kanban.md`'s per-epic rows and `epic-breakdown.md`
  §"Scope narrowed 2026-08-14" both independently state the identical live/moved/split partition:
  live `0,1,2,3,7,8,9`; MOVED `4,5,6,10,11` -> `SD-31-corpus-closure-grind`; MOVED `12,13` ->
  `SD-32-engine-capability-builds`; `14` SPLIT across both. **No disagreement found — neither file
  needed a fix.** (The SD30-E2-F1 acceptance text's own "9 re-cut epics" phrasing in
  `epic-breakdown.md` line 135 predates Epic 0/10-14 and the split; it is stale wording on a
  sub-bullet, not a disagreement between the two files' actual card lists, so left as historical
  text per this package's standing convention rather than edited — the same convention `decisions.md
  §51` itself uses.) Working tree: `git status --porcelain` shows only pre-existing, out-of-scope
  changes (`.gitignore`, `.github/workflows/deploy-site.yml`) belonging to a live, unrelated
  `site-deploy`/`fix/site-deploy-page-workflow` branch effort on this shared checkout (confirmed via
  `./scripts/reclaim.sh`'s branch-skip list) — not SD-30 debris, not touched by this cycle.
  Re-scope receipt in `progress.md`: present (`## 2026-08-14 — Split: Phase 3 to SD-31, Phase 4 to
  SD-32`).
- **F2 — branch/trap-report/work-inventory:** `tranche/10` local HEAD `aa248507` == `origin/tranche/10`
  (`git rev-parse`, both sides). Guarded work-inventory regen run (DoD item 4 procedure, not a bare
  run): `corpus_literal_sweep` exit 0 (CLEAN, 3516/9328 records, 0 findings), 
  `derived_evaluator_fixture_check` exit 0 (49/94 cleared, 1 known pre-existing fail —
  `spindle_of_perfect_knowledge`, same as F1/F4's own prior finding), `v06_work_inventory` guarded
  run exit 0. `git diff --stat docs/work-inventory.json` — 1 line changed (`generated_at` only),
  **zero stamp loss confirmed** (`2322 literal-verified + 49 fixture-verified = 2371`, byte-match to
  F1/F4). Book roster **re-derived fresh** from the regenerated file
  (`python3 -c "... kind=='class_feature' ..."`): **15,472 units, 23 books**, per-book counts
  identical to `decisions.md §33`'s table down to the unit — **no discrepancy, no correction
  needed.** Since Epic 6 (the only card that ever pins a book) is `MOVED` to SD-31, **this cycle
  targets no book** — stating that rather than running 23 no-op trap-reports; the re-derived roster
  above is the artifact SD-31 consumes. `v06_corpus_trap_report -- --audit`: exit 2, 177 pre-existing
  `wiring-class-mismatch` defects (33 companion / 3 monster / 141 monster_ability), byte-identical in
  count and kind-split to F1/F2/F4's own prior reproduction — confirmed pre-existing, not caused or
  worsened by this cycle.

Full receipt, all commands verbatim, DoD table: `progress.md`, cycle `SD30-E2-F1-001`.

**`epic-2-prelaunch` flipped to `COMPLETE`.**

## Update (2026-08-14, `SD30-E3-F1-001`) — SD30-E3-F1 (per-class PI-blacklist sweep) sub-scope COMPLETE

`decisions.md §39.4` had already narrowed `SD30-E3-F1`'s own acceptance to "the blacklist sweep"
(the declared-`NAMEISPI`/`DESCISPI` reader is `SD30-E3-F2`'s separate card). This cycle found the
mechanism the acceptance names (`pi_table_sweep::screen_generated_table`, shared
`pi_screening::PI_BLACKLIST_TERMS`) already built by SD-29 (`579d5941`) and already production-wired
— two live non-test callers, `gen_feat_gap_tables.rs`/`gen_equipment_gap_tables.rs` — and already
covering `class_feature`-shaped content (`docs/governance/pi-sweep-baseline.tsv`'s two `real-leak`
rows inside `acg/archetype_tables.rs` and `advanced_race_guide/archetype_tables.rs`). Epic 6 (this
card's own consumer) moved to `SD-31-corpus-closure-grind` Epic 3 before this cycle fired, so no
SD-30 cycle calls it against a live book. Delivered instead: two new permanent regression tests
(`tests/pi_table_sweep.rs`) proving the pre-commit entry point refuses/passes on real, already-shipped
`class_feature` content (not a fixture), plus the exact six-step invocation contract SD-31's Epic 3
must follow (`decisions.md §52.3`), pointed at from both packages' `forward-scope-register.md` (SD-30
C1.4, SD-31 G1.4).

Full receipt, all commands verbatim, DoD table: `progress.md`, cycle `SD30-E3-F1-001`.

**`SD30-E3-F1` sub-scope COMPLETE. `epic-3-pi-gate` stays `IN-FLIGHT` — F2/F3/F4 remain open and
still hard-block `epic-6-chassis-sweep`'s successor card in `SD-31-corpus-closure-grind`.**

## Update (2026-08-14, `SD30-E3-F2-001`) — SD30-E3-F2 (declared-PI reader) sub-scope COMPLETE

`decisions.md §39.2` had stated no `class_feature` ingest binary exists yet — this cycle found and
corrected that premise (`decisions.md §53.1`, `retro.py correction`
`1786747577757-sd30-e3-f2-declared-541af1`): `src/bin/ingest_pu_classes.rs` (SD-27) already is one,
already production-wired, already shipping `data/corpus/pathfinder_unchained/class_feature/`. This
cycle wired `pi_screening::{declared_product_identity, classify_optional_field_declared}` — the same
shared reader `ingest_race_traits.rs` already uses, no forked implementation — into that binary's
`class_feature`-writing loop: `NAMEISPI:YES` rows now drop before any other per-row processing, named
`{lst}:{line}: {key}` in a new `dropped, NAMEISPI:YES` receipt line; `DESCISPI:YES` descriptions now
redact through the shared reader (a new `descriptions redacted by DESCISPI:YES` receipt line) and the
record's `license`/`pi_field`/`pi_marker` are now genuinely populated from that call instead of a
hardcoded `Some(License::Ogl), None, None` for every record (a second, independent defect this same
change closes). The one source file this binary reads carries zero live `NAMEISPI`/`DESCISPI` tokens
today (`decisions.md §53.2`), so two new unit tests
(`declared_product_identity_of_reads_nameispi_and_descispi_off_the_row`,
`a_descispi_row_is_redacted_through_the_shared_reader_even_with_no_blacklist_term`) replay the real
production functions against real-shaped synthetic rows instead — the same "prove it fails" shape
SD30-E3-F1 used for the same reason. The invocation contract for SD-31's Epic 3 successor (or any
future `class_feature` writer for the 6 books `§39.2` found real exposure in) is `decisions.md §53.5`.

Full receipt, all commands verbatim, DoD table: `progress.md`, cycle `SD30-E3-F2-001`.

**`SD30-E3-F2` sub-scope COMPLETE. `epic-3-pi-gate` stays `IN-FLIGHT` — F3/F4 remain open and still
hard-block `epic-6-chassis-sweep`'s successor card in `SD-31-corpus-closure-grind`.**

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle
dispatch honors the post-edit state.

## Resolution to operator directives

This file is the load-bearing replacement for the Hermes `codex-tranche-10`
board (operator-confirmed 2026-08-01). When a Hermes board card is
referenced from prior doctrine (`decisions.md`, `scope-draft.md`,
`loop-instruction.md`, etc.), the reference resolves to a `kanban.md`
card id at the time of cycle dispatch — for cards retired 2026-08-10, resolve
per the "Retired cards" table above.
