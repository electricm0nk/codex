# SD-30 — Per-cycle Receipts

This file carries the per-cycle receipt for SD-30. Each cycle appends a
new section with the cycle-id and the operator-readable per-cycle facts.

The supervisor reads this file to verify completion before the next
cycle claim (per `decisions.md §14a` local-file dispatch + `loop-instruction.md`
§"Step 6").

## Cycle 0.0 — Chassis Land (planning-ready)

**Date:** 2026-08-01
**Cycle ID:** `SD30-LAND-1`
**Operator:** Todd Hintzmann (directive 2026-08-01)
**Surface:** this directory (`programs/codex/requirements/SD-30-occult-and-companion-content-ingestion/`)

### What landed

- 13+ file canonical chassis per the spec-domain-bundle-authoring skill (matching SD-22 through SD-29's published shape).
- Per-doctrine amendments per operator directive 2026-08-01:
  - **Decision §13** — `tranche/10` branch, no Hermes board (parallel to SD-28's `tranche/8` and SD-29's `tranche/9`).
  - **Decision §14a** — Hermes board retired, local-file dispatch.
  - **Decision §15** — `0.10.<build>` build version.
  - **Decision §16** — cross-book conflict rule with "recently published takes precident" precedence; the class-grant overlap (Occultist family) is the only exception.
  - **Decision §17** — bulk modifications deferred.
  - **Decision §18** — reach gate is the prime rule (the definition of done; engines only when strictly necessary; rules-as-data with pre-computed values; supersedes §12).
  - **Decision §19** — operator ack-chain recorded.

### Book list (sixteen in scope; four deferred)

Confirmed 2026-08-01:

- **In scope (16)**: Occult Adventures, Horror Adventures, Mythic Adventures, Monster Codex, Book of the Damned ×2 volumes, Inner Sea World Guide, Inner Sea Combat, Inner Sea Faiths, Inner Sea Gods, Inner Sea Magic, Inner Sea Races, Inner Sea Temples, Inner Sea Taverns, Inner Sea Bestiary, Inner Sea Intrigue.
- **Deferred (4)**: NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes Handbook — recorded in `forward-scope-register.md C2.x` as future-acquisition candidates.

### Pre-launch state

| Check | Status |
|-------|--------|
| `kanban.md` exists | PENDING (lands this cycle) |
| Branch `tranche/10` pushed to origin | PENDING (operator action at cycle launch) |
| OAuth credentials valid | PENDING (operator action at cycle launch) |
| Working tree clean | ASSUMED (pre-launch verification) |
| Cycle-0 trap-report + work-inventory for 16 books | PENDING (Epic 2 cycle) |

### Next cycle

The next cycle is Epic 2's pre-flight: create `kanban.md` with the
sixteen per-book epics as ready cards; verify branch + OAuth + tree
state; run cycle-0 trap-report + work-inventory for all sixteen books.
Epic 3-N+ per-book cycles dispatch after.

## Cycle 0.0+1 — Unattended-mode acknowledgment (operator directive 2026-08-01)

**Date:** 2026-08-01
**Cycle ID:** `SD30-LAND-2` (unattended-mode directive landing)
**Operator:** Todd Hintzmann (out of town per directive)
**Surface:** this directory (`docs/release/SD-30-occult-and-companion-content-ingestion/`)

### What landed

The operator is out of town and may not see the harness's output for days. Per
operator directive 2026-08-01, this bundle operates in **unattended mode**.

Cycles MUST NOT pause to ask the operator questions. The operator's verbatim:

> "include instructions to all 3 that indicate they will be running in unnattended
> mode since i will be out of town while this runs. They may not stop to ask
> questions - it might be days before i notice."

The doctrine is mirrored across three files:

- `loop-instruction.md` §"OPERATING METHOD" sub-callout (cycle supervisor reads it first).
- `decisions.md` Decision §21 (load-bearing doctrine entry).
- `progress.md` Cycle 0.0+1 (this entry — per-cycle receipt confirms the operator-on-record).

The receipt chain is the operator's after-return review surface. When the
operator returns, the cycle receipts in this file carry the per-cycle decisions
that the harness made on its behalf.

### Operating protocol summary (mirror of `decisions.md §21`)

1. Default-and-flag, not ask.
2. No `clarify` tool calls.
3. Blockers are recorded, not raised.
4. `decision-blocked` IS allowed.
5. Closure is a goal, not a stop signal.

### Bundle-specific unattended-mode notes

The largest content-source surface to date (sixteen books across occult + mythic +
Monster Codex + Inner Sea series + Book of the Damned ×2) is the highest-stakes
unattended-mode target. Most likely operator-decision points:

- **Epic 5 (Mythic Adventures) consumer surface** — record `decision-blocked`
  and route to `forward-scope-register.md C3.1` retrofit per the unattended-mode
  protocol.
- **Epic 3 (Occult Adventures) psychic-discipline consumer surface** — same
  retrofit routing via `C3.2`.
- **Epic 7+ (Inner Sea series) campaign-tool surface** — same retrofit routing
  via `C3.3`.

The deferred books (NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes
Handbook) remain deferred — they are out of scope per the 2026-08-01 absent-book
rule and the unattended-mode protocol does not change that.

## Cycle 0.0+2 — Launch-readiness audit + pre-launch remediation (2026-08-01)

**Date:** 2026-08-01
**Cycle ID:** `SD30-LAND-3`
**Actor:** `sd30-prelaunch-remediation` (operator-directed launch-readiness session)
**Surface:** this directory (`docs/release/SD-30-occult-and-companion-content-ingestion/`)

### What landed

Three parallel audits (package cross-refs, repo tooling, pcgen sources)
evaluated `loop-instruction.md` for unattended launch. Blockers found and
fixed; every figure below carries its command.

- **Roster fix (`c12b1905`).** 11 of 16 books had no `books[]` entry —
  `v06_work_inventory` scanned only `roleplaying_game/`. Twelve
  `campaign_setting/` paths added to `EXTRA_BOOK_DIRS` + stub registrations;
  all 16 now `future_state` with real not-started units. Verified:
  `python3` scan of regenerated `docs/work-inventory.json` against Decision 1
  slots 1-16. `v06_corpus_trap_report` bare names now resolve across
  subtrees (`cargo run --locked --bin v06_corpus_trap_report -- inner_sea_gods`
  → exit 0, captured directly).
- **Record correction: Occult Origins + Haunted Heroes Handbook EXIST** at
  `player_companion/{occult_origins,haunted_heroes_handbook}` (verified:
  `ls ~/workspace/repos/pcgen/data/pathfinder/paizo/player_companion/`).
  The 07-30 "absent" finding used the wrong search root and, for HHH, the
  bare stem `haunted_heroes`. **Operator re-ruled 2026-08-01: sixteen-book
  pin stands; both books deferred by explicit choice** (`decisions.md`
  Decision 1, `forward-scope-register.md C2.3/C2.4` corrected). NPC Codex
  and Planar Adventures remain genuinely absent (`find` across all
  publishers). Retro `correction` events emitted to
  `docs/retro/events/sd30-prelaunch-remediation.jsonl` (5 events).
- **`loop-instruction.md` fixes:** per-bundle progress path corrected to
  this directory (old path pointed at a nonexistent
  `~/workspace/programs/...` dir); step 0 "four candidates" text replaced
  with the sixteen-book truth; hard-stop known-instances line corrected;
  tranche/7 retro stats re-derived (ad-hoc 46% = 56/122; on-screen ~10%
  mentions / 8% sole — the quoted 14%-sole conflated two columns);
  duplicate steps 6/7 merged.
- **`kanban.md`:** rows reordered to claim-priority (epic-1 first, epic-2
  second) and gates encoded on every card's own Status — a top-down
  "highest-priority ready card" read now matches the mandated epic
  ordering.
- **`forward-scope-register.md` C3.1/C3.2/C3.3:** each now carries an
  explicit **Unattended safe default** (never invent a surface; classify
  honestly or OPEN_FINDINGS + shortfall + `decision-blocked`; move on) —
  the fallback `loop-instruction.md` UNATTENDED MODE §4 depends on.
- **`scope-draft.md`:** 07-30 pre-confirmation text (shape finding,
  operator questions, four-book epic table with `0.6.<build>`) rewritten
  as dated RESOLVED records so no cycle reads a scope disagreement.
- Citation drift fixed across `decisions.md` (§14→§13/§14a, §7→§8,
  Recommended-sequencing attribution), `epic-breakdown.md` (§6→§7,
  Closure-F2 authority, ISWG double-count), `README.md` (decision count,
  epic count, authority-surface publish state), `technical-requirements.md`
  TR-30-010 + `acceptance-and-verification.md` AT-30-011 (publish landed).
- **Cycle 0.0's Surface line** reads `programs/codex/requirements/...` —
  correct at authoring time (pre-publish), superseded by the move-not-copy
  publish; the receipt is left as signed history and this entry is the
  correction of record.

### Verification

- `./scripts/verify.sh` full: **PASS**, 10 stages, exit 0 captured
  directly (`docs/retro/events/sd30-prelaunch-remediation.jsonl`
  verification event). Baseline note: `ROOT_FULL_TESTS` 5930→5933
  (new tests, this commit set + sibling SD-28/SD-29 remediation) —
  baseline moved in its own commit per DoD item 7.
- `cargo test --locked --test v06_work_inventory`: 14 passed.
- Idempotence: second generator run diffs `generated_at` only (checked
  post-commit, discarded).

### Pre-launch state after this cycle

| Check | Status |
|-------|--------|
| `kanban.md` exists with ready queue | ✅ (epic-1 first, gates encoded) |
| Branch `tranche/10` pushed to origin | ✅ (cut from remediated `tranche/8` tip) |
| OAuth credentials valid | ✅ (`gh auth status`: electricm0nk, repo scope) |
| Working tree clean | ✅ at cut time (sibling SD-28/SD-29 sessions share this checkout — re-run `git status` at launch) |
| Cycle-0 trap-report + work-inventory for 16 books | Epic 2's cycle (tooling now reaches all 16) |

---

(c) Per-cycle receipts append below this line as cycles fire.

## Cycle R.0 — Re-scope: `class_feature`/archetype bundle (operator directive 2026-08-10)

**Date:** 2026-08-10
**Cycle ID:** `SD30-RESCOPE-1`
**Operator:** Todd Hintzmann
**Surface:** this directory, renamed `docs/release/SD-30-class-feature-archetype-bundle/` (`git mv`
from `SD-30-occult-and-companion-content-ingestion`, history preserved)

### What landed

SD-29's re-scope to corpus-wide kind lanes (`SD-29-corpus-wide-catch-up-lanes/decisions.md §38`,
commit `472acb4f`) claimed every kind except `class_feature`, flagging a live collision with SD-30's
sixteen-book list (`§38.5`, R-29-009/OQ-29-004) without resolving it. Operator directive 2026-08-10:
**SD-30 becomes the `class_feature` bundle.**

- Directory renamed via `git mv`.
- `decisions.md §33-38` added: figures re-derived (15,472 units, 23 books, 40.2%, 109 grounded/0.7%,
  all confirmed exactly against the operator's own brief — no correction needed this pass, unlike the
  prior three briefs the operator flagged); `§34` verifies the SD-28 `§60`/`§63`/`§64` inheritance
  directly; `§35` closes the SD-29 collision by dissolving the book list outright; `§36` finds Epic
  14's harness widening is spell/equipment-shaped, not `class_feature`-shaped, and does NOT move to
  SD-30; `§37` states the per-class measurement-gated launch order; `§38` characterizes the
  2,958-unit `unknown` bucket as a classification/design question owned by the new Epic 4, citing
  SD-28's own prior Epic 15 findings on the same bucket.
- `epic-breakdown.md` and `kanban.md` re-cut from 21 book-shaped cards to 9 dependency-shaped epics:
  Identifier Cleanup, Operator Pre-Launch, PI-Screening Provenance Gate (new — mirrors SD-29 Epic 3),
  Per-Class Archetype Measurement (gates the rest), Archetype Mechanism, Per-Class Chassis Sweep,
  Build Version Numbering, Bundle Code Review, Closure.
- `forward-scope-register.md`: book-specific C2.1-C2.4 retired as moot; C3.1-C3.3 narrowed or retired
  to their `class_feature` shape only; new C1.3 records the `corpus-work-channels.md §9.1` successor
  assignment.
- `risks-and-open-questions.md`: book-shaped risks retired/narrowed; new R-30-010/011/012 cover the
  per-class gate, `unknown`-bucket characterization discipline, and the closed collision; OQ-30-004
  records the collision's closure.
- Mirrored resolution written into SD-29's own `decisions.md §38.5` and `risks-and-open-questions.md`
  (R-29-009/OQ-29-004) — reference-and-resolution only, SD-29's scope/epics untouched, per this
  change's write-scope authorization.

### What did NOT change

Branch `tranche/10`, build version `0.10.<build>`, Hermes-board retirement, reach-gate-as-DoD, the
`Workflow`-tool operating form, and the class-grant boundary with SD-28 (Occultist/Spiritualist/
Medium/Mesmerist, `decisions.md §5`) — none of these were book-scoped, all survive the re-scope
unchanged.

### Next cycle

Epic 1 (identifier cleanup) and Epic 2 (operator pre-launch, re-derived for the 23-book
`class_feature` population) fire first, unchanged in shape from before the re-scope. Epic 4's
class-inventory feature seed (SD30-E4-F1) is the first genuinely new work: enumerate which
`class_feature`-bearing classes remain unmeasured beyond SD-28's 28-class/25-verified set.
### 2026-08-14 — P0.2: PF1e dashboard producer versioned and hardened

Addresses `state-goals-and-lessons.md §1.3` hazards 4 and 5.

- **Backup**: live `pf1e_dashboard_producer.py`/`observer.py` copied to
  `/home/ubuntu/swarm-observer/.backups/{pf1e_dashboard_producer.py,observer.py}.pre-p02-versioning-2026-08-14`
  before any edit.
- **Versioned home**: source of record is now `scripts/observer/pf1e_dashboard_producer.py`
  and `scripts/observer/observer.py` in this repo. The former hermes-tree copies at
  `/home/ubuntu/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/` were
  replaced with symlinks to the repo copies, so the existing cron line (unchanged) now runs the
  versioned scripts:
  `*/5 * * * * /usr/bin/flock -n /home/ubuntu/swarm-observer/PF1e-dashboard.lock /usr/bin/python3 /home/ubuntu/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/pf1e_dashboard_producer.py >> /home/ubuntu/swarm-observer/pf1e-dashboard-producer.log 2>&1`
  5-minute cadence unchanged; crontab itself was not edited.
- **Hazard 4 fix (unrecognised-status crash)**: `build_unit_shards()`'s per-unit loop called
  `doneness_verdict()` with no guard — the ONE call site (of two) that let a `ValueError` from an
  unrecognised `(wiring_class, status)` pair escape uncaught, crashing `main()` and publishing
  nothing for that cron tick. Now wrapped in try/except: degrades the unit to the `unmeasurable`
  doneness verdict, logs a loud `WARNING` line to stderr/the producer log, and records the
  offending cell under a new `doneness_unmapped` dict per kind plus a top-level
  `doneness_unmapped_seen` boolean flag in `unit_index`. `SHARD_SCHEMA` bumped 12 -> 13 (new
  fields would otherwise be hidden behind the existing shape-gated shard cache).
- **Hazard 5 fix (stale wiring-class cache)**: `compute_wiring_class_summary()`'s cache-validity
  check compared only mtime and schema, not which `doc_path` the cache was actually computed
  from — a cache left over from a different `doc_path` invocation, newer than an unrelated doc,
  could be served silently ("produced a false zero during measurement"). Added a
  `cached.get("source_document") == doc_path` requirement alongside the existing mtime/schema
  checks.
- **Verification**: manual run against the live `docs/work-inventory.json` (38,521 units)
  produced JSON with the same top-level keys as `.last-good` and matched `total_units`; watched
  one live 5-minute cron tick land clean (`generated_at` advanced, no traceback, exit 0).
  Degrade path proven by feeding a scratch copy of the inventory with 5 units' `status` doctored
  to a bogus word (`quantum-superposed`, never emitted by the real generator): hardened producer
  published successfully with `doneness_unmapped_seen: true` and the exact 5 units bucketed;
  the pre-fix backup, run against the identical doctored input, crashed with
  `ValueError: doneness: unmapped 'derived' + 'quantum-superposed'` and exit 1 (nothing
  published) — confirming the fix actually changes behavior, not just adds unreachable code.
- **`status_sources_agree: false` diagnosis**: NOT a producer bug. It is hazard 1 from
  `state-goals-and-lessons.md §1.3` (the `v06_work_inventory --summary` regenerator drops the
  2,371 `literal-verified`/`fixture-verified` stamps on a plain run). The committed
  `docs/work-inventory.json` (`generated_at` 2026-08-14T02:06:11Z) still carries 2,322
  `literal-verified` + 49 `fixture-verified` records; a fresh `--summary` run has zero of either,
  shifting every other status count by roughly the same total. Left unfixed here per scope
  (`v06_work_inventory.rs` is owned by another agent this cycle).

### 2026-08-14 — P0.5: pre-launch checklist + full verification gate receipt

Executed the SD-30 pre-launch checklist (loop-instruction.md pre-launch items) and the full
`scripts/verify.sh` gate ahead of the operator's VM resize / Phase 1 orchestrator launch.
`RETRO_ACTOR=p05-prelaunch-gate`.

- **Tree hygiene**: `docs/retro/events/codex.jsonl` carried an uncommitted modification (retro
  events from a prior session). Validated (`python3 scripts/retro.py validate` — 1008 events, all
  valid) and committed standalone as `277e934e`.
- **Claims/collisions check**: `git worktree list` showed 10 stale `.claude/worktrees/wf_*`
  entries with no live owning processes. Two `.reclaim-claim` files found
  (`codex-target-gate-green`, `codex-target-wiring-classifier`); both held dead PIDs (2046131,
  336955 — neither present in `ps`). `hermes kanban list` on the active board (codex-tranche-5)
  showed zero non-done cards. No cargo/rustc/verify.sh processes running. Conclusion: no live
  claims. Note: `codex-target-gate-green` (23.8GB) was modified within the reclaim script's 6h
  window and correctly skipped as "too young" by `reclaim.sh` — consistent with the `gate-green`
  teammate having built recently, not a false negative in the liveness check.
- **Disk/hardware budget** (2026-08-14 ~11:51 EDT, measured):
  ```
  $ nproc
  8
  $ free -h
                 total        used        free      shared  buff/cache   available
  Mem:            45Gi        5.6Gi        20Gi       1.6Mi        20Gi        40Gi
  Swap:              0B          0B          0B
  $ df -h /home/ubuntu
  Filesystem      Size  Used Avail Use% Mounted on
  /dev/sda1       968G  181G  787G  19% /
  $ uptime
   11:51:47 up 2 days,  4:54,  2 users,  load average: 1.02, 1.07, 1.04
  ```
  Per `decisions.md §47`: 8-core budget, concurrency cap stays at 3. The announced VM resize to
  16 shared cores has not landed — no re-derivation triggered this pass; kept as measured (8
  cores), not projected forward.
- **Reclaim**: `./scripts/reclaim.sh --apply` reclaimed 1 item, 0 bytes total (deleted the local
  branch `tranche/9`, already merged into `origin/develop`). 26 items skipped: cargo-target
  candidates too-young or not-a-cargo-dir, verify-logs too-young, worktrees/branches under
  `.claude/worktrees/` forbidden-path-protected. No cargo-target bytes reclaimed this pass.
- **Full gate, first run — RED** (`b88b18fa`, mode `full`, `/tmp/codex-verify-WCt3I9`,
  873s):
  ```
  SUMMARY
    passed:  15  preflight-disk pi-sweep audit-selftest reclaim-selftest driver-selftest
                 corpus-sweep-selftest root-lib desktop reach corpus-sweep frontend-install
                 frontend-test frontend-typecheck clippy class-dump
    FAILED:  1  root-full

  RESULT: FAIL — logs in /tmp/codex-verify-WCt3I9
  VERIFY_EXIT=1
  ```
  Cause: `no_foreign_home_paths::no_foreign_absolute_home_path_under_tests_src_or_scripts`
  found 26 hardcoded `/home/ubuntu` literals in `scripts/observer/observer.py` and
  `scripts/observer/pf1e_dashboard_producer.py` — fallout from the same-day P0.2 change
  (`971cc063`) that moved those files into the repo as source-of-record under `scripts/`, the
  path this guard test scans. The files' pre-existing hardcoded-path defaults had never been
  scanned before the move and tripped the guard for the first time. Escalated to the team lead
  per instruction rather than fixed locally (file ownership was P0.2's, not P0.5's).
- **Remediation**: P0.2's author (`p02-producer`) converted the 26 literals in both files to the
  `os.path.expanduser("~/...")` convention the guard requires. **Commit-attribution anomaly**: a
  shared-index race folded the fix into `89078307` ("docs(sd30): absorb SD-32 into SD-30 and
  delete the package") rather than landing as its own commit — verified directly via
  `git diff 971cc063 89078307 -- scripts/observer/`, which shows exactly the 26-literal
  `os.path.expanduser` conversion and nothing else. No separate `fix(sd30): resolve observer
  paths` commit exists on `origin/tranche/10`; the fix's content is real and correct, only its
  commit message is misattributed to the doc-absorption change it happened to ride in on.
- **root-full re-run — GREEN** (`89078307`, mode `--only root-full`,
  `/tmp/codex-verify-wTycIW`, 191s):
  ```
  SUMMARY
    passed:  1  root-full

  BASELINE NOTES (not failures — update deliberately):
    - BASELINE_ROOT_FULL_TESTS baseline is stale: 6393 recorded, 6398 measured. Update
      /home/ubuntu/workspace/repos/codex/scripts/verify-baselines.env.

  RESULT: PASS
  VERIFY_EXIT=0
  ```
  6398 passed across 547 suites (all 526 `tests/*.rs` suites executed) — above the
  `ROOT_FULL_TESTS=6393` floor, so no failure; baseline left untouched per instruction (floors
  are floors, not auto-bumped).
- **Combined gate result**: stages 1-15 (preflight-disk, pi-sweep, audit-selftest,
  reclaim-selftest, driver-selftest, corpus-sweep-selftest, root-lib, desktop, reach,
  corpus-sweep, frontend-install, frontend-test, frontend-typecheck, clippy, class-dump) are
  carried from the first run at `b88b18fa`, unaffected by the observer-path fix or the
  doc-only SD-32-absorption commit. Stage 16 (root-full) re-ran clean at `89078307`. Net:
  **16/16 gate PASS** across the two runs, no fixes applied by this checklist run itself.
- **Dashboard cron cross-check**: `/home/ubuntu/swarm-observer/PF1e-dashboard.json`
  `generated_at: 2026-08-14T16:15:02Z`, ~1.2 minutes old at check time (well within the 10-minute
  freshness bar); `doneness_unmapped_seen: false`. Cron ticking clean.

**P0.5 verdict: PASS.** Pre-launch checklist complete, gate green (across the documented
two-run/one-fix sequence), no live claims, disk/hardware nominal, dashboard cron healthy. Ready
for the operator's VM resize and Phase 1 orchestrator launch per `sd30-launch-readiness`
sequencing.

## 2026-08-14 — Split: Phase 3 to SD-31, Phase 4 to SD-32 (`decisions.md §51`)

Operator ruling, verbatim: "ok, let's split phase 3 and phase 4 into their own SD's. SD-31 and SD-32.
Take the existing SD-31 and rename it to SD-33." Executed as three commits: (1) rename
`SD-31-pcgen-character-import` → `SD-33-pcgen-character-import` (git mv, history preserved, internal
identifiers updated, live SD-31 owner citations in SD-29's forward-scope register/release-notes updated
to SD-33); (2) create `docs/release/SD-31-corpus-closure-grind/` (former Epics 4/5/6/10/11 + grind-lane
Epic 14) and `docs/release/SD-32-engine-capability-builds/` (former Epics 12/13 + capability-lane Epic
14), each a full compliant package chassis with binding rules reproduced verbatim; (3) this package's
own `decisions.md §51`, `README.md`, `kanban.md`, `epic-breakdown.md`, and
`acceptance-and-verification.md` updated to record the narrowed scope — moved epics marked, not
deleted, per this package's standing convention. SD-30's remaining live scope: Epic 0 (instrument-apply),
Epic 1 (identifier cleanup), Epic 2 (pre-launch), Epic 3 (PI-screening gate, now a cross-SD dependency
consumed by both successors), Epic 7 (version numbering), Epic 8 (bundle code review), Epic 9 (closure).
The `§45` 100%-mandate exit bar is unchanged in substance — it becomes the joint SD-30→SD-31→SD-32
program's exit criterion.
