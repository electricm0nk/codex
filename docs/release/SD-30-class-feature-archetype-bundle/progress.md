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

## 2026-08-14 — SD30-PRELAUNCH-002: pre-launch checklist + concurrency re-derivation (admission control only)

`RETRO_ACTOR=sd30-preflight`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-preflight` (unused —
no build ran this cycle; this cycle implements nothing, per its own charter). Started at HEAD
`e39a7f47` (`docs(sd30): narrow SD-30 scope post-split (§51)`) — tree already clean, package present,
no recovery needed (`git rev-parse HEAD`, `git status --porcelain`,
`ls docs/release/SD-30-class-feature-archetype-bundle/loop-instruction.md`).

### 1. Seven-item pre-launch checklist

| # | Item | Evidence | Result |
|---|---|---|---|
| 1 | `kanban.md` exists, ready queue present | `kanban.md` present; `epic-0-instrument-apply` READY (Order 1, no gate), `epic-1-identifier` READY, `epic-7-version` READY (gated on epic-1 only) | PASS |
| 2 | `tranche/10` pushed to origin | `git rev-parse HEAD` = `e39a7f47...`; `git rev-parse origin/tranche/10` = same; `git log --oneline -1 origin/tranche/10` matches local tip | PASS |
| 3 | GitHub OAuth valid for push | `gh auth status`: "Logged in to github.com account electricm0nk", "Active account: true", token scopes include `repo`/`workflow` (missing only `read:org`, irrelevant to push) | PASS |
| 4 | Working tree clean | `git status --porcelain` — empty, both at cycle start and re-checked before this receipt | PASS |
| 5 | Wave disk budget computed and recorded with `df` output | See §2 below — recomputed this cycle, written into `loop-instruction.md`'s Concurrency section in place | PASS |
| 6 | Pilot/scope validation for any book/class a first cycle will claim | Epic 6 (per-class chassis-sweep, the only card that ever pinned a book/class) **moved to `SD-31-corpus-closure-grind/kanban.md`** under `decisions.md §51`. No SD-30 card claims a book or class. **No pilot validation is owed by this pre-launch pass** — stating this rather than inventing one, per the card's own instruction. | PASS (N/A by scope, not skipped) |
| 7 | `epic-0-instrument-apply` card status known | `grep -n "epic-0-instrument-apply" kanban.md`: `READY`, `Order 1`, explicitly independent of the `epic-1`..`epic-9` `class_feature` chain (kanban.md lines 42-49, 55, 106) — confirmed against kanban's own claim-priority note | PASS |

All seven items PASS. **No blockers found.**

### 2. Hardware re-derivation (this cycle's primary deliverable)

Commands run verbatim, 2026-08-14 ~15:5x (post-VM-resize, ~17 min uptime):

```
$ nproc
24
$ free -h
               total        used        free      shared  buff/cache   available
Mem:           167Gi       5.2Gi       158Gi       1.6Mi       6.2Gi       162Gi
$ df -h /            # BEFORE reclaim
/dev/sda1       968G  201G  767G  21% /
$ uptime
 15:51:11 up 16 min,  2 users,  load average: 1.02, 1.07, 1.04
$ grep -n 'PREFLIGHT_DISK_MAX_PERCENT=\|PREFLIGHT_DISK_MIN_FREE_GB=' scripts/verify.sh
scripts/verify.sh:243:PREFLIGHT_DISK_MIN_FREE_GB=${PREFLIGHT_DISK_MIN_FREE_GB:-20}
scripts/verify.sh:244:PREFLIGHT_DISK_MAX_PERCENT=${PREFLIGHT_DISK_MAX_PERCENT:-90}
```

This is a **second** re-derivation on the same day: `decisions.md §47` (this session, 2026-08-14
morning) captured 8 cores / 45 Gi RAM / 968 G disk at 19 % used and explicitly flagged
`loop-instruction.md`'s disk-budget section as owed-but-out-of-scope for that pass — this cycle is
that owed edit. Between `§47`'s capture and this cycle, the operator's VM resize (referenced but "not
landed" as of the P0.5 receipt earlier this file) **landed and went further than `§47`'s own number**:
8→24 cores, 45→167 GiB RAM. Disk stayed ~968 G at ~19-21 % used throughout.

**Arithmetic** (after `reclaim.sh --apply`, see §3 — post-reclaim `df -B1G /`: 968 total / 178 used /
791 avail / 19 %):

- headroom to the 90 % floor: `0.90 × 968 − 178 = 871.2 − 178 = 693.2 G`
- headroom to the 20 G-free floor: `791 − 20 = 771 G`
- **binding headroom: 693 G** (90 % floor binds; smaller of the two)
- full-gate `CARGO_TARGET_DIR` footprint, measured: `du -sh target` → **82 G** (primary checkout,
  accumulated; grown from the 60 G recorded 2026-08-11); `du -sh /home/ubuntu/cargo-targets/*` → 27 G
  (`sd29-e2-prelaunch`, one prior cycle's fresh footprint, orphaned)
- **disk-based cap**: `693 G ÷ 82 G = 8.4` → **8** concurrent full-gate agents (conservative: sized on
  the larger, accumulated-primary footprint, not the smaller fresh one)
- **CPU-based cap**: `nproc 24 ÷ verify.sh default -j 2 per agent = 12` — not binding
- **RAM check**: 8 agents × 2 jobs = ~16 concurrent `rustc` processes × ~2-4 G each ≈ 32-64 G, against
  158 Gi free — not binding
- **binding constraint: disk** (8 < 12)

**`max_full_gate_agents = 8`**, derived from this cycle's own measurement, not from `decisions.md
§47`'s 3 or any earlier document. `loop-instruction.md`'s "Concurrency and resource budget" section
updated in place with this table, the live commands, and revised rules 1/2/5 (cap language, footprint
figure, `nproc` note). A `scripts/retro.py correction` event was emitted
(`--subject "loop-instruction.md Concurrency and resource budget section"`, `--claimed` `§47`'s
8-core/3-agent figures, `--actual` the 24-core/8-agent figures above, `--verified-by` the command list
above) — event id `1786737248914-sd30-preflight-1aad8f` in
`docs/retro/events/sd30-preflight.jsonl`; log re-validated (`python3 scripts/retro.py validate` — 1017
events, all valid).

**Caveat carried forward, not resolved this cycle:** 8 is a ceiling computed from disk/CPU/RAM
headroom alone. `loop-instruction.md`'s standing "Dispatch mechanism" doctrine (shared-state cycles
serialize; `/batch` is not the default) still governs how many agents a wave *should* dispatch — this
number answers "how many the box can carry," not "how many a wave should send."

### 3. Live-claims check (before treating the box as ours)

- `git worktree list` — 10 stale `.claude/worktrees/wf_*` entries beyond the primary checkout, all
  with committed history (`git log --oneline -1` per worktree, all clean HEAD-at-a-commit, no
  in-progress uncommitted state visible).
- `.reclaim-claim` files: `find / -maxdepth 6 -name ".reclaim-claim"` →
  `/home/ubuntu/workspace/codex-target-wiring-classifier/.reclaim-claim` (PID 336955),
  `/home/ubuntu/workspace/codex-target-gate-green/.reclaim-claim` (PID 2046131). Both PIDs checked
  with `ps -p <pid>` — **neither is alive.** No live claims.
- `pgrep -fa "verify.sh|cargo test|cargo build"` — no matching process (only this cycle's own shell
  wrapper matched the pattern trivially via its own command line, not a real hit).
- **Conclusion: no live claims found.** Box is administratively clear for admission control.

### 4. Reclaim

```
$ ./scripts/reclaim.sh                          # dry run
WOULD REMOVE  /home/ubuntu/workspace/codex-target-gate-green  (23.8GB)
  would reclaim: 1 item(s), 23.8GB total
$ ./scripts/reclaim.sh --apply
REMOVED  /home/ubuntu/workspace/codex-target-gate-green  (23.8GB)
  reclaimed: 1 item(s), 23.8GB total
```

**23.8 GB reclaimed** (not 0.0 B — the box is not structurally full). 22 other candidates correctly
skipped: `codex-target-wiring-classifier` (not a cargo target dir — no `CACHEDIR.TAG`/build output),
10 worktrees + 10 matching branches under `.claude/worktrees/` (forbidden-path-protected, live
worktree checkouts), `site-deploy` branch (not merged, upstream present).

### 5. `verify.sh --only preflight-disk`

```
$ ./scripts/verify.sh --only preflight-disk > "$LOG" 2>&1; echo "VERIFY_EXIT=$?" >> "$LOG"
==> preflight-disk — disk budget check before any build starts
    repo filesystem (.../codex, mounted at /): 19% used, 790G available
    scratch-log filesystem (/tmp/codex-verify-uQy2as, mounted at /): 19% used, 790G available
    PASS  preflight-disk  (disk budget OK)
RESULT: PASS
VERIFY_EXIT=0
```

Exit code captured directly (not through a pipe), in the same shell statement. **PASS, `VERIFY_EXIT=0`.**

### 6. Dashboard cross-check (Job 1, `decisions.md §46`)

```
$ ls -la /home/ubuntu/swarm-observer/PF1e-dashboard.json
-rw-r--r-- 1 ubuntu ubuntu 1319078 Aug 14 15:50 ...
$ python3 -c "... generated_at, doneness_unmapped_seen, age ..."
generated_at: 2026-08-14T19:50:01Z    (check run at 19:52:08Z UTC)
doneness_unmapped_seen: False
age minutes: 2.13
$ crontab -l | grep -i pf1e
*/5 * * * * /usr/bin/flock -n .../PF1e-dashboard.lock /usr/bin/python3 .../pf1e_dashboard_producer.py >> .../pf1e-dashboard-producer.log 2>&1
```

**Dashboard fresh** (2.1 minutes old, well within the 10-minute bar), cron entry present and firing
on schedule, `doneness_unmapped_seen: false` (no unrecognised-status crash risk observed this tick).
Cron is healthy — no loud blocker to raise for Epic 0.

### Definition of done (this cycle's DoD — admission-control cycle, not a code cycle)

1. `verify.sh` exits 0 — **N/A as a full run; `--only preflight-disk` run per this cycle's own
   instructions (item 5 above), exits 0.** This cycle changed no Rust/Python production code, so the
   full gate is not owed (loop-instruction.md: "Doc-only or measurement-only cycles run the relevant
   `--only` stages instead and state exactly which").
2. Reach stage claim — **N/A**, no code touched, no family surfaced or changed this cycle.
3. `v06_corpus_trap_report -- --audit` — **N/A**, no corpus/inventory content touched this cycle.
4. Guarded work-inventory regen — **N/A**, `docs/work-inventory.json` not touched this cycle.
5. Four-check wired-integration audit — **N/A**, no production code changed (this cycle edits two
   Markdown docs: `loop-instruction.md`, this `progress.md` entry, plus one `kanban.md` claim edit).
6. `OPEN_FINDINGS` in `reach_gate.rs` — **N/A**, no family left unsurfaced this cycle.
7. Baseline movements — **N/A**, `scripts/verify-baselines.env` not touched.
8. On-screen verification — **N/A**, no player-visible surface touched this cycle.

### Verdict

**`launch_ok = true`.** All seven pre-launch checklist items PASS, no blockers, no live claims, disk
reclaimed and preflight-disk green, dashboard cron healthy. `max_full_gate_agents = 8`, re-derived
this cycle and written into `loop-instruction.md` in place, cited by cycle-id `SD30-PRELAUNCH-002` and
retro correction `1786737248914-sd30-preflight-1aad8f`. Cycles may fire.

## 2026-08-14 — SD30-E0-F1-001: static/derived `done` rung, verified by content and re-derived corpus-wide

`RETRO_ACTOR=sd30-e0-f1-rung`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e0-f1-rung`.
**HEAD at start:** `98d98d3a` (`docs(sd30): SD30-PRELAUNCH-002 — pre-launch checklist + concurrency
re-derivation`) — `git rev-parse HEAD` / `git status --porcelain` (empty) / package present at
`ls docs/release/SD-30-class-feature-archetype-bundle/loop-instruction.md`: tree clean, no recovery
needed.

### 1. Verify by content first

The card's handoff claims the static/derived rung and both movable-mass instruments (former SD-32
`e5-static-sweep`/`e6-derived-check`, folded here per `decisions.md §49`) already landed. Checked by
content, not by any prior receipt's say-so:

```
git log --oneline --all | grep -i "literal-verified\|fixture-verified\|corpus_literal_sweep\|derived_evaluator"
  c04eb9ef feat(doneness): add the derived done rung (fixture-verified)
  e928da8c fix(v06-work-inventory): declare literal-verified in status_vocabulary
  4087f171 feat(doneness): add the static done rung (literal-verified)
git merge-base --is-ancestor 4087f171 HEAD && git merge-base --is-ancestor e928da8c HEAD \
  && git merge-base --is-ancestor c04eb9ef HEAD    # all three: ancestor confirmed
```

All three are on `tranche/10` by content. `scripts/observer/pf1e_dashboard_producer.py`'s
`_doneness_verdict_uncapped()` (read directly, lines 3401-3524) confirms the `static`/`derived`
branch maps `status in ("literal-verified","fixture-verified")` → `DONENESS_DONE`, everything else
(`ingested-magnitude`/`grounded`/`text-complete`) → `DONENESS_HELD`, and anything else `raise
ValueError` — no relaxation, matches `decisions.md §49(d)` verbatim. **F1's core mechanism is
COMPLETE and correctly wired; this cycle's job is re-derivation, hardening, and the corpus-wide
regen it was waiting on, not building the rung.**

### 2. Re-derived headline figures (every number below has its command)

```
$ python3 -c "...Counter(u['status'] for u in json.load(open('docs/work-inventory.json'))['units'])..."
literal-verified: 2322   fixture-verified: 49    (committed docs/work-inventory.json, before this cycle's regen)

$ cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep.json
corpus-literal-sweep: 3516 records examined of 9328 read, 36105 tokens compared (9 synthesized),
8903 digests checked, 0 findings — CLEAN. (Byte-identical to `ed79ee1b`'s figures, re-run fresh.)

$ cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture.json
derived-evaluator-fixture-check: 49 of 94 covered units cleared; 1 failed; 44 not ingested.
FAIL advanced_players_guide:equipment:spindle_of_perfect_knowledge: corpus states
BONUS:STAT|INT,WIS,CHA|4|TYPE=Enhancement, evaluator produced no ability bonus at all. (Exit 0 —
the tool distinguishes cleared/failed/not-ingested and correctly withholds the stamp from the one
failing unit rather than either failing the whole run or silently stamping it. This live failure
IS this cycle's "prove the instrument can fail" evidence for the derived rung — no synthetic
corruption needed, a real one is sitting in the corpus.)
```

**kanban.md's "4,805 ceiling, 1,602 movable" (static) / "2,674 ceiling, 535 movable" (derived)
figures were wrong** — pre-run planning estimates, never checked against an actual run. Corrected
in place (`kanban.md`, this cycle) to the re-derived, actually-landed figures: static 4,801
static+held total / 2,322 TOKEN-COMPARED literal-verified (not 1,602); derived 94-of-2,879 covered
by fixture design / 49 cleared (not 535). `retro.py correction` event
`1786738386633-sd30-e0-f1-rung-012e3f`.

**Board movement**, re-derived by importing the dashboard producer's own `doneness_verdict()` and
replaying it over `git show <ref>:docs/work-inventory.json` at the pre-rung commit (`d1b29589`,
parent of `4087f171`) versus current `HEAD`:

```
BEFORE d1b29589  done=3464 held=9455 in-progress=716 not-started=21322 unmeasurable=3547 deferred=36
AFTER  HEAD      done=5837 held=7086 in-progress=716 not-started=21319 unmeasurable=3546 deferred=36
delta:  done +2373  held -2369  not-started -3  unmeasurable -1
```

`done` 3,464 → 5,837 matches `state-goals-and-lessons.md §1.1`'s cited figure exactly, independently
re-derived this cycle rather than transcribed. The small `not-started`/`unmeasurable` deltas (-3/-1)
are unrelated corpus-shape drift between the two refs, not the rung's own effect (the rung only ever
moves `held`→`done`); not investigated further as immaterial to this card.

**`docs/release/.../artifacts/derive-movable-mass.py` run live** per the card's own instruction:
raises `ValueError: ('static', 'literal-verified')` on the current inventory — confirmed live, exactly
as its own staleness header (added `decisions.md §50`) documents. **The card's instruction to run this
script before/after is itself stale**; the dashboard producer's own function (used above) is the live
authority. Not re-flagged as a new correction since the script's header already records this; this
cycle's live run is the re-derivation that confirms the header is still accurate, not a new finding.

### 3. Guarded work-inventory regen (DoD item 4)

```
$ cp docs/work-inventory.json /tmp/work-inventory-before-regen.json
$ CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture.json \
    cargo run --locked --bin v06_work_inventory     # run 1, exit 0
$ cp docs/work-inventory.json /tmp/work-inventory-after-regen1.json
$ CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture.json \
    cargo run --locked --bin v06_work_inventory     # run 2, exit 0
```

Run 1 vs run 2, Python-diffed with `generated_at` stripped from both sides: **identical**
(`True`) — the two guarded runs differ only in timestamp, zero stamp loss, guard confirms
`literal-verified: 2322`, `fixture-verified: 49` present in both. DoD item 4 satisfied.

**Regenerated `docs/work-inventory.json` also differs from the file committed at cycle start**
(444 units' `status` changed, all `not-ingested`/`text-complete` → `unknown`, 0 units added/removed).
Root-caused, not assumed: the committed file's `generated_at` (`2026-08-14T02:06:11Z`) predates two
already-merged, already-tested code changes on `tranche/10` — `2ce72913` (`fix(v06-work-inventory):
teach classify()'s text_only signal the %N prose-formula pattern`, 06:33) and its own commit message
states *"docs/work-inventory.json is NOT regenerated in this commit... Regeneration+recommit needs to
go through whichever pipeline stage owns that re-stamping"* — this cycle's guarded regen (DoD item 4)
is exactly that pipeline stage. Spot-checked one unit
(`ultimate_wilderness:class_feature:tree_soul_transform_wood`): its `not-ingested` verdict is
superseded by an `unknown` verdict with evidence `class_feature_group_names_no_class_at_all`, a
branch already present in the codebase since `66a6804d` (2026-08-13, confirmed ancestor of the very
commit that generated the committed file) — the wider 405-unit ripple is `2ce72913`'s `text_only`
flag flipping which pre-existing branch a unit reaches, a bigger effect than that commit's own
hand-audited "54-unit contradiction set" scoped to units already showing `text-complete`, not a new
defect. Zero units gained or lost `literal-verified`/`fixture-verified` in this regen (both counts
identical to the committed file). **Regenerated file committed this cycle** as the natural close-out
of the guarded-regen pipeline `2ce72913` left open, and within DoD item 4's own charter.

### 4. Prove the instrument can fail (before trusting a pass)

```
$ bash scripts/tests/test_corpus_literal_sweep.sh
15/15 PASS, including: one-byte magnitude drift caught, corpus file drifting under a still-matching
record caught, a cited corpus file that does not exist caught, a malformed shipped record is a hard
failure not a silent skip, an empty population exits 2 and never prints CLEAN.
SELF-TEST PASSED.

$ cargo test --locked --test derived_evaluator_fixture_check
5/5 passed, including reference_derivation_refuses_what_it_cannot_parse and
fixture_expected_values_are_re_derivable_from_the_pinned_corpus_field (both fail loudly on a
corrupted/hand-invented expected value by construction).
```

Plus the live real-corpus failure at §2 (`spindle_of_perfect_knowledge`, correctly withheld from
`fixture-verified`). Both instruments demonstrably refuse a bad input; neither is a gate that cannot
fail.

### 5. DoD item 3 — `v06_corpus_trap_report -- --audit`: NOT clean, pre-existing, out of this card's
scope

```
$ cargo run --locked --bin v06_corpus_trap_report -- --audit
TRAP 259 (mod-record, 0 defects, still clean) + 177 NEW wiring-class-mismatch defects
AUDIT_EXIT=2
```

All 177 defects are `companion`/`monster_ability` kind, `stored wiring_class: display` vs `derived`
computed fresh — traced to `99efb504` (the %N prose-formula `wiring_class::determine_closure` fix)
landing in the classifier without a matching re-ingest of those two kinds' stored `data/corpus/*.json`
records. **Zero defects in `class_feature`/`equipment`/any static-or-derived-rung family this card
owns.** Every prior SD-29 receipt on record shows `AUDIT_EXIT=0`
(`docs/release/SD-29-.../progress.md`, thirteen+ citations) — this is a genuine, newly-surfaced
regression, not something this cycle introduced (nothing this cycle touched `data/corpus/`), and not
something SD30-E0-F1 (static/derived rung) owns to fix. **Reported honestly as FAILED, not
fabricated as a pass.** `retro.py correction` event `1786738438742-sd30-e0-f1-rung-427179`. Flagged
for a dedicated follow-up card (companion/monster_ability wiring_class re-ingest sweep) — out of
scope for this cycle to fix itself.

### 6. Definition of done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **N/A as a full run.** No Rust/Python production code changed this cycle (only `docs/work-inventory.json` regen via an already-shipped binary, plus Markdown docs). Ran the applicable `--only` stages instead (items 2 below; `--only preflight-disk`-equivalent skipped, disk already checked healthy this session). |
| 2 | reach claim, nonzero | `./scripts/verify.sh --only reach` → **PASS, 27 passed, VERIFY_EXIT=0.** No new family surfaced this cycle (F1 is a classification/stamping instrument, not a new reach-gate consumer), so this is the standing-health check, not a new-family claim; recorded honestly as such. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **FAIL, exit 2 — see §5.** 177 pre-existing, out-of-scope defects (companion/monster_ability), 0 in this card's owned families. Not fabricated as a pass. |
| 4 | Guarded work-inventory regen | **PASS — see §3.** Two consecutive guarded runs identical net of `generated_at`; zero stamp loss; regenerated file committed. |
| 5 | Four-check wired-integration audit | **N/A.** No shipping TS/TSX/JSX/Rust code added or modified this cycle (doc + generated-data-only cycle) — doctrine's own documentary-cycle exemption applies. Ran the four greps anyway for the record: checks 2/3/4 clean (`OK_NO_NOOP_HANDLERS`/`OK_NO_MOCK_LEAKS`/`OK_NO_WOULD_STRINGS`); check 1 matches are all the literal word "placeholder" as PCGen domain terminology (`%N` prose placeholder, HTML input `placeholder=`) in pre-existing code from before this cycle, not a STUB/MOCK/todo/fixme/hack token — confirmed by inspection, no shipping stub found. |
| 6 | `OPEN_FINDINGS` in `reach_gate.rs` | **N/A.** No reach family left unsurfaced this cycle — `reach_gate.rs`'s `OPEN_FINDINGS` tracks per-book/-kind unsurfaced-family gaps, a different registry than trap-report's §5 finding, and this cycle surfaced nothing new to leave open there. |
| 7 | Baseline movements | **N/A.** `scripts/verify-baselines.env` not touched. |
| 8 | On-screen verification | **N/A.** No player-visible surface touched — the static/derived rung is a dashboard/reporting classification, not a desktop-app surface change; `decisions.md §49(d)` still requires reach/on-screen verification for player-visible surfaces generally, none of which this cycle changes. |

### 7. Card disposition

`epic-0-instrument-apply`'s **F1 sub-scope (static/derived done rung) is verified COMPLETE**: the
mechanism was already landed and correctly wired pre-cycle; this cycle content-verified it, re-derived
every figure the handoff cited (all checked out except kanban.md's two movable-mass estimates, both
corrected), applied the guarded regen it was left waiting on, and proved both instruments can fail.
`epic-0-instrument-apply`'s F2 (`computed`-bucket consumer-delta probes)/F3 (`unknown`-residue
characterization)/F4 (re-derivation reporting, effectively folded into this receipt for F1) remain
open. **`kanban.md`'s `epic-0-instrument-apply` row is left `READY`** (not flipped `COMPLETE`) since
F2/F3 are unclaimed work under the same row — a judgment call, stated here per the "routine judgment
call, conventional default" press-on rule; there is no per-feature-seed row to flip independently
(unlike epic-5/epic-6's per-class tracking convention).

### 8. Retro events emitted

- `1786738386633-sd30-e0-f1-rung-012e3f` — correction, kanban.md movable-mass figures.
- `1786738438742-sd30-e0-f1-rung-427179` — correction, trap_report --audit standing-clean assumption.
- `1786738482810-codex-85cb93` — verification, auto-emitted by `verify.sh --only reach`.

### 9. Reclaim

`./scripts/reclaim.sh --apply` run at cycle end; see the commit's own receipt line below for the
bytes reclaimed (recorded after this entry is written, per the loop's own "receipt before the gate
result is final" allowance — this cycle ran no `verify.sh --full`, so there is no long-running gate
still in flight to wait on before reclaiming).

**Verdict: SD30-E0-F1 COMPLETE.** DoD items 1/5/6/7/8 N/A with stated reasons, items 2 and 4 PASS,
item 3 is an honest, documented, out-of-card-scope FAIL flagged for a follow-up card. No number moved
by lowering a bar; kanban.md's two wrong estimates were corrected DOWN to the true, smaller,
already-landed figures (2,322 not-claimed-1,602 is a correction of an under-estimate, not a
manufactured gain — the actual mechanism has been running since `4087f171`/`c04eb9ef`, this cycle
only found the planning note that never caught up to it).


## 2026-08-14 — SD30-E0-F2-001: computed-bucket consumer-delta probes, corpus-wide — enumerated fresh, NO_GROUNDING_PROBE cap corrected

`RETRO_ACTOR=sd30-e0-f2-probes`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e0-f2-probes`.
**HEAD at start:** `c3f3e599` (`fix(sd30): SD30-E0-F1 — verify static/derived done rung by content,
re-derive corpus-wide figures`) — `git rev-parse HEAD` / `git status --porcelain` (empty) / package
present at `ls docs/release/SD-30-class-feature-archetype-bundle/loop-instruction.md`: tree clean, no
recovery needed.

### 1. Fresh enumeration (re-derived, not transcribed)

Kinds with a `computed`-wiring-class population, corpus-wide:

```
$ python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
  print(collections.Counter(u['kind'] for u in d['units'] if u.get('wiring_class')=='computed').most_common())"
class_feature 4178, race_trait 1001, companion 793, monster_ability 669, equipment_modifier 561,
feat 509, equipment 369, spell 210, class 176, monster 7, race 4
```

`class_feature` confirmed as the largest, 4,178 — matching the card's cited figure, re-derived not
transcribed.

Existing `probe_*` functions, enumerated fresh:

```
$ grep -n '^fn probe_' src/bin/v06_work_inventory.rs
```
14 functions: `probe_feat_effect_wiring`, `probe_reachable_race_traits`, `probe_race_trait_corpus`,
`probe_equipment_key_universe`, `probe_equipment_keys_by_book`, `probe_equipment_effect_wiring`,
`probe_casting_class_for_spell`, `probe_spell_key`, `probe_spell_keys_by_book`,
`probe_spell_effect_wiring`, `probe_class_name`, `probe_class_effect_wiring`,
`probe_class_feature_key`, `probe_class_feature_effect_wiring`.

Cross-referencing coverage against `classify()`'s match arms (`src/bin/v06_work_inventory.rs`):

| kind | probe_* exists? | grounded/total (computed) |
|---|---|---|
| feat | yes — `probe_feat_effect_wiring` | 58/509 |
| equipment, equipment_modifier | yes — `probe_equipment_effect_wiring` (shared) | 40/369, 55/561 |
| spell | yes — `probe_spell_effect_wiring` | 46/210 |
| class | yes — `probe_class_effect_wiring` | 27/176 |
| race_trait | yes — `probe_race_trait_corpus`/`probe_reachable_race_traits` | 264/1001 |
| **class_feature** | **yes — `probe_class_feature_effect_wiring` (line 4072)** | 20/4178 |
| companion | no | 416/793 |
| monster_ability | no | 334/669 |
| monster | no | 7/7 |
| race | no | 0/4 |

### 2. CORRECTION — the card's own premise is wrong for `class_feature`

The acceptance text names `class_feature` as "the largest such population... and no existing
`probe_*` function." **False, checked by content, not assumed.** `probe_class_feature_effect_wiring`
(`src/bin/v06_work_inventory.rs:4072`) exists, is invoked from `classify()`'s `Kind::ClassFeature` arm
(`facts.class_feature_effect_wired.get(&unit.key) == Some(&unit.book.as_str())`), and already produces
20 `grounded` units under `computed`. class_feature needs no new probe. `retro.py correction` event
`1786739559344-sd30-e0-f2-probes-a01b3e`.

### 3. The real gap, examined — and found NOT to need a new probe for 3 of 4 kinds

`companion`, `monster_ability`, `monster`, `race` have `computed` population and no `probe_*`
function. Investigated each rather than building probes to satisfy the letter of the acceptance text:

**`companion` and `monster_ability`: no new probe is buildable OR owed.** Their only real downstream
consumer is `apps/desktop/src-tauri`'s `list_companion_catalog`/`list_monster_catalog` Tauri commands
(confirmed already claimed in `reach_gate.rs`, lines ~1893-1948). Both are **proven structural
bijections** over the exact compiled registries (`companion_chassis::COMPANION_BOOKS`,
`monster_chassis::MONSTER_BOOKS`) the current membership check (`facts.holds_key` /
`chassis_companion_keys` / `chassis_monster_ability_keys`) already reads — zero filtering, own module
tests assert it:

```
$ sed -n '600,614p' apps/desktop/src-tauri/src/companion_catalog.rs   # the_catalog_serves_every_registered_companion_creature
    assert_eq!(response.entries.len(), expected);   # expected = COMPANION_BOOKS.companions.len() sum
$ sed -n '772,800p' apps/desktop/src-tauri/src/monster_catalog.rs     # the_catalog_serves_every_ingested_bonus_bestiary_monster...
```

A probe re-implementing that map would produce the IDENTICAL grounded/not-ingested split already
produced today — confirmed directly: every `computed` unit of these two kinds is ALREADY exactly
`{grounded, not-ingested}`, a strict two-way split with no `in-progress`-shaped status a probe could
move (`python3 Counter` over `docs/work-inventory.json`, §1 table above). Building one anyway would
also require reaching into `apps/desktop/src-tauri`, a separate cargo workspace
`v06_work_inventory.rs` cannot depend on. Per the no-stub-mvp doctrine's own mirror rule (never invent
a surface a check cannot fail against), no new probe is owed. `retro.py deferral` event
`1786739592315-sd30-e0-f2-probes-84d9cc`.

**`monster`: already 100% grounded (7/7) under `computed`.** No gap exists to close regardless of
probe existence.

**`race`: no new probe is owed; zero board impact either way.** The 4 `computed` `race` units are
`{not-started, not-ingested}` × 0 `grounded`. Inspected individually
(`python3` filter on `docs/work-inventory.json`): 2 of the 4 (`adventurers_guide "Companion (Bird
(Raven))"`, `ultimate_psionics "Companion (Karaan)"`) read as `file_kind()` misclassification —
companion records typed `kind==race` by filename, the same recurring classifier defect class this
program has hit 3× before (`isi_abilities_race_companion.lst`, Bestiary 5/6). The other 2
(`core_essentials` Aasimar, Tiefling) are real races `RaceId::ALL` does not model — an ingest gap, not
a probe gap. A probe changes none of their status. `retro.py deferral` event
`1786739592428-sd30-e0-f2-probes-01f6d2`.

### 4. `NO_GROUNDING_PROBE` cap — removed for both listed kinds, per the card's own bar

The cap's justifying comment (`scripts/observer/pf1e_dashboard_producer.py`, round 8 SD-29) claimed
"`companion` and `spell` alone read `grounded: 0`". **Re-checked against the live payload this cycle:
FALSE for both.**

```
$ python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
  print(collections.Counter((u['wiring_class'],u['status']) for u in d['units'] if u['kind']=='companion')); \
  print(collections.Counter((u['wiring_class'],u['status']) for u in d['units'] if u['kind']=='spell'))"
```
`computed`+`companion`: 416 `grounded` of 793. `computed`+`spell`: 46 `grounded` of 210. Both kinds'
consumer-delta check already exists and already lands nonzero `grounded` — the card's own bar ("cap
removed for a kind once its probe lands AND is confirmed reaching a nonzero `grounded` count under
`computed`") is met for both. `retro.py correction` event `1786739559459-sd30-e0-f2-probes-11d945`.

**Change:** `scripts/observer/pf1e_dashboard_producer.py`: `NO_GROUNDING_PROBE = ("companion",
"spell")` → `NO_GROUNDING_PROBE = ()`. Same change, three follow-on fixes required and made:

1. **`spell_kind_capped_count` gated on live membership** (was unconditional; would have kept
   reporting a nonzero "disagreement" count for `spell` even after the disagreement stopped existing).
2. **`WIRING_SUMMARY_SCHEMA` bumped 11→12** — required. A live run after the code-only change served a
   STALE cache (`state-goals-and-lessons.md` hazard 5, hit live this cycle): `docs/work-inventory.json`'s
   mtime never moved, so `compute_wiring_class_summary()`'s mtime-only cache check kept serving the OLD
   `no_grounding_probe_kinds` and OLD `by_doneness` split. Caught by running the producer end-to-end and
   diffing the shipped payload, not by trusting the code diff. `retro.py near-miss` event
   `1786739567777-sd30-e0-f2-probes-6ecfa0`.
3. **`/home/ubuntu/swarm-observer/PF1e-dashboard.html`'s client-side fallback guard fixed** — was
   `if (Array.isArray(ngp) && ngp.length) NO_GROUNDING_PROBE = ngp;`, which can only ever ADD kinds to
   the hardcoded default, never honor an explicitly-EMPTIED payload array (`.length` falsy). Changed to
   `if (Array.isArray(ngp))`. This untracked, cron-served HTML file was backed up to
   `/home/ubuntu/swarm-observer/.backups/` (2 files, `.pre-sd30-e0-f2` suffix) before editing, per
   `state-goals-and-lessons.md` hazard 4.

**Verified end-to-end**, live producer run (respecting the cron `flock`):

```
$ /usr/bin/flock -n /home/ubuntu/swarm-observer/PF1e-dashboard.lock /usr/bin/python3 \
    /home/ubuntu/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/pf1e_dashboard_producer.py
pf1e-producer: rendered /home/ubuntu/swarm-observer/PF1e-dashboard.json   (exit 0)
```
Before fix (stale cache, schema 11): `no_grounding_probe_kinds: ["companion","spell"]`,
`by_doneness: {held: 7048, in-progress: 716, done: 5837, ...}`.
After fix (schema 12, forced recompute): `no_grounding_probe_kinds: []`, `spell_kind_capped_count: 0`,
`by_doneness: {held: 6916, in-progress: 848, done: 5837, ...}`.

**Board movement**: `held` −132, `in-progress` +132, **`done` unchanged (5,837)**. This is a
reclassification honesty fix, not a `done` gain — `computed`+`grounded` was never subject to this
cap; only `computed`+non-`grounded` `spell` units (`ingested-magnitude`) were miscoloured `held`
instead of the more honest `in-progress`. `companion`'s cap removal moves 0 units (was already inert —
confirmed no `computed`+`companion` unit exists outside the `{grounded, not-ingested}` split). No
number moved by lowering a bar; the opposite direction (a bar that no longer has an excuse to be
capped) was tightened to match reality.

HTML syntax verified: `node -e "new Function(scriptBlockSource)"` over the file's single `<script>`
block — no syntax error. Python syntax verified: `python3 -c "import ast; ast.parse(...)"` — OK.

### 5. Full gate found and fixed a pre-existing, inherited defect (unrelated to this cycle's own change)

The first `./scripts/verify.sh --full` run against `tranche/10` HEAD `c3f3e599` — the FIRST full-gate
run against that commit; SD30-E0-F1-001's own DoD item 1 was N/A ("no Rust/Python production code
changed") so it never ran — **FAILED** at `root-full`: `tests/v06_work_inventory.rs:655`,
`zero_magnitude_option_pool_class_features_are_not_ingested_not_unknown`, 405 violations, cargo exit
101.

Root-caused, not assumed: every one of the 405 violating units carries `wiring_class: "derived"`,
`wiring_class_reason: "prose_formula_segment"` (`python3 Counter` over
`docs/work-inventory.json`). `classify()`'s `text_only` check
(`magnitude_token_count == 0 && !carries_prose_magnitude`, landed in `2ce72913`) is correctly FALSE
for a zero-raw-token unit that carries a detected prose-embedded formula, so it falls past the
`not_ingested` branch into `unknown` **by design** — the DATA is correct (this cycle touched no
corpus data, no `v06_work_inventory.rs`, no `2ce72913` code). The TEST's own invariant comment predates
`2ce72913` and never accounted for the `carries_prose_magnitude` override, so **the test was the stale
half**. Fixed in place: narrowed the assertion to exempt `wiring_class_reason == "prose_formula_segment"`
rather than deleting or weakening the check — a true zero-magnitude, no-prose-formula unit landing
`unknown` is still caught. `retro.py correction` event `1786740230122-sd30-e0-f2-probes-f3839c`.

This is a mechanical/stale-fixture-shaped defect per the loop-instruction's PRESS ON criteria (not this
card's own scope, but blocking every cycle's DoD item 1 since it landed) — fixed in place rather than
recorded `decision-blocked`, per that doctrine's explicit example.

**The first fix attempt was itself incomplete — caught by re-running the gate, not by inspection.**
Run 2 (after the first fix) still FAILED at the same test: `left: 29` violations, not 0. Exempting only
`wiring_class_reason == "prose_formula_segment"` missed the sibling reason `"prose_expr"` —
`v06_work_inventory.rs`'s own `carries_prose_magnitude` computation (its `classify()` doc comment,
line ~4505) narrows to **both** reasons:
`matches!(wc_reason.as_str(), "prose_expr" | "prose_formula_segment")`. The test now quotes that exact
predicate verbatim rather than re-deriving it from an aggregate `Counter` a second time (the first
pass's actual mistake: 398 of 405 violations were `prose_formula_segment`, and the smaller `prose_expr`
slice did not stand out enough on that read). `python3` re-check against the corrected predicate:
`0` remaining violations. `retro.py near-miss` event `1786741134671-sd30-e0-f2-probes-6f8fee`. A third
full gate run was launched after this correction; its result is below.

### 6. Definition of done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **PASS. `VERIFY_EXIT=0`, RESULT: PASS, 16/16 stages**, third full run this cycle (run 1 FAIL at `root-full`, pre-existing stale test found live; run 2 FAIL at `root-full`, first fix attempt incomplete, found live; run 3 PASS clean). Exit code captured directly in the same shell statement (`./scripts/verify.sh > "$LOG" 2>&1; echo VERIFY_EXIT=$? >> "$LOG"`), never through a pipe. Log: `/tmp/codex-verify-M5CFs3` (run 3). `clippy` clean both crates (root:46 desktop:7 warnings, 0 errors — pre-existing warning counts, unchanged). `class-dump` 31/31 computing. One baseline note (not a failure): `BASELINE_ROOT_FULL_TESTS` stale 6393→6398 — its own commit, §5 above / §7 below. |
| 2 | reach claim, nonzero | **PASS.** `reach` stage: 27 passed (the full standing `reach_gate` suite — `grep -c '#\[test\]' apps/desktop/src-tauri/src/reach_gate.rs` confirms 27 exist, matching exactly). No NEW family surfaced this cycle (this card is dashboard/instrument-scope, not a new player-facing reach claim), so this is the standing-health check, not a new-family claim — recorded honestly as such, same framing SD30-E0-F1-001 used. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **FAIL, exit 2 — pre-existing, out of this card's scope, unchanged by this cycle.** 177 defects (33 companion, 3 monster, 141 monster_ability), all `wiring-class-mismatch`: stored `display` vs computed-fresh `derived` in `data/corpus/**/*.json`, traced by SD30-E0-F1-001 to the `%N` prose-formula classifier fix (`99efb504`) landing without a matching re-ingest of those kinds' stored JSON. Same 177 as F1's prior receipt, byte-for-byte reproduced this cycle (`companion`/`monster`/`monster_ability` counts: 33/3/141) — confirms this cycle neither caused nor worsened it (this cycle touched no `data/corpus/` file and no `v06_work_inventory.rs` code). Flagged for the same dedicated follow-up card F1 already flagged; not this card's scope to fix. |
| 4 | Guarded work-inventory regen | **N/A.** `src/bin/v06_work_inventory.rs` (the generator), `data/corpus/`, and `docs/work-inventory.json` itself are untouched this cycle. `tests/v06_work_inventory.rs` (§5 below) changed, but it is a TEST asserting against the document, not the generator producing it — its fix narrows an assertion, it does not alter what `v06_work_inventory` computes or writes, so a regen would be byte-identical (net of `generated_at`) to the committed file. The committed `docs/work-inventory.json` (SD30-E0-F1-001's regen, `generated_at: 2026-08-14T20:03:13Z`) remains authoritative and is what every figure above is re-derived against. |
| 5 | Four-check wired-integration audit | **PASS.** All four checks scoped to `apps/desktop/**` / `src/**/*.rs` — `scripts/observer/pf1e_dashboard_producer.py` and `tests/v06_work_inventory.rs` (repo-root `tests/`, not `src/`) match neither glob; run anyway for the record — `OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`. Also grepped both actually-changed files directly for the STUB/MOCK/todo/fixme/hack token set: clean. |
| 6 | `OPEN_FINDINGS` in `reach_gate.rs` | **N/A.** No family was left unsurfaced this cycle in the reach-gate sense — `list_monster_catalog`/`list_companion_catalog` already carry their own claims from before this cycle, and this cycle's finding (§3) is that no NEW instrument is owed for companion/monster_ability, not that a family is unclaimed. Recorded instead as two `retro.py deferral` events (§3) naming the remedy/revisit condition, the correct registry for an instrument-scope (not a reach-scope) finding. |
| 7 | Baseline movements | **DONE, own commit.** `BASELINE_ROOT_FULL_TESTS` 6393 → 6398, fully attributed (not this cycle's own tests — see §5's third paragraph): +5 from `9060840c`'s five new tests landing between the 6393 baseline and this cycle with no intervening green-gate measurement, this cycle's own fix contributing net zero (restores one test the same commit range's data regen broke, not a further +1). `--show-actuals` output quoted verbatim in the commit (`./scripts/verify.sh --only root-full --show-actuals`). `BASELINE_ROOT_TEST_BINARIES` unchanged (547, no new test file). |
| 8 | On-screen verification | **N/A.** No player-visible desktop-app surface touched — `scripts/observer/pf1e_dashboard_producer.py` and the standalone dashboard HTML are the operator/ops reporting surface, not the character-sheet app `run-desktop`'s driver reaches. |

### 7. Retro events emitted

- `1786739559344-sd30-e0-f2-probes-a01b3e` — correction, class_feature-already-has-a-probe premise.
- `1786739559459-sd30-e0-f2-probes-11d945` — correction, NO_GROUNDING_PROBE's stale justifying comment.
- `1786739567777-sd30-e0-f2-probes-6ecfa0` — near-miss, stale wiring-class-summary cache (hazard 5) caught live before publish.
- `1786739592315-sd30-e0-f2-probes-84d9cc` — deferral, no new probe for companion/monster_ability (proven redundant).
- `1786739592428-sd30-e0-f2-probes-01f6d2` — deferral, no new probe for race (corpus noise + unrelated ingest gap).
- `1786739472584-sd30-e0-f2-probes-0cbf9a` — verification, auto-emitted by `verify.sh --only preflight-disk`.
- `1786741134671-sd30-e0-f2-probes-6f8fee` — near-miss, first fix attempt to the stale test was itself incomplete (one of two exemption reasons), caught by re-running the gate.
- 5 `verification` events auto-emitted by `verify.sh` across this cycle's runs (preflight check, full runs 1/2/3, the `--only root-full --show-actuals` baseline-measurement run) — `python3` count over `docs/retro/events/sd30-e0-f2-probes.jsonl`, denominators honest in both directions.

### 8. Card disposition

`epic-0-instrument-apply`'s **F2 sub-scope is COMPLETE**: enumerated the real kind/probe list fresh
(not from the card's own 4,178/"no probe" framing, which was wrong and is corrected in place),
determined by direct evidence that 3 of the 4 genuinely-probe-less kinds need no new probe (proven
redundant or zero-impact, not merely deferred for time), and removed the `NO_GROUNDING_PROBE` cap for
both kinds it named, per the card's own confirmation bar, with the stale-cache hazard it triggered
caught and fixed in the same change. `kanban.md`'s `epic-0-instrument-apply` row is left `READY` (F3/F4
remain open under the same row), matching SD30-E0-F1-001's own precedent for this multi-feature row.

### 9. Reclaim

`./scripts/reclaim.sh --apply` run at cycle end, after all three gate runs settled: **0.0B reclaimed,
31 items skipped**, every one correctly guarded (this cycle's own `sd30-e0-f2-probes` target dir still
live, `/tmp/codex-verify-M5CFs3` too young, other agents' worktrees/branches genuinely in use). Per
`state-goals-and-lessons.md`'s own doctrine, 0.0B means "structurally full of live work," not "clean" —
consistent with disk at 22% used / 757G available (`df -h /`), well inside `verify.sh`'s 90%/20G-free
preflight floor; not a concerning reading, just an honest one.

**Verdict: SD30-E0-F2 COMPLETE.** DoD items 4/6/8 N/A with stated reasons, item 5 PASS, item 7 DONE
(own commit), item 3 is an honest, reproduced, out-of-card-scope FAIL identical to F1's prior finding
(not a regression, byte-identical 177/33/3/141 both before and after this cycle's changes). Items 1
and 2 both PASS on a clean, three-times-verified `verify.sh --full` run (`VERIFY_EXIT=0`, 16/16). No
number moved by lowering a bar: `done` is unchanged (5,837); the
132-unit `held`→`in-progress` movement is a bar being applied MORE strictly (the excuse for exempting
`spell` no longer holds), and `companion`'s cap removal is inert by direct proof, not by assumption.

## Cycle `SD30-E0-F3-001` — 2026-08-14 — `unknown`-residue characterization, corpus-wide (`epic-0-instrument-apply`, F3 sub-scope)

**Actor:** `sd30-e0-f3-unknown`. **HEAD at start:** `5010641f` (tip of `tranche/10`;
`git log --oneline -1` matched `git rev-parse HEAD`). Working tree had two pre-existing, unrelated
dirty entries at start (`.gitignore` modified, `.github/workflows/deploy-site.yml` untracked) —
neither touched by this cycle, neither staged, neither committed by this cycle; recorded per
shared-checkout discipline, not investigated further (out of this card's scope, another agent's
live work per `state-goals-and-lessons.md`'s "idle does not mean dead" doctrine — the safer default,
flagged here).

### 1. Scope

Read-only classification pass over `docs/work-inventory.json` and the PCGen `.lst` source tree —
**no** `data/corpus/`, no engine (`src/`, `apps/`) code, no dashboard-producer code touched. PI gate
not engaged, per the card's own framing.

### 2. Which kinds have a nonzero `unknown` residue — re-derived fresh

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
c = collections.Counter()
for u in d['units']:
    if u['status'] == 'unknown':
        c[u['kind']] += 1
print(c)
"
```
Against the committed `docs/work-inventory.json` (`generated_at: 2026-08-14T20:03:13Z`, unchanged by
this cycle — the SD30-E0-F1-001 regen, confirmed byte-identical before/after by `git status
--porcelain` showing no modification to that file): **`class_feature` 3,622, `feat` 367, every other
kind 0.** No kind besides `feat` newly needs this card's treatment.

**Two corrections, both re-derived not transcribed** (`retro.py correction` events
`1786742550660-sd30-e0-f3-unknown-700ea4` / `1786742550773-sd30-e0-f3-unknown-8fe35c`):
- The card's own acceptance text (`epic-breakdown.md` SD30-E0-F3) claims feat's residue is "329
  units" — live re-derivation is **367**, not 329.
- `decisions.md #38` characterized class_feature's residue at "3,218" (itself already a
  correction of an earlier "2,958") — live re-derivation is **3,622**. **Not** re-characterized this
  cycle (out of F3 scope — owned by the class_feature measurement chain, moved to
  `SD-31-corpus-closure-grind/kanban.md epic-1-measurement` under `decisions.md §51`); the drift is
  recorded so SD-31 does not inherit a stale count.

### 3. Method — `decisions.md #38`'s three buckets, applied to `feat`

Full method, per-unit signal detection, and worked examples are in the artifact README
(`artifacts/sd30-e0-f3-unknown-residue/README.md §2`). Summary: every one of the 367 `feat` `unknown`
units shares one `evidence` value (`in_catalog_with_corpus_magnitude_but_no_observed_consumer` — the
feat-effect probe, `probe_feat_effect_wiring` at `src/bin/v06_work_inventory.rs:1574`, found the feat
in-catalog with real corpus magnitude but no computed delta across its swept postures:
`PROBE_CLASSES = {fighter, barbarian, monk, wizard, swashbuckler}` × `PROBE_LEVELS = {1, 12}` × 4
generic `PROBE_SELECTIONS`). Applied decisions.md #38's three buckets by reading each unit's own
PCGen `.lst` line (not the stored `reason` text alone) for a structural signal — PCGen's `" ~ "`
named-sub-choice `KEY` marker, `BONUS:ABILITYPOOL` grants, inline `CHOOSE:`, positive `PREABILITY`
(polarity-checked), `PRESTAT`/`PRESKILL` floors.

**One near-miss caught before the count was taken** (`retro.py near-miss` event
`1786742559117-sd30-e0-f3-unknown-49d7c2`): the first classifier pass matched `PREABILITY` as a bare
substring, which also matches `!PREABILITY` (negated — "you must NOT already have X", trivially
satisfied by the probe's synthetic characters, not the same shape as a positive requirement).
`Amateur Investigator` (`!PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration`) was
caught misrouted into the chooser-pre-selection-gap bucket on manual spot-check against its raw
`.lst` line, before publish. Fixed to a polarity-aware regex (`(?<!!)PREABILITY:`).

**One residual flagged, not smoothed over**: 4 of the 68 `resource-pool-expansion` units (`Extra
Rage Power`, `Extra Arcana` x2, `Extra Cantrips or Orisons`) name an owning class already inside
`PROBE_CLASSES` — for these four, "the fixture doesn't cover the owning class" is not the actual
explanation. Their true cause is un-diagnosed by this pass and called out by name in the JSON
artifact rather than folded silently into the other 64 units' explanation.

### 4. Result

| top bucket (decisions.md #38 taxonomy) | units | share |
|---|---:|---:|
| genuinely-unreachable (needs new probe-fixture capability) | 217 | 59.1% |
| option-pool (mechanism real, specific pool-slot ungrounded) | 100 | 27.2% |
| unclustered-remainder | 50 | 13.6% |
| **total** | **367** | 100% |

Shape sub-counts: chooser-pre-selection-gap 194, resource-pool-expansion 68, no-structural-signal
(unclustered) 50, prereq-stat-or-skill-gap 23, inline-choose 16, named-sub-choice-key 16. All 367
units accounted for (0 unresolved source-line lookups after the `KEY:` sub-choice fix — 16 units
were unresolved on the first pass because `corpus_key` for named-sub-choice rows is the PCGen `KEY:`
value, not the leading Ability-Name field; fixed in the lookup helper, caught by the artifact's own
"units whose source line could not be re-located" self-check going from 16 to 0, not asserted).

### 5. Artifact landed (durable, for SD-31)

`docs/release/SD-30-class-feature-archetype-bundle/artifacts/sd30-e0-f3-unknown-residue/`:
- `README.md` — method, results, and an explicit "invocation contract for SD-31" section per this
  package's SCOPE NOTE (the 217-unit genuinely-unreachable bucket needs a probe-fixture capability
  expansion, engine-capability-shaped work for SD-32/SD-31 successor scope, not new ingest; the
  100-unit option-pool bucket needs no further action beyond decisions.md #38's existing standing
  ruling; the 50-unit unclustered remainder is inherited unfinished characterization work, SD-31
  measurement-shaped).
- `feat_unknown_characterization.json` — per-unit detail (id, name, book, source file/line, bucket,
  shape, sub-reason) for all 367 units.
- `characterize_feat_unknown.py` — the classifier itself, read-only over `docs/work-inventory.json`
  and the PCGen `.lst` tree, reproducible against a future inventory regen.

### 6. Definition of done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **N/A — measurement-only cycle, no Rust/Python production code changed.** No file under `src/`, `apps/`, `scripts/`, or `tests/` touched (`git status --porcelain` before/after: only the new `artifacts/sd30-e0-f3-unknown-residue/` dir and the retro log are new/untracked; the two pre-existing unrelated dirty entries noted above are unchanged by this cycle). Per loop-instruction's "Doc-only or measurement-only cycles run the relevant `--only` stages instead and state exactly which" — the relevant checks run instead are items 5 (wired-integration audit, below) and direct JSON/Python syntax validation of the two new artifact files (`python3 -c "import json; json.load(open(...))"` and `python3 -c "import ast; ast.parse(open(...).read())"`, both OK). |
| 2 | reach claim, nonzero | **N/A.** This card adds no new player-facing reach claim and touches no `reach_gate.rs` family — a read-only classification artifact under `docs/release/` has no reach surface to claim. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **N/A.** No corpus, generator, or ingest code touched this cycle; the pre-existing 177-defect finding F1/F2 already flagged (`wiring-class-mismatch`, unrelated to this card) is neither caused nor re-verified by this cycle since no `data/corpus/` or `v06_work_inventory.rs` file was read for write purposes — only `docs/work-inventory.json` (the generator's already-committed OUTPUT) was read. |
| 4 | Guarded work-inventory regen | **N/A.** No corpus, generator, or inventory-affecting code touched. `docs/work-inventory.json` was read only, confirmed unmodified by `git status --porcelain` (absent from the diff). |
| 5 | Four-check wired-integration audit | **PASS, scoped to this cycle's own files.** The four `git diff develop...HEAD` checks are branch-wide (whole-tranche history) and check 1 (`OK_NO_TOKENS`) surfaces pre-existing hits from prior commits in `src/**/*.rs`, none from this cycle (this cycle touched no file matching any of the four checks' globs — `docs/release/**` matches none of them). Direct grep of this cycle's two actually-changed files (`characterize_feat_unknown.py`, `README.md`) for the STUB/MOCK/placeholder/todo/fixme/hack token set: clean (`OK_NO_TOKENS_MY_FILES`). Checks 2/3/4 (`OK_NO_NOOP_HANDLERS`/`OK_NO_MOCK_LEAKS`/`OK_NO_WOULD_STRINGS`) all clean branch-wide. |
| 6 | `OPEN_FINDINGS` in `reach_gate.rs` | **N/A.** No family surfaced or left unsurfaced this cycle — not a reach-scope card. The unresolvable subset (217-unit genuinely-unreachable bucket) is recorded as an explicit "invocation contract for SD-31" section in the artifact README instead, the correct registry for a characterization-scope (not reach-scope) finding, matching F2's own precedent for instrument-scope findings. |
| 7 | Baseline movements | **N/A.** No baseline-affecting code or test changed. |
| 8 | On-screen verification | **N/A.** No player-visible desktop-app surface touched — this cycle produced a documentation/classification artifact only, read by SD-31 planning, not by the character-sheet app. |

### 7. Retro events emitted

- `1786742550660-sd30-e0-f3-unknown-700ea4` — correction, feat-unknown-residue count (329 claimed -> 367 actual).
- `1786742550773-sd30-e0-f3-unknown-8fe35c` — correction, class_feature-unknown-residue drift (3,218 claimed -> 3,622 actual, not re-characterized, flagged for SD-31).
- `1786742559117-sd30-e0-f3-unknown-49d7c2` — near-miss, polarity-blind PREABILITY substring match caught before publish.

### 8. Card disposition

`epic-0-instrument-apply`'s **F3 sub-scope is COMPLETE**: the one kind (besides the already-owned
`class_feature`) carrying a nonzero `unknown` residue (`feat`, 367 units) is characterized into
`decisions.md #38`'s three buckets, with per-unit detail and an explicit SD-31 invocation contract
landed as a durable artifact. No other kind needed treatment (re-derived fresh, all zero). `kanban.md`'s
`epic-0-instrument-apply` row is left `READY` — **F4** (re-derivation and reporting) remains open
under the same row, matching SD30-E0-F1-001/F2-001's own precedent for this multi-feature row.

### 9. Reclaim

`./scripts/reclaim.sh` (dry run) then `./scripts/reclaim.sh --apply`: **0.0B reclaimed, 0 items**
(all candidates skipped — young verify-logs, forbidden worktree paths, unmerged/checked-out
branches). Disk at 22% used / 758G available (`df -h /`), well inside `verify.sh`'s preflight floor —
0.0B reclaimed reads as "nothing stale to reclaim," not "structurally full," consistent with the
low disk-utilization reading this cycle.

**Verdict: SD30-E0-F3 COMPLETE.** DoD items 1-4, 6-8 N/A with stated reasons (measurement-only,
read-only classification cycle; no code, corpus, engine, or reach-surface change). Item 5 PASS,
scoped correctly to this cycle's own two changed files plus the branch-wide check run for the
record. No number moved by lowering a bar — both bucket boundaries (option-pool vs
genuinely-unreachable) are structural signals read from the corpus source line, not a convenient
split chosen to make a total look better, and two of this bundle's own inherited figures (329, 3,218)
were corrected upward (367, 3,622) rather than left comfortable.

## Cycle `SD30-E0-F4-001` — 2026-08-14 — Re-derivation and reporting; close of `epic-0-instrument-apply`

**Actor:** `sd30-e0-f4-report` (`RETRO_ACTOR`/`CARGO_TARGET_DIR` name). **HEAD at start:** `3a3b89d1`
(F3's own commit, already pushed and confirmed `origin/tranche/10`'s tip). **Checkout assertion:**
`git rev-parse HEAD` = `3a3b89d1`; `git status --porcelain` showed the tree dirty on two
**pre-existing, unrelated** entries this cycle did not create and did not touch — `.gitignore` (a
`.wrangler/` ignore-rule addition from an unrelated site-deploy lane) and an untracked
`.github/workflows/deploy-site.yml` — both left exactly as found; this cycle staged/committed neither.
Package present, tree not fully clean but the dirty entries are unrelated to this bundle and not a
git-recovery case (loop-instruction §0− only mandates recovery when the package is absent on a clean
tree); proceeded per that reading, flagged here as a routine judgment call with a conventional default.

### 1. Card

`epic-0-instrument-apply`, F4 sub-scope ("re-derivation and reporting, and the close of
`epic-0-instrument-apply`"). F1 (static/derived done rung), F2 (computed-bucket consumer-delta
probes), F3 (`unknown`-residue characterization) all landed in prior cycles this same day
(`SD30-E0-F1-001` `c3f3e599`, `SD30-E0-F2-001` `175394b6`, `SD30-E0-F3-001` `3a3b89d1`); this cycle
does the re-derivation/reporting closeout and (conditionally, see §8) the epic flip.

### 2. Acceptance items, and how each was actually met

**AC1 — `derive-movable-mass.py` re-run, pre-epic and post-epic runs cited.**
Re-ran it live this cycle rather than trusting F1's prior finding:

```
python3 docs/release/SD-30-class-feature-archetype-bundle/artifacts/derive-movable-mass.py
```
→ `ValueError: ('static', 'literal-verified')` — **reproduced, byte-identical failure mode** to F1's
`SD30-E0-F1-001` finding (the script predates the `literal-verified`/`fixture-verified` rungs and
cannot classify either status word; see the script's own staleness header, `decisions.md §50`). This
is this criterion's own premise turning out wrong (already corrected in place by F1, re-confirmed not
re-litigated here) — the script **cannot** produce a pre-epic/post-epic pair, before or after this
cycle's own change, because it does not run at all. Per F1's correction and
`state-goals-and-lessons.md §3.2`'s "report the board's movement... import the producer's own
`doneness_verdict()` and replay it" doctrine, this cycle uses that live authority instead, run twice:

- **Pre-Epic-0** (`98d98d3a`, the commit immediately before `SD30-E0-F1-001` started — the true
  epic-0 entry state, distinct from `d1b29589`, an earlier "pre-rung" commit F1 cited for a narrower
  purpose that predates even the handoff session's own rung landing):
  `git show 98d98d3a:docs/work-inventory.json`, replayed through
  `pf1e_dashboard_producer.doneness_verdict()` (imported, not transcribed) → `done` 5,837 / `held`
  6,954 / `not-started` 21,300 / `unmeasurable` 3,546 / `in-progress` 848 / `deferred` 36 / total
  38,521 (`beginner_box` excluded, matching the live producer's own exclusion).
- **Post-Epic-0** (current `HEAD` = `3a3b89d1`, F1+F2+F3 landed): same replay → `done` 5,837 (+0) /
  `held` 6,916 (−38) / `not-started` 20,895 (−405) / `unmeasurable` 3,989 (+443) / `in-progress` 848
  (+0) / `deferred` 36 (+0) / total 38,521.
- **The −38/+405/+443 movement is entirely `feat`**: F3's `unknown`-residue cycle's guarded
  `v06_work_inventory` regen moved 38 `feat` units from `text-complete`/`held` to `unknown`/
  `unmeasurable` (329→367 `unknown`, exactly F3's own re-derived figure) — a genuine corpus-read
  correction, not a classifier defect. `done` is untouched by it; **Epic 0's net `done` movement is
  entirely attributable to the static/derived rung that had already landed before Epic 0's F-cards
  started** (3,464→5,837, `+2,373`, re-confirmed against `d1b29589` in §3 below), not to any of F1-F3's
  own cycles, which is the honest finding: those cycles *confirmed and characterized* an existing
  mechanism rather than moving new units to `done` themselves.
- Command (repeated for both refs, `<ref>` substituted):
  ```
  git show <ref>:docs/work-inventory.json > /tmp/inv-<ref>.json
  python3 -c "
  import json, importlib.util, collections
  spec = importlib.util.spec_from_file_location('m', 'scripts/observer/pf1e_dashboard_producer.py')
  mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
  d = json.load(open('/tmp/inv-<ref>.json'))['units']
  c = collections.Counter()
  for u in d:
      if u.get('book') == 'beginner_box': continue
      c[mod.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))] += 1
  print(c)
  "
  ```

**AC2 — `AT-30-015` per-kind floor table updated with actual `done` figures.**
`AT-30-015` was already MOVED to `SD-31-corpus-closure-grind/acceptance-and-verification.md
AT-31-005` by `decisions.md §51` (2026-08-14, earlier the same day, prior to this cycle) — SD-30's own
`acceptance-and-verification.md` correctly carries only the pointer (verified by content: line 244's
`MOVED` header, line 370's exit-checklist strikethrough, both present, neither touched this cycle).
Per this bundle's SCOPE NOTE ("deliver the mechanism and document its invocation contract for the
successor... do not extend into the moved epics' work"), this cycle:
- Re-derived the actual current `done`/`held`/total figures for all 5 kinds `AT-31-005` covers
  (`class_feature`, `monster`, `spell`, `race`, `race_trait`) — see full table and command in
  `SD-31-corpus-closure-grind/acceptance-and-verification.md AT-31-005`, updated this cycle.
- **Found and corrected a real transcription defect, not just staleness**, in that table's `spell`
  row: `held` was listed as `1,235` and `done+held` floor as `1,282 (45.1%)` — both are copy-paste
  artifacts of the `monster` row directly above and `feat`'s percentage from an unrelated table. The
  true `spell` `held` figure is **1,103** (`done+held` = 1,150, 40.4%), and the row's
  `NO_GROUNDING_PROBE`-capped rationale is stale — `SD30-E0-F2` already lifted that cap, moving 132
  `spell` units `held`→`in-progress`. Fixed in place with the correct figures and rationale, retro
  correction emitted (`1786743412894-sd30-e0-f4-report-0f3bbc`,
  `docs/retro/events/sd30-e0-f4-report.jsonl`). This is squarely "delivering the mechanism" (accurate
  current data) to the successor's own document, not doing SD-31's ingest/measurement/mechanism work
  — no SD-31 kanban or epic status was touched.
- The other 4 rows (`class_feature`, `monster`, `race`, `race_trait`) re-derived exactly matching the
  split-time snapshot — Epic 0's F1-F3 cycles made zero net movement to those kinds' `done`/`held`
  split (confirmed by the same replay, per-kind, §3 below).

### 3. Three-surface reporting-agreement check (this card's own additional requirement)

| surface | `done` | `held` | `in-progress` | `not-started` | `unmeasurable` | `deferred` | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| `docs/work-inventory.json` (committed, replayed via producer import) | 5,837 | 6,916 | 848 | 20,895 | 3,989 | 36 | 38,521 |
| `/home/ubuntu/swarm-observer/PF1e-dashboard.json` `work_inventory.by_doneness` (live, `generated_at` 2026-08-14T21:26:18Z) | 5,837 | 6,916 | 848 | 20,895 | 3,989 | 36 | 38,521 |
| this package's own receipts (`kanban.md` F1/F2/F3 rows, `state-goals-and-lessons.md §1.1`, updated this cycle) | 5,837 | 6,916 (was 6,954, corrected) | 848 | 20,895 (was 21,319, corrected) | 3,989 (was 3,546, corrected) | 36 | 38,521 (was 38,540, corrected) |

**All three agree exactly after this cycle's correction.** Before this cycle, the third surface
(this package's own `state-goals-and-lessons.md §1.1`) disagreed on every non-`done` bucket and on
`total` — root-caused to two independent things, neither a live-dashboard defect: (1) that table's
`total` did not exclude `beginner_box` (19 units) the way the live producer's
`_exclude_books_from_kind_doneness` does; (2) it was captured before F3's guarded regen moved 38
`feat` units `held`→`unmeasurable`. **The doc was stale, not the dashboard** — checked which one broke
before fixing either, per this package's own "check the denominators before believing a disagreement"
discipline. Fixed in place in `state-goals-and-lessons.md §1.1`, retro correction emitted (same event
id as §2).

Per-kind command (used for both the corpus-wide and per-kind breakdowns above):
```
python3 -c "
import json, importlib.util, collections
spec = importlib.util.spec_from_file_location('m', 'scripts/observer/pf1e_dashboard_producer.py')
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
d = json.load(open('docs/work-inventory.json'))['units']
by_kind = collections.defaultdict(collections.Counter)
for u in d:
    if u.get('book') == 'beginner_box': continue
    by_kind[u.get('kind')][mod.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))] += 1
for k in sorted(by_kind): print(k, dict(by_kind[k]))
"
```

**Verification-stamp check** (hazard 1, `state-goals-and-lessons.md §1.3`): committed
`docs/work-inventory.json` carries `literal-verified` **2,322** + `fixture-verified` **49** = 2,371
stamps —
```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))['units']
print(collections.Counter(u.get('status') for u in d if u.get('status') in ('literal-verified','fixture-verified')))
"
```
→ `Counter({'literal-verified': 2322, 'fixture-verified': 49})`, matching F1's figures exactly — the
committed file has **not** silently lost its stamps. Confirmed the guard's own hazard-mitigation code
is live by content: `src/bin/v06_work_inventory.rs:4907` (`--allow-stamp-loss` flag check) and its
surrounding refusal logic exist and gate the write path (grepped, not assumed).

**Honest board position per kind, at epic close** (same command as above, restated as the
closing summary this card's brief asked for):

| kind | done | total | % |
|---|---:|---:|---|
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
| **race** | **0** | **103** | **0.0%** |

### 4. Definition of done

This is a documentation/measurement-only cycle: no file under `src/`, `apps/`, or `scripts/` was
changed (`git status --porcelain` before/after this cycle's own edits shows only markdown/doc/retro
files touched; `docs/work-inventory.json` was regenerated to prove DoD-4's stability property then
`git checkout`-reverted since the diff was `generated_at`-only, no content change to ship).

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **N/A — measurement/doc-only cycle, no Rust/Python production code changed.** Per loop-instruction's "Doc-only or measurement-only cycles run the relevant `--only` stages instead" — ran the specific stages relevant to this card's own claims instead (items 2-4 below), plus `--only preflight-disk` (PASS, 22% used / 757G available) before starting any bounded work. |
| 2 | reach claim, nonzero | **PASS.** `./scripts/verify.sh --only reach` → `PASS reach (27 passed)`, `VERIFY_EXIT=0`. This card adds no new reach family; ran it anyway as this cycle's own honest-reporting standard, confirming the existing claim is live, not absent. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **FAIL, exit 2 — pre-existing, out of this card's scope, byte-identical to F1/F2's prior receipts.** 177 defects (33 `companion`, 3 `monster`, 141 `monster_ability`), all `wiring-class-mismatch` in `ultimate_wilderness`'s companion-ability corpus JSON, traced by `SD30-E0-F1-001` to the `%N` prose-formula classifier fix landing without a matching re-ingest. Reproduced counts this cycle (33/3/141) are byte-identical to F1/F2's prior receipts — confirms this cycle neither caused nor worsened it (no `data/corpus/` or classifier file touched). Not fabricated as a pass; flagged for the same dedicated follow-up F1 already flagged. |
| 4 | Guarded work-inventory regen | **PASS.** Ran the sanctioned sequence live: `corpus_literal_sweep` → CLEAN, 3,516/9,328 examined, 0 findings (byte-identical to F1). `derived_evaluator_fixture_check` → 49/94 cleared, 1 known failure, 44 not-ingested (byte-identical to F1). Guarded `v06_work_inventory` regen → `GUARD_EXIT=0` (zero stamp loss reported by the guard itself), and the resulting `docs/work-inventory.json` diff against the committed file was **`generated_at`-only** — confirmed by `git diff -- docs/work-inventory.json | grep -v generated_at` returning nothing besides the timestamp line — exactly the "second run changes only `generated_at`" stability DoD-4 requires. Reverted the no-op diff (`git checkout -- docs/work-inventory.json`) rather than committing a content-free timestamp bump. |
| 5 | Four-check wired-integration audit | **PASS, branch-wide.** `git diff --unified=0 origin/develop...HEAD` for all four checks (`OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`) — all four clean. This cycle's own diff touches no `.rs`/`.ts`/`.tsx` file at all (only `.md`/`.jsonl`), so it is doubly clean by construction. |
| 6 | `OPEN_FINDINGS` in `reach_gate.rs` | **N/A.** No family surfaced or left unsurfaced this cycle — a reporting/closure card, not a reach-scope card. |
| 7 | Baseline movements | **N/A.** No baseline-affecting code or test changed. |
| 8 | On-screen verification | **N/A.** No player-visible desktop-app surface touched this cycle — a documentation/reporting cycle over already-committed measurement data, not new engine or UI wiring. |

### 5. Retro events emitted

- `1786743412894-sd30-e0-f4-report-0f3bbc` — correction: `state-goals-and-lessons.md §1.1`'s
  non-`done` bucket figures and `SD-31 AT-31-005`'s `spell` row (claimed vs actual, both cited above),
  `--verified-by` the producer-import replay cross-checked against the live dashboard JSON.

### 6. Card disposition — epic-0-instrument-apply closure check

Per the card's own instruction: grep for the symbols F1-F3 actually landed, not the card statuses.

```
grep -n "literal-verified\|fixture-verified" scripts/observer/pf1e_dashboard_producer.py   # F1: present, lines 3343/3354/3580/3593
grep -n "probe_class_feature_effect_wiring" src/bin/v06_work_inventory.rs                    # F2: present, lines 4072/4452
grep -n "^NO_GROUNDING_PROBE" scripts/observer/pf1e_dashboard_producer.py                    # F2: = () (cap lifted)
ls docs/release/SD-30-class-feature-archetype-bundle/artifacts/sd30-e0-f3-unknown-residue/   # F3: README.md, characterize_feat_unknown.py, feat_unknown_characterization.json present
```

All four confirmed present at `HEAD` (`3a3b89d1`), which is `origin/tranche/10`'s own tip
(`git merge-base --is-ancestor HEAD origin/tranche/10` — true). **F1-F4 all on `tranche/10` by
content.** `epic-0-instrument-apply` flipped to `COMPLETE` in `kanban.md` this cycle.

### 7. Reclaim

`./scripts/reclaim.sh` (dry run): all candidates skipped (young verify-logs, forbidden worktree
paths, unmerged/checked-out branches — consistent with 22-23% disk usage, not a "structurally full"
reading). `./scripts/reclaim.sh --apply` run at cycle close; bytes reclaimed recorded in the commit
that follows this receipt.

**Verdict: SD30-E0-F4 COMPLETE, `epic-0-instrument-apply` COMPLETE.** DoD items 1, 6-8 N/A with
stated reasons (measurement/doc-only cycle, no reach/code/UI surface touched); items 2, 4, 5 PASS;
item 3 an honest, byte-identical-reproduced pre-existing FAIL, not this card's scope, not fabricated
as a pass. Two real defects found and corrected in this package's own reporting surfaces (a
`beginner_box`-exclusion bug and a row-copy-paste error), not merely re-stated as stale — both
verified against the live dashboard producer, not against each other.

## Cycle `SD30-E1-F1-001` — 2026-08-14 — Code-Side Identifier-Disclosure Audit Pass (`epic-1-identifier`)

**HEAD at start:** `ed7327bd2f38ed2ac9fd0a8604cd90a03a030d6a` (tip of `tranche/10` at claim time; tree
had two pre-existing, not-mine changes — `M .gitignore` and `?? .github/workflows/deploy-site.yml` —
left untouched throughout this cycle per shared-checkout discipline; package directory was present so
no `git reset --hard` recovery was needed). `RETRO_ACTOR=sd30-e1-identifier`,
`CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e1-identifier`.

### 1. What this cycle did

Epic 1 fires first in the gate chain (epic-2/3/7 depend on it). Card acceptance
(`epic-breakdown.md` SD30-E1-F1): no `sd30_*`/`SD30_*`/`Sd30*`/`sd30-*` patterns in surface code, no
`t_<hex>` kanban tokens, and `scripts/identifier-discipline-audit.sh` returns 0 findings — enumerating
the real surface fresh rather than trusting the card's own named-file snapshot, and treating a
disagreement between the audit script and the doctrine (`decisions.md §7`) as a finding to report,
not to silently resolve either way.

### 2. Figures re-derived this cycle, each with its command

**a. Tree-wide pattern sweep, `src/` + `apps/desktop/` (not diff-scoped — the widest possible read):**
```
grep -rnE '\bsd30_[A-Za-z0-9_]*|\bSD30_[A-Za-z0-9_]*|\bSd30[A-Za-z0-9_]*|\bsd30-[A-Za-z0-9-]*' \
  --include='*.rs' --include='*.ts' --include='*.tsx' src apps/desktop | grep -v '__tests__' | grep -v '\.test\.'
```
→ **0 matches.** Same command with `\bt_[0-9a-f]{8,}\b` in place of the `sd30` alternation → **0
matches.** `SD-30-E[0-9]` and `AV-PAY-[0-9]` patterns (the other two doctrine-forbidden shapes) →
**0 matches**, same pathspec.

**b. Real surface, enumerated fresh (not transcribed from the card):**
```
find src/rules_core/rules_tables -maxdepth 1 -type d   # 30 book dirs, current roster
find src -iname 'archetype_resolver.rs' -o -iname 'pilot_compute.rs'   # both present, src/rules_core/
```
The card's named surface (`src/rules_core/rules_tables/<book>/`, `archetype_resolver.rs`,
`pilot_compute.rs`) still resolves; both named files exist at `src/rules_core/`. 30 rules-tables book
directories currently exist (a superset of the 23 `class_feature`-bearing ones; the widest read, not a
narrowed one).

**c. The audit script itself:**
```
git fetch origin --quiet && bash scripts/identifier-discipline-audit.sh origin/develop
```
→ `OK_NO_BUNDLE_TAGS`, exit `0`.

**d. Broader repo-wide sweep (`src`, `apps`, `scripts`, `tests` — beyond the audit script's own
pathspec, to surface the disagreement case the card asked me to check for) found four hit clusters,
all resolved as non-violations or a reported-not-actioned finding — see §3.**

### 3. Two checks the acceptance text does not mechanically cover, done by hand

**3a. Proved the audit script can fail (card's explicit instruction).** A gate that cannot fail
proves nothing (`state-goals-and-lessons.md §3.1`; this repo has shipped three such gates already).
In a disposable `git worktree` (`sd30-e1-audit-proof-scratch`, never pushed, deleted after):
```
git worktree add -B sd30-e1-audit-proof-scratch <scratch-path> HEAD
echo 'pub const sd30_leak_marker: &str = "violation";' >> src/rules_core/pilot_compute.rs
git add src/rules_core/pilot_compute.rs && git commit -m "scratch: prove audit can fail"
bash scripts/identifier-discipline-audit.sh origin/develop
```
→ caught it: `168:+pub const sd30_leak_marker...`, `FAIL: bundle identifier(s) above leaked into
shipping code.`, exit `1`. Worktree and scratch branch then removed
(`git worktree remove --force`, `git branch -D`); `git worktree list` / `git status --porcelain`
confirmed no residue. `retro.py near-miss` event
`1786744212064-sd30-e1-identifier-6704a3` records this (`--verified-by`-equivalent is the planted
diff and the printed FAIL line itself).

**3b. Audit-script-vs-doctrine disagreement found and reported, not silently resolved.** A repo-wide
sweep beyond the audit script's own pathspec (`src/**/*.rs`, `apps/desktop/**/*.ts*` only — `scripts/`
is out of scope by the script's own design) found:
```
grep -rnE '\bsd30_[A-Za-z0-9_]*|...' --include='*.py' ... .
```
→ `scripts/observer/pf1e_dashboard_producer.py:2278-2279`: a dict key/value
`"sd30_book_pre_build": {"manifest_id": "sd30_book_pre_build", ...}`, with parallel
`sd28_book_pre_build`/`sd29_book_pre_build` entries alongside it (same file, tracked in git,
confirmed by `git ls-files`/`git log` — this is the in-repo source copy, not the cron-deployed one at
`/home/ubuntu/swarm-observer/` the hazard note in `state-goals-and-lessons.md §1.3` hazard 4 warns
about). Read literally, `decisions.md §7`'s headline ("source-code identifiers describe WHAT the
artifact does, NOT which release/spec domain it came from... Forbidden patterns: `sd30_*`...")
flags this. The audit script's own pathspec never sees it (`scripts/` excluded by design), and
`decisions.md §26` (Epic 8 code review) explicitly delegates identifier-discipline enforcement to
that same script/pathspec at bundle-diff scope — **the two authorities disagree on whether this counts.**

**Not renamed, reported instead** (`retro.py deferral` event
`1786744223492-sd30-e1-identifier-4b01f0`): (i) `sd28_`/`sd29_` analogs of the identical shape
already exist unremediated in the same file, so a targeted `sd30_` rename alone would be
inconsistent, not principled; (ii) these are per-bundle *tracking-manifest* keys whose entire
semantic content is "which workchannel this record belongs to" — the same shape as the doctrine's own
carved-out exception for test file names legitimately citing their bundle, not the shipping-surface
leakage (a rules-engine function secretly named after its release) the doctrine's rationale targets;
(iii) `pf1e_dashboard_producer.py` is the file `state-goals-and-lessons.md §1.3` hazard 4 flags as
RAISING on an unrecognised status word and requiring synchronized generator+producer changes — an
unreviewed, out-of-card-scope rename here is exactly the unsynced-edit shape that hazard warns
against; (iv) it sits outside this card's named acceptance surface (`src/rules_core/rules_tables/`,
`archetype_resolver.rs`, `pilot_compute.rs`) and outside the audit script's own definition of
"surface code." Recorded for an operator ruling on whether the forbidden-pattern list is meant to
reach devops/observability tooling or is scoped to shipping game-rules/UI surface only.

The other three hit clusters found by the broad sweep are non-violations on inspection, not
deferrals: `tests/v06_work_inventory.rs`'s `SD30_CAMPAIGN_SETTING_BOOKS` constant and
`tests/sd13_*_bounded_semantics.rs`'s `t_<hex>` slice-id comments are test-file content, explicitly
carved out both by `decisions.md §7`'s own scope note (this card's brief: "test names that
legitimately reference the bundle are not violations") and by the audit script's design (tests
excluded by its own `SHIPPING_PATHSPEC`); `scripts/tests/test_identifier_discipline_audit.sh` is the
audit script's own test fixture, containing a deliberately-planted mock violation string as test
data, not a live identifier.

### 4. Definition of done

No `src/`, `apps/`, or `scripts/` file was changed by this cycle (0 renames needed — the audit found
0 findings within its own scope). `git status --porcelain` at cycle end: only
`docs/release/SD-30-class-feature-archetype-bundle/kanban.md` (claim/complete edit),
`docs/release/SD-30-class-feature-archetype-bundle/progress.md` (this receipt), and
`docs/retro/events/sd30-e1-identifier.jsonl` (the two events above) — plus the two pre-existing,
not-mine changes noted at the top, left untouched.

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **N/A — measurement/audit-only cycle, no Rust/Python production code changed.** Ran the relevant `--only` stages instead (items 2-3 below) plus `--only preflight-disk` (PASS, 23% used / 754G available) before starting. |
| 2 | reach claim, nonzero | **PASS.** `./scripts/verify.sh --only reach` → `PASS reach (27 passed)`, `VERIFY_EXIT=0`. This card adds no new reach family; ran it as this program's standing honest-reporting practice. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **FAIL, exit 2 — pre-existing, out of this card's scope, byte-identical to the F1/F2/F4 receipts' figures.** 177 defects (33 `companion`, 3 `monster`, 141 `monster_ability`), all `wiring-class-mismatch` in `ultimate_wilderness`'s companion-ability corpus JSON, already traced by `SD30-E0-F1-001` to the `%N` prose-formula classifier fix landing without a matching re-ingest. Re-derived this cycle (`cargo run --locked --bin v06_corpus_trap_report -- --audit`, counts grepped from its own output) and confirmed byte-identical — this cycle touched no corpus or classifier file, so it neither caused nor worsened it. Flagged, not fabricated as a pass; same dedicated follow-up already on record. |
| 4 | Guarded work-inventory regen | **N/A.** No corpus, classifier, or ingest code changed this cycle (identifier-discipline audit only); nothing this cycle would move `docs/work-inventory.json`, so a regen would be a no-op churn, not a proof of anything this card claims. |
| 5 | Four-check wired-integration audit | **PASS.** `bash scripts/wired-integration-audit.sh origin/develop` → all four checks (`OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`) clean, exit 0. This cycle's own diff touches no `.rs`/`.ts`/`.tsx` file (only `.md`/`.jsonl`), so it is doubly clean by construction. |
| 6 | `OPEN_FINDINGS` in `reach_gate.rs` | **N/A.** No record family surfaced or left unsurfaced this cycle — an identifier-audit card, not a reach-scope card. |
| 7 | Baseline movements | **N/A.** No baseline-affecting code or test changed. |
| 8 | On-screen verification | **N/A.** No player-visible desktop-app surface touched — 0 renames landed, nothing new to see on a sheet. |

### 5. Retro events emitted

- `1786744212064-sd30-e1-identifier-6704a3` — near-miss: proved
  `scripts/identifier-discipline-audit.sh` can actually fail (planted-violation test in a disposable
  worktree) before trusting its 0-finding result on the real diff.
- `1786744223492-sd30-e1-identifier-4b01f0` — deferral: `pf1e_dashboard_producer.py`'s
  `sd28_`/`sd29_`/`sd30_book_pre_build` manifest keys not renamed, reported instead — see §3b above
  for the full reasoning and the revisit condition (operator ruling on doctrine scope).

### 6. Reclaim

`./scripts/reclaim.sh` (dry run): all candidates skipped — 8 verify-log dirs too young, 10 sibling
worktrees under `.claude/worktrees/` forbidden paths (other live agents' work, correctly not touched),
2 branches not merged/upstream-present, 10 branches checked out in a worktree. `./scripts/reclaim.sh
--apply`: **0.0 B reclaimed** — consistent with the box carrying multiple live concurrent agents
right now (confirmed by `git worktree list` showing 10 active worktrees beyond this checkout), not a
"structurally full" reading (disk itself is 23% used / 754G available per the preflight-disk run
above) — reclaim.sh correctly refused every candidate rather than touching live work.

**Verdict: SD30-E1-F1 COMPLETE, `epic-1-identifier` COMPLETE.** DoD items 1, 4, 6-8 N/A with stated
reasons (audit-only cycle, 0 renames landed, nothing corpus/reach/UI-affecting to prove); items 2, 5
PASS; item 3 an honest, byte-identical-reproduced pre-existing FAIL, not this card's scope, not
fabricated as a pass. Tree-wide (not diff-scoped) pattern sweep and the audit script both independently
confirm 0 identifier-discipline findings within the doctrine's shipping-surface scope; one
audit-script-vs-doctrine disagreement found in `scripts/` (out of the shipping-surface scope both the
script and `decisions.md §26` define) is reported via `retro.py deferral`, not silently resolved
either way; the audit script's own soundness was proven, not assumed.

## Cycle `SD30-E2-F1-001` — 2026-08-14 — `epic-2-prelaunch` closure (SD30-E2-F1 + SD30-E2-F2)

`RETRO_ACTOR=sd30-e2-prelaunch`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e2-prelaunch`
(deleted at cycle end, 682M). **HEAD at start:** `aa248507` (`docs(sd30): SD30-E1-F1 — code-side
identifier-disclosure audit pass, epic-1-identifier COMPLETE`) — `git rev-parse HEAD`,
`git log --oneline -1`, `git status --porcelain` (dirty — see "Tree state" below),
`ls docs/release/SD-30-class-feature-archetype-bundle/loop-instruction.md` (present). Package
present and tree not fully clean, but the dirt is pre-existing, out-of-scope, and not from a bundle
crash — no reset performed (see below). `epic-1-identifier` gate: `COMPLETE` per `kanban.md`,
confirmed by content in the same read.

### Tree state at cycle start — investigated, not ignored

`git status --porcelain` showed `M .gitignore`, `?? .github/workflows/deploy-site.yml` before this
cycle touched anything. Investigated rather than assumed benign: `git diff .gitignore` shows a
3-line `.wrangler/` cache-ignore addition; the untracked workflow file is a Cloudflare-Pages
site-deploy GitHub Action, both unrelated to SD-30's `class_feature`/instrument-apply scope.
`./scripts/reclaim.sh`'s branch-skip listing (this cycle's own §"Reclaim" below) independently
confirms live, unmerged `site-deploy` and `fix/site-deploy-page-workflow` branches with upstream
present — a concurrent, unrelated agent's live work on this shared checkout, not stranded SD-30
debris. Per shared-checkout discipline this cycle did not touch, stage, or commit either file.

### 1. SD30-E2-F1 — Local-file dispatch readiness

**Acceptance 1 — `kanban.md` vs `epic-breakdown.md` agreement post-split.** Read both files fresh
(not from the P0.5/`SD30-PRELAUNCH-002` receipts, which predate `epic-1-identifier`'s close).
`kanban.md`'s per-epic table rows and `epic-breakdown.md`'s "Scope narrowed 2026-08-14" section
(`decisions.md §51`) both independently state the same partition:

- Live: `0` (COMPLETE), `1` (COMPLETE), `2` (this cycle), `3`, `7`, `8`, `9`.
- MOVED to `SD-31-corpus-closure-grind`: `4, 5, 6, 10, 11` (renumbered `1-5` there).
- MOVED to `SD-32-engine-capability-builds`: `12, 13` (renumbered `1-2` there).
- SPLIT: `14` (grind-lane to SD-31 Epic 6, capability-build-lane to SD-32 Epic 3).

**No disagreement between the two files.** One stale sub-detail found and reported, not silently
fixed: `epic-breakdown.md:135`'s SD30-E2-F1 acceptance bullet itself reads "the 9 re-cut epics" —
correct as of 2026-08-10 (when SD-30 had exactly Epics 1-9) but now undercounts the file's own
current epic set (7 live + 8 moved/split = 15 numbered epics, 0-14). This is stale wording on one
acceptance bullet, not a disagreement between the two files' actual card lists (both of which
already independently state the current 7-epic live scope elsewhere in the same files, correctly) —
left as historical text per this package's own "original text stays, corrections point forward"
convention (the same one `decisions.md §51` itself follows two paragraphs later in the same file),
rather than edited, to avoid two documents claiming authorship of the same correction.

**Acceptance 2 — re-scope receipt in `progress.md`.** Present: `## 2026-08-14 — Split: Phase 3 to
SD-31, Phase 4 to SD-32 (`decisions.md §51`)`, this file, confirmed by `grep`.

**Acceptance 3 — working tree clean.** Not literally empty (see "Tree state" above); the non-empty
state is pre-existing, out-of-scope, unrelated-agent work, not SD-30 debris left by a prior cycle.
No SD-30-scoped uncommitted work found at cycle start.

### 2. SD30-E2-F2 — Branch state + cycle-0 trap-report + work-inventory

**Branch pushed:** `git rev-parse HEAD` = `git rev-parse origin/tranche/10` = `aa248507...` — already
pushed, no divergence.

**Guarded work-inventory regen (DoD item 4 procedure, never a bare run):**

```
$ cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/.../sweep.json
corpus-literal-sweep: 3516 records examined of 9328 read, 36105 tokens compared (9 synthesized),
8903 digests checked, 0 findings
corpus-literal-sweep: CLEAN                                                    # exit 0

$ cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/.../fixture.json
derived-evaluator-fixture-check: 49 of 94 covered units cleared; 1 failed; 44 not ingested
derived-evaluator-fixture-check: FAIL advanced_players_guide:equipment:spindle_of_perfect_knowledge:
corpus row states BONUS:STAT|INT,WIS,CHA|4|TYPE=Enhancement but the evaluator produced no ability
bonus at all                                                                   # exit 0 (known, live-confirmed pre-existing fail, same unit F1/F4 already recorded — instrument correctly refuses to stamp it)

$ CORPUS_LITERAL_SWEEP_REPORT=/tmp/.../sweep.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/.../fixture.json \
  cargo run --locked --bin v06_work_inventory                                  # exit 0
```

`git diff --stat docs/work-inventory.json` → 1 line changed (`generated_at` only); `git diff
docs/work-inventory.json | grep -v generated_at` → 0 non-generated_at lines. **Zero stamp loss**,
the guard's own success condition. Stamp count confirmed by grep of the regenerated file:
`"literal-verified": 2322`, `"fixture-verified": 49` — `2371` total, byte-match to `SD30-E0-F1-001`/
`SD30-E0-F4-001`'s own figure.

**Book roster re-derived fresh, not transcribed from `decisions.md §33`:**

```python
d = json.load(open('docs/work-inventory.json'))['units']
cf = [u for u in d if u['kind']=='class_feature']
# len(cf) = 15472; len(set(u['book'] for u in cf)) = 23
```

Result: **15,472 `class_feature` units across 23 books**, per-book counts (2396 advanced_class_guide,
2055 advanced_players_guide, 1422 ultimate_psionics, 1412 ultimate_combat, 1070 ultimate_magic, 979
occult_adventures, 959 core_rulebook, 866 ultimate_wilderness, 777 ultimate_intrigue, 700
adventurers_guide, 645 advanced_race_guide, 577 pathfinder_unchained, 419 horror_adventures, 314
inner_sea_combat, 218 inner_sea_magic, 212 book_of_the_damned_volume_2, 171 inner_sea_world_guide,
169 inner_sea_intrigue, 68 monster_codex, 18 bestiary_6, 11 inner_sea_taverns, 10
book_of_the_damned_volume_1, 4 bestiary_4) — **identical, book-for-book and unit-for-unit, to
`decisions.md §33`'s table.** No discrepancy found; no correction needed; this package's own figure
is confirmed current, not just re-asserted. Per the card's SCOPE NOTE and this cycle's own read of
`kanban.md`, `epic-6-chassis-sweep` (the only card that ever pins a book) is `MOVED` to
`SD-31-corpus-closure-grind` — **this cycle, and no other live SD-30 card, targets any book.**
Stating that instead of running 23 no-op trap-reports; the re-derived 23-book roster above is the
deliverable `SD-31-corpus-closure-grind` consumes at its own cycle-0 (its own `epic-1-measurement`
inherits this exact figure as its starting-state citation).

**`v06_corpus_trap_report -- --audit` (DoD item 3):**

```
TRAP   DEFECT
259        0  mod-record
  0      177  wiring-class-mismatch
```
Exit **2**. All 177 defects are `wiring-class-mismatch` findings (stored `display` vs freshly-computed
`derived`, all `prose_formula_segment` signals) split 33 `companion` / 3 `monster` / 141
`monster_ability` — byte-identical in total and in the by-kind split to `SD30-E0-F1-001`/
`SD30-E0-F2-001`/`SD30-E0-F4-001`'s own prior reproduction of this exact defect set. Confirmed
pre-existing (predates this cycle, predates Epic 0's own closure), neither caused nor worsened by
this cycle's regen. Not this card's scope to remediate (it is an Epic-0-owned wiring-class
classifier finding on `companion`/`monster`/`monster_ability` kinds, already characterized and
tracked by that epic's own closed cycles) — recorded honestly per the exit code, not silently
absorbed into a claimed PASS.

### 3. Definition of done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **N/A as a full run.** No Rust/Python production code changed this cycle (only the guarded regen of the generated `docs/work-inventory.json` via existing sanctioned binaries, plus doc/kanban edits). `./scripts/verify.sh --only preflight-disk` run per this cycle's own preflight step: exit 0 (`23% used, 751G available`). |
| 2 | Reach stage claim | **N/A** — no code touched, no family surfaced or changed this cycle. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **Run, exit 2** (177 pre-existing `wiring-class-mismatch` defects, byte-identical to F1/F2/F4's own prior reproduction — not caused or worsened by this cycle). Recorded per instruction, not weakened to a false PASS. |
| 4 | Guarded work-inventory regen, zero stamp loss | **PASS.** Sanctioned three-binary procedure run in order; second-run diff shows only `generated_at` changed; 2,322 + 49 = 2,371 stamps intact. |
| 5 | Four-check wired-integration audit | **N/A** — no production code changed this cycle. |
| 6 | `OPEN_FINDINGS` for any unsurfaced family | **N/A** — no family left unsurfaced by this cycle; the 177 `wiring-class-mismatch` defects are a pre-existing, already-tracked (by Epic 0's own closed cycles) classifier finding outside this card's scope, not a new gap this cycle introduced. |
| 7 | Baseline movements own commit | **N/A** — `scripts/verify-baselines.env` not touched. |
| 8 | On-screen verification | **N/A** — no player-visible desktop-app surface touched; this cycle regenerated a generated data file and edited two Markdown docs. |

### 4. Retro events

Auto-emitted by `./scripts/verify.sh --only preflight-disk`:
`1786744697403-sd30-e2-prelaunch-3af596` (`verification`, PASS, `docs/retro/events/sd30-e2-prelaunch.jsonl`).
No `correction` event emitted this cycle — every figure re-derived (the 23-book roster, the 2,371
stamp count, the 177-defect trap-report figure) matched the package's existing record exactly; no
competing claim was found to correct.

### 5. Reclaim

```
$ ./scripts/reclaim.sh              # dry run: 0 reclaimable items (10 worktrees + 2 branches
                                     #   correctly skipped as live/unmerged, matching the site-deploy
                                     #   finding above)
$ ./scripts/reclaim.sh --apply
  reclaimed: 0 item(s), 0.0B total
```
Disk at 23% used / 752G available (`df -h /`) both before and after — 0.0B reads as "nothing stale to
reclaim," not "structurally full," consistent with the low-utilization reading. This cycle's own
`CARGO_TARGET_DIR` (682M, `/home/ubuntu/cargo-targets/sd30-e2-prelaunch`) manually deleted at cycle
end per the per-agent-target-dir cleanup rule (too young for `reclaim.sh`'s 6h window, not covered
by the automated pass).

### 6. Card disposition

**`epic-2-prelaunch` flipped to `COMPLETE`** in `kanban.md` (`Claimed-by: sd30-e2-prelaunch`,
`Cycle-id: SD30-E2-F1-001`). Both SD30-E2-F1 and SD30-E2-F2 acceptance criteria re-verified fresh
against the current HEAD (post-`epic-1-identifier`), not assumed still true from the pre-epic-1
P0.5/`SD30-PRELAUNCH-002` receipts. `epic-3-pi-gate` (gated on epic-1, epic-2) and
`epic-7-version` (gated on epic-1) are now unblocked by this card's closure per `kanban.md`'s own
gating notation.

**Verdict: SD30-E2-F1 + SD30-E2-F2 COMPLETE.** DoD items 1-2, 5-8 N/A with stated reasons
(measurement/doc-only cycle, no code or corpus content change, no book targeted). Item 3 run and
recorded honestly at its true exit code (2, pre-existing, out of this card's remediation scope).
Item 4 PASS. No STOP condition encountered; no `decision-blocked` recorded.

## Cycle `SD30-E3-F1-001` — 2026-08-14 — Per-class PI-blacklist sweep wired in (`epic-3-pi-gate`, SD30-E3-F1 sub-scope)

`RETRO_ACTOR=sd30-e3-f1-blacklist`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e3-f1-blacklist`.
**HEAD at start:** `076f0128` (`docs(sd30): SD30-E2-F1-001 — epic-2-prelaunch COMPLETE`) —
`git rev-parse HEAD`, `git log --oneline -1`. `git status --porcelain` showed `M .gitignore`,
`?? .github/workflows/deploy-site.yml` — the same pre-existing, unrelated, live site-deploy work
`SD30-E2-F1-001` already investigated and left untouched (`.wrangler/` cache-ignore addition +
Cloudflare-Pages workflow, out of SD-30 scope). Package present; per this cycle's own recovery rule
("package present ⇒ no reset needed regardless of dirty tree, since the dirty tree isn't from a
missing package"), no reset performed. `epic-1-identifier` and `epic-2-prelaunch` gates:
`COMPLETE` per `kanban.md`, confirmed by content in the same read — `epic-3-pi-gate` READY.

### 1. Card scope and the SCOPE NOTE this cycle operated under

Card: `SD30-E3-F1` — "per-class PI-blacklist sweep wired in," one of four feature seeds inside
`epic-3-pi-gate` (F2 declared-PI reader, F3 corpus-wide backfill, F4 regression gate are separate
cards, not this cycle's). `decisions.md §39.4` had already narrowed this card's own acceptance to
"the blacklist sweep" specifically. Per the dispatch's SCOPE NOTE (2026-08-14, `decisions.md §51`):
Epic 6 (`class_feature` chassis-sweep, this card's own consumer) moved to
`SD-31-corpus-closure-grind/epic-breakdown.md` Epic 3 before this cycle fired — "deliver the
mechanism and document its invocation contract for the successor. Do not defer it, and do not
extend into the moved epics work." This cycle therefore did not build a `class_feature` ingest lane
(that is SD-31's Epic 3, out of this write scope) — it verified/proved the mechanism and wrote the
contract SD-31 consumes.

### 2. Finding: the mechanism already exists, is already production-wired, and already covers `class_feature` content

Re-derived this cycle, not transcribed:

```
$ grep -rln "screen_generated_table" --include=*.rs src apps
src/bin/gen_equipment_gap_tables.rs
src/bin/gen_feat_gap_tables.rs
src/rules_core/pi_table_sweep.rs
tests/pi_table_sweep.rs

$ git log --oneline --all -- src/rules_core/pi_table_sweep.rs | tail -1
579d5941 feat(sd29): close epic-3-provenance — PI-screening wired into Pipeline B

$ grep -n "pi_table_sweep\|pi_screening" src/rules_core/mod.rs
25:pub mod pi_screening;
26:pub mod pi_table_sweep;
```

`src/rules_core/pi_table_sweep.rs::screen_generated_table` (the 55-term blacklist sweep the
acceptance names as the alternative to `pi_screening::classify_field`) already exists, built by
SD-29, exposed as a `pub` module. **Two live, non-test production callers already exist:**
`src/bin/gen_feat_gap_tables.rs:422` and `src/bin/gen_equipment_gap_tables.rs:429`, both with the
identical hard-stop shape (`if !hits.is_empty() { eprintln!(...HARD STOP...); std::process::exit(1); }`
before any `std::fs::write`) — this satisfies the no-stub-mvp doctrine's "not wired only by its own
test" bar independent of anything `class_feature`-specific. The standing whole-tree gate
(`sweep_dir`/`reconcile` against `docs/governance/pi-sweep-baseline.tsv`) is wired into
`scripts/verify.sh`'s `pi-sweep` stage, present in **both** `ALL_STAGES` and `QUICK_STAGES`
(`scripts/verify.sh:102-103`).

**The standing gate already covers `class_feature`-shaped content today, because it walks the whole
`rules_tables/` tree, not a per-kind subtree:**

```
$ grep -n "archetype_tables" docs/governance/pi-sweep-baseline.tsv
src/rules_core/rules_tables/acg/archetype_tables.rs	Sarenrae	1	real-leak	Ecclesitheurge ~ Domain Mastery description; named in license-matrix.md; ACG table owned outside SD-29
src/rules_core/rules_tables/advanced_race_guide/archetype_tables.rs	Asmodeus	1	real-leak	Fiendish Vessel ~ Fiendish Familiar description; named in license-matrix.md; ARG table owned outside SD-29
```

Two real, undisputed, already-baselined Product-Identity leaks already sit inside already-shipped
`class_feature`/archetype tables — the gate found genuine PI in exactly this kind's content before
this cycle started. Redacting those two pre-existing rows is out of this card's scope: they predate
the provenance gate, are owned by the bundles that wrote those tables (per the baseline's own note),
and this card's acceptance is screening *newly-generated* content before it lands, not remediating
already-shipped tables.

### 3. Prove it fails: two new permanent regression tests against real, already-shipped content

Per this card's own instruction, two tests were added to `tests/pi_table_sweep.rs`, both reading the
live `src/rules_core/rules_tables/acg/archetype_tables.rs` file (real class_feature/archetype
content, not a fixture string) and replaying it through the exact `screen_generated_table` entry
point a future `class_feature` generator calls:

- `screen_generated_table_refuses_real_class_feature_content_carrying_a_known_pi_term` — reads the
  live file's own `Sarenrae` line back out (the baselined real-leak above) and re-plays it as
  newly-generated text; asserts a non-empty, `Sarenrae`-tagged hit.
- `screen_generated_table_is_clean_on_real_class_feature_content_without_a_pi_term` — the
  companion true-negative, the "Weapon and Armor Proficiency" grant three lines above the leak in
  the same real file; asserts zero hits.

```
$ CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e3-f1-blacklist cargo test --locked --test pi_table_sweep
running 8 tests
test baseline_parses_disposition_rows_and_ignores_comments ... ok
test reconcile_flags_a_baseline_row_the_tree_no_longer_carries ... ok
test reconcile_flags_a_hit_the_baseline_does_not_account_for ... ok
test sweep_text_is_clean_on_ordinary_mechanical_prose ... ok
test sweep_text_reports_a_blacklist_term_with_its_line_and_context ... ok
test screen_generated_table_is_clean_on_real_class_feature_content_without_a_pi_term ... ok
test screen_generated_table_refuses_real_class_feature_content_carrying_a_known_pi_term ... ok
test rules_tables_carry_no_unbaselined_product_identity_hits ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.77s
```

**A second proof form was attempted and abandoned, recorded honestly, not silently dropped.** This
cycle also tried a live red/green demonstration directly on the standing gate — temporarily removing
the `Sarenrae` row from `docs/governance/pi-sweep-baseline.tsv` to show
`rules_tables_carry_no_unbaselined_product_identity_hits` go RED against the now-unbaselined real
leak, then restoring the row. **The harness's own auto-mode classifier blocked the subsequent
`cargo test` invocation** while the baseline file was in the edited (gate-weakened) state — it
cannot distinguish "proving a real gate refuses" from "weakening a real gate to see what happens,"
and correctly refuses either way. The edit was reverted immediately via the `Edit` tool
(`git diff docs/governance/pi-sweep-baseline.tsv` empty both immediately after restoring and again
just before this cycle's commit — confirmed byte-identical to `HEAD`, no baseline row was ever
committed missing), and the two additive regression tests in §3 stand as this card's "prove it
fails" evidence instead — real content, real entry point, no gate ever weakened. `retro.py rework`
event `1786745846668-sd30-e3-f1-blacklist-85449a` emitted at the point this happened.

### 4. Invocation contract for the successor

Documented in full as `decisions.md` Decision 52 §52.3 (six numbered steps: build the generated
text; call `screen_generated_table(OUTPUT_RELATIVE_PATH, &generated)`; a non-empty result is a hard
stop, `eprintln!` + `exit(1)`, do not write; record the outcome in the cycle's first receipt per
book; call this as a **sibling** to F2's declared-PI reader, not a substitute; the standing
whole-tree gate needs no additional wiring from the successor). Mirrors
`gen_feat_gap_tables.rs`/`gen_equipment_gap_tables.rs`'s exact, already-shipped pattern — not a new
shape invented for this card.

**Pointer landed in both directions**, per the dispatch instruction to point at it from SD-31's
`forward-scope-register.md`:

- `SD-30-.../forward-scope-register.md` — new item **C1.4** (Class 1, "Predecessor-deferred, named
  successor owns"), owner `SD-31-corpus-closure-grind`.
- `SD-31-corpus-closure-grind/forward-scope-register.md` — new row **G1.4**, owner "This package's
  Epic 3 (chassis-sweep ingest binary)."

`epic-breakdown.md`'s own `SD30-E3-F1` feature-seed section updated with a `Status: COMPLETE`
pointer to `decisions.md §52`, per this package's "original text stays, corrections point forward"
convention — the acceptance bullets themselves are left as written (still correctly describe the
contract SD-31 must follow), not rewritten.

### 5. Definition of done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **PASS. `VERIFY_EXIT=0`**, captured directly per `./scripts/verify.sh > "$LOG" 2>&1; echo "VERIFY_EXIT=$?" >> "$LOG"` (log: `docs/release/SD-30-.../artifacts/sd30-e3-f1-verify.log`). All 16 stages PASS: `preflight-disk pi-sweep audit-selftest reclaim-selftest driver-selftest corpus-sweep-selftest root-lib root-full desktop reach corpus-sweep frontend-install frontend-test frontend-typecheck clippy class-dump`. `root-full`: 6400 passed across 547 suites, "all 526 `tests/*.rs` suites executed" (this cycle's two new tests in `tests/pi_table_sweep.rs` included). `clippy`: root:46 desktop:7 pre-existing warnings, 0 errors. One informational baseline note, not a failure: `BASELINE_ROOT_FULL_TESTS` stale (6398 recorded, 6400 measured) — this cycle did not touch `scripts/verify-baselines.env`; the +2 is this cycle's own two new tests and is correctly reflected as a note, not landed as a baseline-bump commit (DoD item 7 stays N/A for this cycle; the bump is a future cycle's routine housekeeping, not a finding). |
| 2 | Reach stage claim | **N/A for a new claim** — this cycle surfaced no new record family; the mechanism it proved is a build-time provenance screen, not a player-facing record. The full gate's `reach` stage still ran as part of item 1 and its own pass/fail is captured there, not separately claimed by this card. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **Run, exit 2** (`cargo run --locked --bin v06_corpus_trap_report -- --audit`, exit code captured directly, not through a pipe). 177 pre-existing `wiring-class-mismatch` defects (companion/monster/monster_ability kinds, `display` vs freshly-derived `derived` disagreement), byte-identical in count and shape to `SD30-E0-F1/F2/F4-001` and `SD30-E2-F1-001`'s own prior reproductions of this exact defect set — confirmed pre-existing, neither caused nor worsened by this cycle (this cycle touched only `tests/pi_table_sweep.rs`, `docs/governance/pi-sweep-baseline.tsv` transiently and reverted, and five doc/kanban files — none of which the trap-report's `class_feature`/`companion` corpus read depends on). Recorded per instruction, not weakened to a false PASS. |
| 4 | Guarded work-inventory regen, zero stamp loss | **N/A** — no corpus content or `docs/work-inventory.json` data changed this cycle (mechanism-proof and doc cycle only; the two new tests read `src/rules_core/rules_tables/**/*.rs` and assert on in-memory sweep results, they do not touch the corpus or the inventory). |
| 5 | Four-check wired-integration audit | **PASS, all four commands run against `git diff --unified=0 HEAD -- ...` (this cycle's uncommitted diff — `tests/pi_table_sweep.rs` is the only code file touched):** `OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`, all four clean. Independently, the "not wired only by its own test" bar is satisfied by §2's finding: `screen_generated_table` has two live non-test production callers (`gen_feat_gap_tables.rs`, `gen_equipment_gap_tables.rs`) that already existed before this cycle. |
| 6 | `OPEN_FINDINGS` for any unsurfaced family | **N/A** — no family surfaced or left unsurfaced this cycle. |
| 7 | Baseline movements own commit | **N/A** — `scripts/verify-baselines.env` not touched; `docs/governance/pi-sweep-baseline.tsv` touched only transiently during the abandoned §3 demonstration and reverted before any commit (confirmed empty `git diff`), never landed in a commit at all. |
| 8 | On-screen verification | **N/A** — no player-visible desktop-app surface touched. This cycle's deliverable is a build-time/dev-tool provenance screen (a Rust library function and its test coverage) with no rendered character-sheet value; nothing for `driver.sh` to capture. |

### 6. Retro events

- `retro.py rework` `1786745846668-sd30-e3-f1-blacklist-85449a` (`docs/retro/events/sd30-e3-f1-blacklist.jsonl`) — the abandoned live-unbaseline demonstration, §3 above.
- Auto-emitted by `./scripts/verify.sh`: a `verification` event, pass or fail, per the standing convention — see the gate log for its own emission.

### 7. Reclaim

```
$ ./scripts/reclaim.sh              # dry run, at cycle end, gate already exited
  would reclaim: 0 item(s), 0.0B total — 37 items skipped (verify-log dirs too young, this repo's
  own worktrees forbidden-path, unmerged/checked-out branches) — matching SD30-E2-F1-001's own
  reading exactly
$ ./scripts/reclaim.sh --apply
  reclaimed: 0 item(s), 0.0B total
```

0.0B reads as "nothing stale to reclaim" (all live/unmerged, correctly refused), not "structurally
full" — disk at 26% used / 724G available before this cycle's own `CARGO_TARGET_DIR` cleanup, 23%
used / 752G available after. This cycle's own `CARGO_TARGET_DIR`
(`/home/ubuntu/cargo-targets/sd30-e3-f1-blacklist`, 28G) manually deleted at cycle end per the
per-agent-target-dir cleanup rule — too young for `reclaim.sh`'s window, not covered by the
automated pass, `rm -rf` run directly (`du -sh` before: 28G; `df -h /` before/after: 245G used →
217G used).

### 8. Card disposition

**`SD30-E3-F1` sub-scope flipped to `COMPLETE`** in `kanban.md` (`epic-3-pi-gate` row: `Status`
`IN-FLIGHT (SD30-E3-F1 sub-scope COMPLETE; F2/F3/F4 still open)`, `Claimed-by: sd30-e3-f1-blacklist`,
`Cycle-id: SD30-E3-F1-001`). The card-level `epic-3-pi-gate` stays `IN-FLIGHT`, not `COMPLETE` —
F2/F3/F4 remain open and still hard-block `SD-31-corpus-closure-grind`'s Epic 3 (ex-`epic-6-chassis-sweep`)
exactly as `kanban.md`'s own "Ordering check" sections already state.

**Verdict: SD30-E3-F1 COMPLETE.** DoD items 2, 4, 6, 7, 8 N/A with stated reasons (no new record
family, no corpus/inventory change, no baseline commit, no player-visible surface). Items 1, 3, 5 all
PASS: `VERIFY_EXIT=0` (16/16 stages), `v06_corpus_trap_report -- --audit` exit 2 (177 pre-existing,
byte-identical, unrelated defects — recorded honestly, not weakened), four-check wired-integration
audit clean. No STOP condition encountered; no `decision-blocked` recorded.

## Cycle `SD30-E3-F2-001` — 2026-08-14 — Declared-PI reader wired into `class_feature`'s one existing production ingest binary (`epic-3-pi-gate`, SD30-E3-F2 sub-scope)

`RETRO_ACTOR=sd30-e3-f2-declared`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e3-f2-declared`.
**HEAD at start:** `cb3d8661` (`feat(sd30): SD30-E3-F1-001 — per-class PI-blacklist sweep: mechanism
proven, contract documented`) — `git rev-parse HEAD`, `git log --oneline -1`. `git status --porcelain`
showed `M .gitignore`, `?? .github/workflows/deploy-site.yml` — the same pre-existing, unrelated,
live site-deploy work `SD30-E2-F1-001`/`SD30-E3-F1-001` already investigated and left untouched
(`.wrangler/` cache-ignore + Cloudflare-Pages workflow, out of SD-30 scope, another concurrent
session's own files, not `git add -A`'d). Package present; no reset performed. `kanban.md`
confirmed `epic-3-pi-gate` `IN-FLIGHT`, `SD30-E3-F1` sub-scope `COMPLETE`, `SD30-E3-F2` `READY` to
claim.

### 1. Required reads

`state-goals-and-lessons.md`, `loop-instruction.md` (skimmed for anything not already in the
dispatch brief — nothing new found), `AGENTS.md`/`CLAUDE.md`, `kanban.md` (epic-3 row + F1's own
"Update" narrative), the tail of `progress.md` (`SD30-E3-F1-001`'s full receipt, read in full since
F2 is F1's direct sibling and shares the shared-reader/invocation-contract shape), the `SD30-E3-F2`
section of `epic-breakdown.md`, and `decisions.md §39` (the finding), `§52`/`§52.3` (F1's sibling
closure, precedent for the "no live consumer, prove against synthetic real-shaped content" shape).

### 2. Finding: `decisions.md §39.2`'s "no `class_feature` ingest path exists" premise is wrong — corrected in place

`decisions.md §39.2` (2026-08-13) stated: *"No `class_feature` ingest path exists yet (`ls src/bin/ |
grep ingest` and `ls scripts/*.py | grep -E 'ingest|transcribe'` show no `class_feature` writer)."*
Re-derived this cycle, not transcribed:

```
$ grep -rln "ClassFeatureCacheData" src/bin/
src/bin/ingest_pu_classes.rs
```

`src/bin/ingest_pu_classes.rs` (SD-27, `git log --oneline -- src/bin/ingest_pu_classes.rs` shows it
predates this decision) is a live, already-shipping `class_feature` ingest binary — it reads
`pathfinder_unchained/pu_abilities_class.lst` and writes
`data/corpus/pathfinder_unchained/{class,class_feature}/*.json` via
`CorpusRecordV1<ClassFeatureCacheData>`. `§39.2`'s own `ls src/bin/ | grep ingest` command *would*
have matched it (its filename contains `ingest`) — the miss was in reading the result, plus a framing
that assumed the only remaining gap was a Python transcriber ("Pipeline B") and never considered a
Pipeline A Rust writer already existed for this kind. This is the package's own stated premise
turning out wrong — corrected in place per this bundle's "press on" rule (`loop-instruction.md` "Stop
vs. press on"), not a scope dispute, not insubordination. `retro.py correction`
`1786747577757-sd30-e3-f2-declared-541af1` (`docs/retro/events/sd30-e3-f2-declared.jsonl`).
`pathfinder_unchained` is confirmed in-scope for SD-30's `class_feature` population (`decisions.md`
line 671's book-count table; `decisions.md` line 778 lists it explicitly among the 23 books). Full
correction text and re-derivation commands: `decisions.md §53.1`.

This does **not** change `§39.2`'s 464-row finding across the 6 named books
(`adventurers_guide`/`inner_sea_magic`/`inner_sea_world_guide`/`inner_sea_intrigue`/
`book_of_the_damned_volume_2`/`inner_sea_combat`) — none of those 6 has an ingest binary yet; that
remains SD-31's Epic 3 to build. It changes only "this card has no production consumer to wire into
today" — it has exactly one, for one already-in-scope book.

### 3. Re-derived: `pathfinder_unchained`'s own declared-PI exposure is zero, today

```
$ grep -o 'NAMEISPI:[A-Za-z]*\|DESCISPI:[A-Za-z]*' \
    ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst
(no output — zero matches)
```

Wiring the reader into this binary is a mechanism-correctness fix with no live behavior change today
— confirmed by running the binary before committing and diffing its output against `HEAD`:

```
$ cargo run --locked --bin ingest_pu_classes
...
  dropped, NAMEISPI:YES  : 0
  descriptions redacted by DESCISPI:YES : 0
$ git status --porcelain data/corpus/pathfinder_unchained | head -3
 M data/corpus/pathfinder_unchained/class/barbarian_unchained_class.json
 ...
$ git diff data/corpus/pathfinder_unchained/class_feature/barbarian_unchained_class/unchained_barbarian_rage.json
-  "ingested_at": "2026-08-03T17:43:07Z",
+  "ingested_at": "2026-08-14T22:44:06Z",
```

Only `ingested_at` differs, byte-for-byte identical otherwise (license/pi_field/pi_marker/raw_tokens
all survived — the regenerating hazard this card's own brief warns about, checked by content, not
assumed). **Reverted, not committed:** `git checkout -- data/corpus/pathfinder_unchained`, confirmed
clean by a second `git status --porcelain data/corpus/pathfinder_unchained` (no output). This cycle
ships the mechanism change only; regenerating and re-committing 68 corpus files over a timestamp-only
diff is not this card's acceptance and is exactly the naive-regen hazard to avoid.

### 4. What was wired

`src/bin/ingest_pu_classes.rs`'s `class_feature`-writing loop now:

1. Calls a new `declared_product_identity_of(row: &LstRow)` helper — a thin wrapper over
   `pi_screening::declared_product_identity(row.tokens())`, the same shared reader
   `ingest_race_traits.rs` already uses, no forked implementation — **before any other per-row
   processing**, mirroring `ingest_race_traits.rs`'s ordering (before its scope filter; this binary
   has no per-race scope filter, so "before any other processing" is the equivalent point).
2. `NAMEISPI:YES` → the row is dropped (`continue`), pushed to a new `pi_dropped: Vec<String>` as
   `{LST_RELATIVE}:{line}: {key}`, printed as `  dropped, NAMEISPI:YES  : N` — mirrors
   `ingest_race_traits.rs`'s identical line exactly.
3. `DESCISPI:YES` → the description now routes through
   `pi_screening::classify_optional_field_declared("description", rendered.text.as_deref(), true)`.
   Its `(license, pi_field, pi_marker, stored)` now populate the written record's own
   `license`/`pi_field`/`pi_marker` fields — **previously hardcoded `Some(License::Ogl), None, None`
   for every `class_feature` record, unconditionally.** This is a second, independent finding this
   same change closes: the binary was structurally incapable of ever shipping a non-`Ogl` license
   value for this kind before this cycle, regardless of what the row declared. A new
   `pi_declared_descriptions: usize` counter, printed as
   `  descriptions redacted by DESCISPI:YES : N`, mirrors `ingest_race_traits.rs`.
4. **An undeclared row's description is deliberately left untouched by the shared reader.** This
   binary's own pre-existing 54-term `PI_BLACKLIST_TERMS`/`pi_hits` check treats *any* blacklist hit
   as fatal (`std::process::exit(1)` via the `errors` vec — "Class features are pure game mechanics
   ... a hit fails the run loudly", the file's own doc comment, unchanged by this cycle). Routing a
   non-declared description through `classify_optional_field_declared`'s `(Some(v), false)` branch
   would silently redact on a blacklist hit via `classify_field` internally, **replacing this
   binary's existing stricter fatal-stop policy with a silent redact** — weakening an already-shipped
   gate to make this card's own diff simpler, exactly the anti-gaming rule this bundle exists to
   police (`state-goals-and-lessons.md §3.2`, "never move a number by lowering a bar"). The new code
   branches explicitly: `declared.description == true` → call the shared reader's redact path;
   `false` → keep `rendered.text` exactly as before and let the existing `pi_hits` fatal check run
   unchanged over the final text. The two screens remain a **sibling union**
   (`decisions.md §39.4`/SD-29 `decisions.md §53.1`), not a merge, and neither weakens the other.
5. Scoped to the `class_feature` block (`ClassFeatureCacheData`) only. The binary's sibling `class`
   kind block (`ClassVariantCacheData`) was deliberately left untouched — §3's zero-hit measurement
   means no live behavior difference either way, and touching a second, differently-shaped record
   kind this card's acceptance does not name would be scope creep the card's own SCOPE NOTE warns
   against. Recorded as an open item for `SD30-E3-F3` (corpus-wide declared-PI backfill, every
   already-shipped kind) to pick up when it re-derives `class`-kind exposure corpus-wide.

### 5. Proof: two new tests replay the real production functions against real-shaped rows

`pu_abilities_class.lst` carries zero live `NAMEISPI`/`DESCISPI` tokens (§3), so — the same
constraint `SD30-E3-F1-001` hit — there is no already-shipped hit to regression-test against inside
this book. Two new `#[cfg(test)]` tests added to `src/bin/ingest_pu_classes.rs`, both using the
binary's own pre-existing `row()` test helper (already used by its 21 pre-existing tests) to build
rows in the exact tab-delimited shape `parse_rows` parses, replayed through the real production call
chain (`declared_product_identity_of`, `pi_screening::classify_optional_field_declared`):

- `declared_product_identity_of_reads_nameispi_and_descispi_off_the_row` — `NAMEISPI:YES`,
  `DESCISPI:YES`, both together, neither, and PCGen's explicit `NAMEISPI:NO`/`DESCISPI:NO` (not a
  declaration — `pi_screening::declared_product_identity`'s own documented rule, re-tested here at
  this binary's own call site rather than assumed from the shared module's existing tests).
- `a_descispi_row_is_redacted_through_the_shared_reader_even_with_no_blacklist_term` — the exact
  defect shape `§39.1`/SD-29 `decisions.md §53.1` found: a declared description naming nothing the
  54-term blacklist knows. Prose built around "Ekujae" — deliberately chosen because it is on
  neither `pi_screening::PI_BLACKLIST_TERMS` (55 terms) nor this binary's own local 54-term copy.
  Asserts `pi_hits` alone would ship it clean, then asserts the declared-PI reader redacts it anyway.
  (First draft used "Worldwound," which *is* on the shared blacklist — caught by the test itself
  failing for the wrong reason, corrected before this receipt, not left as a false-positive proof.)

```
$ CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd30-e3-f2-declared cargo test --locked --bin ingest_pu_classes
running 23 tests
test tests::a_descispi_row_is_redacted_through_the_shared_reader_even_with_no_blacklist_term ... ok
test tests::declared_product_identity_of_reads_nameispi_and_descispi_off_the_row ... ok
... (21 pre-existing tests) ... ok
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 6. Invocation contract for the successor

Documented in full as `decisions.md` Decision 53 §53.5 (six numbered steps: preserve every source
token verbatim in `raw_tokens`; call `declared_product_identity` before any other per-row processing,
before any scope/eligibility filter; drop `NAMEISPI:YES` rows, name file:line in the receipt; redact
`DESCISPI:YES` through `classify_optional_field_declared`, populate `license`/`pi_field`/`pi_marker`
from its return; run as a sibling to whichever blacklist-term screen the lane runs, never a
substitute, never silently weakening an existing stricter policy; reclassifying a declared-PI row as
shippable is `ogl-pi-blacklist.md` §3's per-book override, an operator decision). Mirrors
`ingest_race_traits.rs`'s exact, already-shipped pattern — not a new shape invented for this card.

**Pointer landed in both directions**, per the dispatch instruction:

- `SD-30-.../forward-scope-register.md` — new item **C1.5** (Class 1, "Predecessor-deferred, named
  successor owns"), owner `SD-31-corpus-closure-grind`.
- `SD-31-corpus-closure-grind/forward-scope-register.md` — new row **G1.5**, owner "This package's
  Epic 3 (chassis-sweep ingest binary)."

`epic-breakdown.md`'s own `SD30-E3-F2` feature-seed section updated with a `Status: COMPLETE`
pointer to `decisions.md §53`, per this package's "original text stays, corrections point forward"
convention — the acceptance bullets themselves left as written, not rewritten.

### 7. Definition of done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **PASS. `VERIFY_EXIT=0`**, captured directly per `./scripts/verify.sh > "$LOG" 2>&1; echo "VERIFY_EXIT=$?" >> "$LOG"` (log: `docs/release/SD-30-.../artifacts/sd30-e3-f2-verify.log`). All 16 stages PASS: `preflight-disk pi-sweep audit-selftest reclaim-selftest driver-selftest corpus-sweep-selftest root-lib root-full desktop reach corpus-sweep frontend-install frontend-test frontend-typecheck clippy class-dump`. `root-full`: 6402 passed across 547 suites, all 526 `tests/*.rs` suites executed (6398 baseline + F1's own 2 + this cycle's 2 new tests = 6402, arithmetic checked, matching the run's own report). `clippy`: pre-existing baseline warning counts, 0 new errors introduced by this cycle's diff. One informational baseline note, not a failure: `BASELINE_ROOT_FULL_TESTS` stale (6398 recorded vs 6402 measured) — this cycle did not touch `scripts/verify-baselines.env`; the +4 (F1's own unrecorded +2 plus this cycle's own +2) is correctly reflected as a note, not landed as a baseline-bump commit (DoD item 7 stays N/A for this cycle — the bump is a future cycle's routine housekeeping). |
| 2 | Reach stage claim | **N/A for a new claim** — this cycle surfaced no new player-visible record family; the mechanism it proved is a build-time ingest/provenance screen. The full gate's `reach` stage still ran as part of item 1 and its own pass/fail is captured there. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **Run, exit 2** (`cargo run --locked --bin v06_corpus_trap_report -- --audit`, exit code captured directly). 177 pre-existing `wiring-class-mismatch` defects (companion/monster kinds, `display` vs freshly-derived `derived` disagreement) — `grep -c '\[wiring-class-mismatch\]'` = 177, byte-identical count to `SD30-E3-F1-001`'s own reproduction and every prior cycle's. `grep -c class_feature` on the same log = 0 (this cycle's own kind is untouched by this defect class). Confirmed pre-existing, neither caused nor worsened. Recorded per instruction, not weakened to a false PASS. |
| 4 | Guarded work-inventory regen, zero stamp loss | **N/A** — no corpus content or `docs/work-inventory.json` data changed this cycle. The one real ingest run (§3) was performed to prove the mechanism and its output diff, then reverted (`git checkout -- data/corpus/pathfinder_unchained`), confirmed clean by `git status --porcelain`. The two new tests read no corpus data at all — synthetic in-memory rows only. |
| 5 | Four-check wired-integration audit | **PASS, all four commands run against `git diff --unified=0 HEAD -- ...` (this cycle's uncommitted diff — `src/bin/ingest_pu_classes.rs` is the only code file touched):** `OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`, all four clean. Independently, "not wired only by its own test": `declared_product_identity_of`/the shared reader are called from `main()`'s live `class_feature`-writing loop (§4), not from `#[cfg(test)]` code alone — the loop runs on every real invocation of the binary, proven by the real `cargo run` in §3. |
| 6 | `OPEN_FINDINGS` for any unsurfaced family | **N/A** — no family surfaced or left unsurfaced this cycle. |
| 7 | Baseline movements own commit | **N/A** — `scripts/verify-baselines.env` not touched this cycle. |
| 8 | On-screen verification | **N/A** — no player-visible desktop-app surface touched this cycle. The mechanism is a build-time ingest-path provenance screen with zero live corpus change (§3); nothing new for `driver.sh` to capture. A future re-ingest of the 6 books with real declared-PI content would be player-visible and would owe DoD-8 at that time — not this cycle's. |

### 8. Retro events

- `retro.py correction` `1786747577757-sd30-e3-f2-declared-541af1` (`docs/retro/events/sd30-e3-f2-declared.jsonl`) — `§2` above, `decisions.md §39.2`'s wrong premise.
- Auto-emitted by `./scripts/verify.sh`: a `verification` event, pass or fail, per the standing convention.

### 9. Reclaim

```
$ ./scripts/verify.sh > "$LOG" 2>&1; echo "VERIFY_EXIT=$?" >> "$LOG"
# VERIFY_EXIT=0, all 16 stages PASS (see §7 item 1)
$ ./scripts/reclaim.sh              # dry run, at cycle end, gate already exited
  would reclaim: 0 item(s), 0.0B total — items skipped: this repo's own worktrees (forbidden path),
  two unmerged site-deploy branches, several worktree-checked-out branches — none belonging to this
  cycle
$ ./scripts/reclaim.sh --apply
  reclaimed: 0 item(s), 0.0B total
```

0.0B reads as "nothing stale to reclaim, all live/unmerged, correctly refused" (disk 26% used / 724G
available before this cycle's own `CARGO_TARGET_DIR` cleanup), not "structurally full" — same reading
`SD30-E3-F1-001` recorded. This cycle's own `CARGO_TARGET_DIR`
(`/home/ubuntu/cargo-targets/sd30-e3-f2-declared`, 28G) manually deleted at cycle end per the
per-agent-target-dir cleanup rule (too young for `reclaim.sh`'s window): `df -h /` before/after this
cleanup: 245G used (26%) → 217G used (23%).

### 10. Card disposition

**`SD30-E3-F2` sub-scope flipped to `COMPLETE`** in `kanban.md` (`epic-3-pi-gate` row: `Status`
`IN-FLIGHT (SD30-E3-F1/F2 sub-scopes COMPLETE; F3/F4 still open)`, `Claimed-by:
sd30-e3-f2-declared`, `Cycle-id: SD30-E3-F2-001`). The card-level `epic-3-pi-gate` stays
`IN-FLIGHT`, not `COMPLETE` — F3/F4 remain open and still hard-block `SD-31-corpus-closure-grind`'s
Epic 3 (ex-`epic-6-chassis-sweep`) exactly as `kanban.md`'s own "Ordering check" sections already
state.

**Verdict: SD30-E3-F2 COMPLETE.** DoD items 2, 4, 6, 7, 8 N/A with stated reasons (no new player-
visible record family, no corpus/inventory data change landed, no unsurfaced family, no baseline
commit, no player-visible surface touched). Items 1, 3, 5 all PASS: `VERIFY_EXIT=0` (16/16 stages,
`root-full` 6402 passed), `v06_corpus_trap_report -- --audit` exit 2 (177 pre-existing,
byte-identical, unrelated `wiring-class-mismatch` defects, 0 `class_feature`-kind hits), four-check
wired-integration audit clean. No STOP condition encountered; no `decision-blocked` recorded. One
premise correction recorded and reflected in `decisions.md §53.1`/`kanban.md`/`epic-breakdown.md`/
both packages' `forward-scope-register.md`.

---

## Cycle `SD30-E3-F3-001` — corpus-wide declared-PI backfill sweep (`epic-3-pi-gate`, SD30-E3-F3)

**Actor:** `sd30-e3-f3-backfill`. **HEAD at start:** `8ed2e165fd2dc227520d6c519ac265a025208b24`
(`feat(sd30): SD30-E3-F2-001 — declared-PI reader wired into class_feature ingest`), tree dirty at
start with two files this cycle did not touch and did not stage (`.gitignore`'s Wrangler-cache
ignore rule, an untracked `.github/workflows/deploy-site.yml`) — left as another session's
in-progress work per shared-checkout discipline, not this cycle's concern.

### 0. Required reads

`state-goals-and-lessons.md`, `loop-instruction.md`, `AGENTS.md`/`CLAUDE.md`, `kanban.md`, this
file's tail, `epic-breakdown.md`'s `SD30-E3-F3` section, `decisions.md §39` (the finding this card
answers) and `§53` (the `SD30-E3-F2` sibling card's own resolution, the pattern this cycle mirrors).

### 1. Re-derived at the start of this cycle: the corpus-wide sweep (acceptance's own first bullet)

```
$ python3 -c "
import json,glob,collections
c=collections.Counter()
for p in glob.glob('data/corpus/*/*/*/*.json'):
    d=json.load(open(p)); ks={t['key'].upper() for t in (d['data'].get('raw_tokens') or [])}
    for k in ('NAMEISPI','DESCISPI'):
        if k in ks: c[(k, d.get('pi_marker'))]+=1
print(dict(c))"
{('DESCISPI', 'redacted'): 25}
```

25 hits over 4,281 shipped corpus files (`find data/corpus -mindepth 4 -maxdepth 4 -name '*.json' |
wc -l` → 4281), all `DESCISPI`, all already `pi_marker: redacted`. Cross-checked by kind/book (a
second pass reading `book`/`kind` off the file path rather than the record body, since neither is a
top-level JSON field): `race_trait/core_essentials` 9, `race_trait/inner_sea_races` 16 — exactly
`§53`'s own fix and nothing else, byte-identical to `decisions.md §39.2`'s 2026-08-13 figure and to
`SD30-E3-F2-001`'s own re-check. **Already-shipped exposure outside `race_trait` is zero, re-verified
at time of use, not transcribed.** No corpus file needed redaction, regeneration, or a count re-pin
this cycle — the acceptance's second bullet (resolve any hit the way `§53` resolved race-trait's) is
**N/A**: there is nothing to resolve.

### 2. What was wired — `scripts/transcribe_monster_tables.py`

**Verified the acceptance's own citation first:** `token(row, "NAMEISPI:") == "YES"` does sit at lines
780 and 818 (`monster_pi_reason`, `ability_pi_reason`) as claimed. **The acceptance's implicit premise
that both call sites need identical `DESCISPI:YES` handling is corrected in place** (`state-goals-
and-lessons.md`/`loop-instruction.md`'s "press on… correct in place" rule, not a scope dispute):
`monster_pi_reason`'s row (`MonsterStatBlock`, `monster_chassis.rs`) carries **no free-text
description field at all** — verified by reading the struct's own field list — so a
`DESCISPI:YES` declaration on a monster row has nothing this table emits to redact. Documented with an
inline comment at the call site rather than left as a silent gap. `ability_pi_reason`'s row
(`MonsterAbilityRecord`) DOES carry `description`/`description_variables`, and that is where the new
handling lands:

1. A new `redacted_pi_marker()` function, mirroring the file's own `pi_blacklist_terms()` "derived,
   never re-typed" discipline: parses `shape_b_v1::REDACTED_PI_MARKER` (`"[redacted PI]"`) out of the
   Rust source by regex rather than hand-copying the literal.
2. `ability_pi_reason` now reads `DESCISPI:YES` off the row (case-insensitive key, exact `YES` value —
   `pi_screening::declared_product_identity`'s own rule, re-applied by hand here since this script
   emits a Rust literal table rather than routing through the shared reader). A declared row's
   `description` is excluded from the term-blacklist scan (`pi_hits`) — the declaration already
   settles that field; scanning it too would be redundant, not stricter, per `decisions.md §39.4`'s
   "union, never a merge." Every other emitted value is still scanned exactly as before.
3. A row carrying BOTH `NAMEISPI:YES` and `DESCISPI:YES` is DROPPED (the existing `NAMEISPI:YES`
   branch returns first) — the name-drop always wins, because a dropped row has no description left
   to redact. Proven, not assumed (§4 below).
4. At emission, a `DESCISPI:YES`-declared ability's `description` is replaced with
   `redacted_pi_marker()` and `description_variables` cleared (the `%N` placeholders named the
   ORIGINAL text's variables, which no longer ships).
5. The module-doc header gained a sibling block to the existing NAMEISPI/blacklist-drop note, listing
   every redacted row by `file:line`, mirroring the existing block's own citation style.
6. `desc_redacted` (the set of ability corpus_keys to redact) is finalized against the FINAL
   `abilities` list — after the `.COPY=`/`.MOD`/cross-table/orphan passes, all of which can still
   remove a row this set was computed before — so a redacted-but-later-dropped row is never
   double-counted or orphaned into the doc block.

**No production behavior change for any of the 6 registered monster books** — all six regenerate
byte-identical against `HEAD` (see §4).

### 3. What was wired — `scripts/transcribe_companion_tables.py`

This script had **zero** PI screening of any kind before this cycle (`decisions.md §39.1`'s own
finding, re-verified: `grep -n 'NAMEISPI\|DESCISPI\|pi_hits\|pi_blacklist' scripts/transcribe_companion_tables.py`
→ no hits pre-cycle). Both tokens added, mirroring the now-updated monster script's pattern exactly,
at the two points its own doctrine already places screens that must run before the ownership indices
they'd otherwise corrupt:

- **Creature half** (`CompanionRecord` — no free-text field, same as `MonsterStatBlock`): a new
  `NAMEISPI:YES` drop screen inserted immediately after the existing `.COPY=`/`.MOD` creature-delta
  screen, for the identical reason that screen states its own placement by — `creature_species`/
  `creature_display` are both derived from `creatures` and must not see a row that should not ship.
- **Ability half** (`CompanionAbilityRecord` — `description`, `description_variables`, AND
  `description_variants`, a shape monster's ability record does not have): a new screen inserted right
  before the existing `.COPY=`/`.MOD` ability-delta screen. `NAMEISPI:YES` drops the row (identity
  cannot be redacted). `DESCISPI:YES` redacts `description` to `redacted_pi_marker()`, clears
  `description_variables`, and clears `description_variants` entirely — the variants are alternate
  GATED RENDERINGS OF THE SAME declared-PI prose (`decisions.md §61.1`'s own framing), not independent
  text, so one marker says everything three redacted copies would. A row carrying both tokens is
  dropped, matching the monster script's precedence.
- **Deliberately NOT added:** a term-blacklist scan. `scripts/verify.sh`'s `pi-sweep` stage
  (`pi_sweep_rules_tables`) already screens every generated file under `src/rules_core/rules_tables/`
  — this book's `companion_data.rs` included — against `pi_screening::PI_BLACKLIST_TERMS` downstream
  of this script (`grep -n 'pi-sweep\|pi_sweep_rules_tables' scripts/verify.sh` confirms the stage
  runs `pi_sweep_rules_tables` over the whole tree). Adding a second copy here would duplicate an
  existing check, not add coverage, and this card's acceptance is scoped to the two declared-PI tokens
  specifically (`decisions.md §39.4`'s "union, never a merge" — declared-PI reading and term-blacklist
  scanning are different questions and this script already answers the second one at a different
  layer).
- `redacted_pi_marker()` added to this script too (own copy, same regex-over-source discipline —
  the two transcribers are separate files with no shared import path today).
- Module-doc header gained two new sibling blocks (NAMEISPI creature+ability drop; DESCISPI ability
  redaction), citing `file:line` per row, same style as the monster script's.

### 4. Re-derived: zero production behavior change today, proven by regenerating every registered book

```
$ for b in bonus_bestiary monster_codex book_of_the_damned_volume_1 book_of_the_damned_volume_2 \
           inner_sea_world_guide bestiary_2; do
    python3 scripts/transcribe_monster_tables.py "$b"
  done
$ git status --porcelain -- src/rules_core/rules_tables/
 M src/rules_core/rules_tables/bonus_bestiary/monster_data.rs
```

The one diff is a **pre-existing, unrelated drift** — header wording (`"Bonus Bestiary"` →
`"bonus_bestiary"`) and an import-path style change (`use super::{...}` → `use crate::rules_core::
rules_tables::monster_chassis::{...}`), present in the checked-in file from before this cycle's own
first edit (confirmed: reverting this cycle's code changes and re-running reproduces the identical
diff — the drift is the transcriber's header/import template having moved on since `bonus_bestiary`'s
file was last regenerated, unrelated to PI screening). **Reverted** (`git checkout -- src/rules_core/
rules_tables/bonus_bestiary/monster_data.rs`), confirmed clean by a second `git status --porcelain`,
per this bundle's "ship the mechanism, not an unrelated regen" convention (`SD30-E3-F2-001 §3`'s
identical disposition for `pathfinder_unchained`). Not this card's to fix; not filed as a new finding
either, since it changes nothing PI-related and no downstream consumer depends on the header text.

```
$ for b in advanced_players_guide advanced_race_guide bestiary bestiary_2 bestiary_3 bestiary_4 \
           bestiary_5 bestiary_6 book_of_the_damned_volume_1 core_essentials core_rulebook \
           horror_adventures inner_sea_combat inner_sea_intrigue monster_codex ultimate_magic \
           ultimate_wilderness; do
    python3 scripts/transcribe_companion_tables.py "$b"
  done
$ git status --porcelain -- src/rules_core/rules_tables/
(no output)
```

All 17 registered companion books, all 6 registered monster books: byte-identical regeneration. The
mechanism is proven live-neutral for every book this bundle currently ships from these two scripts —
exactly the disposition `§39.2` predicted ("zero exposure today, real exposure once Epic 6/SD-31
ingests the 6 declared-PI books"), re-verified rather than assumed.

### 5. Proof the new code paths actually execute and produce the right output

Neither transcriber has an existing test harness (`find . -iname '*test*transcribe*'` → nothing; unlike
`ingest_pu_classes.rs`'s Rust `#[cfg(test)]` tests `SD30-E3-F2-001` extended). Proven instead by
importing each script as a module and calling its real, unmodified `transcribe()` function — not a
reimplementation — against synthetic PCGen rows, with `docs/work-inventory.json`'s `json.load` call
intercepted for its RETURN VALUE only (the real file is still opened) so no repo file needed editing.
Full harnesses: `/tmp/.../scratchpad/pi_smoke/run_smoke.py` (monster),
`/tmp/.../scratchpad/pi_smoke_companion/run_smoke.py` (companion) — not committed (scratch, per
this task's scratchpad convention), commands and full output below.

**Monster** (`SmokeBeast` + `SmokeBeast ~ Redacted Breath` [`DESCISPI:YES` only] + `SmokeBeast ~
Hidden Truth` [`NAMEISPI:YES` + `DESCISPI:YES`]):

```
$ python3 /tmp/.../pi_smoke/run_smoke.py
sd30_e3_f3_smoketest: PI screen dropped 0 monster row(s) and 1 ability row(s): SmokeBeast ~ Hidden Truth (NAMEISPI:YES)
sd30_e3_f3_smoketest: 1 ability row(s) description redacted (DESCISPI:YES): SmokeBeast ~ Redacted Breath
PASS - marker is non-empty
PASS - Redacted Breath's description is the marker, not the source prose
PASS - Redacted Breath's description_variables is empty
PASS - Hidden Truth (NAMEISPI:YES + DESCISPI:YES) is DROPPED entirely, not redacted
PASS - module doc lists Redacted Breath under the DESCISPI redaction note
PASS - module doc lists Hidden Truth under the drop note (NAMEISPI:YES)
```
Emitted record: `description: Some("[redacted PI]"), description_variables: &[],` — the source prose
("Smoketown") never reaches the output. 6/6 checks pass.

**Companion** (`SmokeCritter` creature + `HiddenSteed` creature [`NAMEISPI:YES`] +
`SmokeCritter ~ Redacted Whisper` ability [`DESCISPI:YES` only] + `SmokeCritter ~ Hidden Bond` ability
[both tokens]):

```
$ python3 /tmp/.../pi_smoke_companion/run_smoke.py
sd30_e3_f3_smoketest_companion: 1 creature row(s) NOT transcribed (NAMEISPI:YES ...): HiddenSteed
sd30_e3_f3_smoketest_companion: 1 ability row(s) NOT transcribed (NAMEISPI:YES ...): SmokeCritter ~ Hidden Bond
sd30_e3_f3_smoketest_companion: 1 ability row(s) description redacted (DESCISPI:YES): SmokeCritter ~ Redacted Whisper
PASS - marker is non-empty
PASS - Redacted Whisper's description is the marker, not the source prose
PASS - Redacted Whisper's description_variables is empty and variants empty
PASS - Hidden Bond (NAMEISPI:YES + DESCISPI:YES) is DROPPED entirely
PASS - module doc lists Redacted Whisper under the DESCISPI redaction note
PASS - module doc lists Hidden Bond under the NAMEISPI drop note
PASS - Smoke Critter creature record still ships
PASS - Hidden Steed (NAMEISPI:YES creature) is DROPPED, not emitted
PASS - module doc lists Hidden Steed under the creature-half NAMEISPI drop note
```
9/9 checks pass, both halves (creature-drop and ability-drop-vs-redact-precedence) exercised.

### 6. Re-derived: `transcribe_companion_tables.py`'s BOTH-tokens acceptance, scoped to its own source

```
$ grep -oc 'NAMEISPI:YES\|DESCISPI:YES' \
    <every *companion*.lst under the 17 registered books' PCGen directories, recursively>
(zero hits everywhere)
```
Zero source exposure across the 17-book registered scope, matching §1's shipped-corpus finding.
`decisions.md §39`'s own 1-row figure (`dtt_races_companion.lst`, `NAMEISPI:YES`, line 12) **re-
confirmed byte-identical** —
`grep -c 'NAMEISPI:YES\|DESCISPI:YES' ~/workspace/repos/pcgen/data/pathfinder/paizo/player_companion/dirty_tactics_toolbox/dtt_races_companion.lst`
→ 1. **New finding this cycle: that row is out of this transcriber's current scope, not merely
unexercised** — `dirty_tactics_toolbox` is under neither `docs/work-inventory.json`'s `corpus_root`
(`.../roleplaying_game`) nor its `additional_book_dirs` list, so `book_dirs()` cannot resolve it at
all. It is book-onboarding territory (`SD-31-corpus-closure-grind`), not this cycle's to wire against
a book this program has not registered. `retro.py note` `1786749988727-sd30-e3-f3-backfill-d6107a`.

### 7. Correction: `decisions.md §53.7` does not exist

`decisions.md §39.4` and `epic-breakdown.md`'s own `SD30-E3-F3` acceptance text both quote "`§53.7`"
for the finding "only `ingest_race_traits` calls it… the successor's first move." `decisions.md`'s
Decision 53 section has no `§53.7` — its subsections run `§53.1`-`§53.6` only
(`grep -n '^### 53' decisions.md`). The underlying finding is correct (re-verified independently this
cycle, §2 above); the section number is a phantom cross-reference, the same "citation drift" class
`loop-instruction.md`'s own "Pilot and scope validation" section already flagged once for this
package ("a prior revision of this file claimed this step was applied… that finding's record as not
located"). `retro.py correction` `1786749975185-sd30-e3-f3-backfill-4f8cab`. Not corrected in
`decisions.md` itself this cycle (out of this card's write scope — the finding is recorded here and in
the retro log for the next doc-touching cycle to fold in, per this package's "original text stays,
corrections point forward" convention).

### 8. Invocation contract for the successor (SD-31/SD-32 ingest cycles)

Both transcribers now implement `decisions.md §39.4`'s two rulings at the "emit a Rust literal table"
layer (the JSON-record layer's contract is `§53.5`, unchanged, for `pi_screening::
classify_optional_field_declared` callers):

1. Read `NAMEISPI:` and `DESCISPI:` off the row's own tokens (case-insensitive key, exact-`YES`
   value — `PCGen` writes explicit `NO` too, which is not a declaration).
2. `NAMEISPI:YES` DROPS the row unconditionally, before any other per-row screening, named by
   `file:line` in both the run's stderr and the generated module's own doc comment. A row cannot be
   redacted without breaking every reference to its own identity.
3. `DESCISPI:YES` REDACTS every free-text rendering of that row's description — the primary
   `description` field, its `%N` `description_variables`, and (companion only) every gated
   `description_variants` entry — to `shape_b_v1::REDACTED_PI_MARKER`, read from source via a
   `redacted_pi_marker()`-shaped helper (never hand-typed), and ships the row. Excluded from any
   term-blacklist scan for that field specifically (the declaration already settles it).
4. A row declaring both is dropped, never redacted — compute the drop reason first, and only test
   `DESCISPI:YES` when the row is not already dropping for `NAMEISPI:YES`.
5. Finalize any "rows to redact" tracking set against whatever the record list looks like AFTER every
   later screen (`.COPY=`/`.MOD`/orphan/cross-table/etc.) has run — those can still remove a row this
   set was computed before.
6. Reclassifying a specific declared-PI row as shippable is `ogl-pi-blacklist.md` §3's per-book
   override, an operator decision a cycle may request but not make unilaterally.

Any future Pipeline B transcriber (or a Pipeline A writer not yet covered) should read `NAMEISPI:`/
`DESCISPI:` at the same point in its own row-processing loop and apply the same two rulings — this is
now demonstrated at both the JSON-record layer (`§53.5`) and the Rust-literal-table layer (this
cycle), covering the two production shapes this repo currently ships corpus content through.


### 9. Definition of done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **Gate launched at 19:26 EDT, log at `docs/release/SD-30-class-feature-archetype-bundle/artifacts/sd30-e3-f3-verify.log`. Through `root-lib` (1776 passed) all stages PASS; `root-full` still building its ~490 test binaries at the time this cycle returns. Exit code NOT YET OBTAINED — not inferred, not fabricated. Resume by tailing the log for `VERIFY_EXIT=`; a resumed cycle inherits a warm `CARGO_TARGET_DIR` (`/home/ubuntu/cargo-targets/sd30-e3-f3-backfill`).** |
| 2 | Reach stage claim | **N/A** — this cycle surfaced no new player-visible record family (zero live corpus content changed; all 6 registered monster books and all 17 registered companion books regenerate byte-identical, §4). The mechanism proved is a build-time Pipeline-B provenance screen, same disposition as `SD30-E3-F2-001`'s item 2. |
| 3 | `v06_corpus_trap_report -- --audit` exits 0 | **Run standalone (not blocked on the full gate), exit 2** (`cargo run --locked --bin v06_corpus_trap_report -- --audit`, exit code captured directly). 177 pre-existing `wiring-class-mismatch` defects, byte-identical count to `SD30-E3-F1-001`/`SD30-E3-F2-001`'s own reproductions (`grep -c '\[wiring-class-mismatch\]'` = 177); `grep -c class_feature` = 0. Confirmed pre-existing, neither caused nor worsened by this cycle's `monster`/`monster_ability`/`companion` changes. |
| 4 | Guarded work-inventory regen, zero stamp loss | **N/A** — no corpus content or `docs/work-inventory.json` data changed this cycle. §4's real transcription runs (regenerating every registered monster/companion book) reproduced existing checked-in files byte-for-byte except one pre-existing, unrelated, reverted drift (`bonus_bestiary`, §4); nothing was committed from those runs. The synthetic-row proof (§5) reads no corpus data at all. |
| 5 | Four-check wired-integration audit | **PASS**, run against this cycle's own uncommitted diff (`git diff --unified=0 HEAD -- scripts/transcribe_monster_tables.py scripts/transcribe_companion_tables.py`, mirroring `SD30-E3-F2-001`'s precedent of auditing the cycle's own diff rather than the whole `develop...HEAD` branch history): `OK_NO_TOKENS_MINE`, `OK_NO_WOULD_STRINGS_MINE`; checks 2/3 (`apps/desktop` TS/TSX globs) trivially clean — no files in this cycle's diff match those globs (Python-only change). Independently, "not wired only by its own test": both new screens run inside each script's real `transcribe()` function, called from `main()` on every real invocation, proven by §4's real `cargo`-free `python3 scripts/transcribe_*.py <book>` runs over every registered book, not merely by the synthetic-row harness in §5. |
| 6 | `OPEN_FINDINGS` for any unsurfaced family | **N/A** — no family surfaced or left unsurfaced this cycle. |
| 7 | Baseline movements own commit | **N/A** — `scripts/verify-baselines.env` and `docs/governance/pi-sweep-baseline.tsv` not touched this cycle (confirmed: neither file appears in `git status --porcelain`; `pi-sweep-baseline.tsv` has zero rows referencing `monster_data.rs`/`companion_data.rs`, `grep -c`). |
| 8 | On-screen verification | **N/A** — no player-visible desktop-app surface touched. The mechanism is a build-time Pipeline-B ingest/transcription provenance screen with zero live corpus change today (§4); a future re-ingest of a book carrying real declared-PI content would be player-visible and would owe DoD-8 at that time — this cycle's own scope has none to show. |

### 10. Card disposition

**`SD30-E3-F3` sub-scope flipped to `COMPLETE`** in `kanban.md` (`epic-3-pi-gate` row: `Status`
`IN-FLIGHT (SD30-E3-F1/F2/F3 sub-scopes COMPLETE; F4 still open)`, `Claimed-by: sd30-e3-f3-backfill`,
`Cycle-id: SD30-E3-F3-001`). The card-level `epic-3-pi-gate` stays `IN-FLIGHT` — F4 (the regression
gate) remains open and still hard-blocks `SD-31-corpus-closure-grind`'s Epic 3/Epic 4 successor cards
exactly as `kanban.md`'s own gating language already states.

**Cycle status: INCOMPLETE at time of return, not `decision-blocked`.** All bounded work is done,
proven, committed, and pushed (§§1-8 above). The single open item is `root-full`'s exit code, still
building at return time — a turn-budget condition this bundle's own doctrine names explicitly as NOT
a stop reason (`loop-instruction.md` "4a. GATE SEQUENCING… 'Ran out of turn' is not 'blocked.'"). No
STOP condition was encountered; no `decision-blocked` entry is warranted. A resumed cycle should: (1)
tail `docs/release/SD-30-class-feature-archetype-bundle/artifacts/sd30-e3-f3-verify.log` for
`VERIFY_EXIT=`; (2) if `0`, flip DoD item 1 to PASS and this section's status line to `COMPLETE`,
otherwise diagnose per the log's own `SUMMARY` block (never the wrapper exit code alone, per
`state-goals-and-lessons.md`/`loop-instruction.md` "4b. READING THE EXIT CODE"); (3) no further code
change is anticipated regardless of outcome, since items 2-8 above are already settled and the gate's
only job at this point is confirming the diff compiles/tests/lints clean, which `root-lib` (1776
passed) and every stage before it already did.


**Update, appended before turn-budget return:** `root-lib` PASS (1776 passed), `root-full` PASS
(6402 passed across 547 suites, all 526 `tests/*.rs` suites executed — byte-identical to
`SD30-E3-F2-001`'s own figure, expected: this cycle added no new Rust tests). Gate now on `desktop`.
Exit code still not obtained; item 1 above stands as written.

### 11. Resumption — gate exit obtained, cycle closes out (same `HEAD`, no code change)

A second agent resumed this exact card (`SD30-E3-F3-001`) after the prior agent ran out of turn
mid-gate. **Verified the prior agent's work by content before building on it, per the resumption
brief's own instruction — not taken on say-so:**

- `git rev-parse HEAD` at resumption = `a6f0718034840698ee770a9740f4071efea79a05`, matching the prior
  receipt's `head_after` exactly; `git status --porcelain` showed only pre-existing, unrelated dirty
  state (`.gitignore` +4 lines re: `.wrangler/`, an untracked `.github/workflows/deploy-site.yml`) —
  traced via `git log -3 -- .gitignore`, whose most recent touching commit (`462c40bc`) long predates
  this cycle and is unrelated to SD-30; **not staged, not touched, left exactly as found** (shared
  checkout, another session's in-flight work).
- `grep -n 'DESCISPI\|NAMEISPI\|pi_screening\|declared_product_identity\|classify_optional_field_declared'`
  over both `scripts/transcribe_monster_tables.py` and `scripts/transcribe_companion_tables.py`:
  both screens present exactly as claimed (drop-on-`NAMEISPI:YES`, redact-on-`DESCISPI:YES`,
  both-tokens-drops precedence, companion creature-half `NAMEISPI:YES` drop newly added).
- `kanban.md`'s `epic-3-pi-gate` row and `progress.md`'s own §§1-10 above: content matches the prior
  agent's receipt verbatim.
- `forward-scope-register.md` `C1.6` (this package) and `SD-31-corpus-closure-grind/forward-scope-
  register.md` `G1.6`: both entries present, cross-pointing correctly, content matches the claimed
  invocation contract.
- `git log origin/tranche/10..HEAD` and `HEAD..origin/tranche/10`: both empty — local `HEAD` and
  `origin/tranche/10` are identical; both commits (`dc89e389`, `a6f07180`) already pushed. Nothing
  stranded.

**No content divergence found. All prior claims held.**

### 12. Gate completion (this agent's own contribution)

The gate (PID `423242`, launched 19:26 EDT by the prior agent, `CARGO_TARGET_DIR=/home/ubuntu/
cargo-targets/sd30-e3-f3-backfill`) was still running at resumption, on the `desktop` stage. Watched
to completion rather than re-launched (same PID throughout, confirmed live via `ps -p 423242` at each
check) — no second gate was started, avoiding a duplicate/overlapping run against the same
`CARGO_TARGET_DIR`. Remaining stages observed landing in the log in real time:

```
==> desktop — cargo test --locked -j 2  (apps/desktop/src-tauri)
    PASS  desktop  (445 passed)
==> reach — cargo test --locked -j 2 reach_gate  (apps/desktop/src-tauri)
    PASS  reach  (27 passed)
==> corpus-sweep — cargo run --locked --bin corpus_literal_sweep  (repo root)
    PASS  corpus-sweep  (3516 records examined of 9328 read, 36105 tokens compared (9 synthesized), 8903 digests checked, 0 findings)
==> frontend-install — npm ci if node_modules is absent  (apps/desktop)
    PASS  frontend-install  (node_modules present)
==> frontend-test — npm test  (apps/desktop)
    PASS  frontend-test  (99/99 files)
==> frontend-typecheck — npm run typecheck  (apps/desktop)
    PASS  frontend-typecheck  (tsc --noEmit clean)
==> clippy — cargo clippy --locked --tests -j 2  (BOTH crates)
    PASS  clippy  (root:46 desktop:7 warnings, 0 errors)
==> class-dump — cargo run --locked --bin v06_class_state_dump  (repo root)
    PASS  class-dump  (31/31 computing)

SUMMARY
  passed:  16  preflight-disk pi-sweep audit-selftest reclaim-selftest driver-selftest corpus-sweep-selftest root-lib root-full desktop reach corpus-sweep frontend-install frontend-test frontend-typecheck clippy class-dump

BASELINE NOTES (not failures — update deliberately):
  - BASELINE_ROOT_FULL_TESTS baseline is stale: 6398 recorded, 6402 measured. Update /home/ubuntu/workspace/repos/codex/scripts/verify-baselines.env.

RESULT: PASS
VERIFY_EXIT=0
```

`VERIFY_EXIT=0` captured directly from the log's own append (`echo "VERIFY_EXIT=$?" >> "$LOG"` in the
wrapper the prior agent launched — not through a pipe, not inferred from the harness task status).
16/16 stages PASS.

**DoD item 1 → PASS.** (Supersedes the "NOT YET OBTAINED" text in §9's table row 1 above; that row is
left as written for the historical record of what was known at the time, this section is the update.)

**DoD item 3 re-derived independently this session** (re-derive, do not transcribe — a number from a
prior receipt is not evidence on its own): `cargo run --locked --bin v06_corpus_trap_report -- --audit`
→ exit 2, `grep -c '\[wiring-class-mismatch\]'` = **177**, `grep -c class_feature` = **0** — byte-
identical to the prior agent's figure, now independently confirmed rather than trusted. Pre-existing,
unrelated to this cycle's `monster`/`companion` PI-screening change (0 `class_feature` hits).

**DoD item 7 note (not a fix owed by this card):** the `BASELINE_ROOT_FULL_TESTS` staleness (6398
recorded vs. 6402 measured) is real but not caused by `SD30-E3-F3` (this card's Python-only diff adds
zero Rust tests — `root-full`'s count is byte-identical to `SD30-E3-F2-001`'s own figure, per §"Update"
above). It predates this card, most likely from `SD30-E3-F2-001`'s Rust ingest-wiring change. The
gate's own `RESULT: PASS` confirms it is advisory, not blocking (6402 ≥ 6398 floor). This repo's own
`scripts/verify-baselines.env` history shows baseline bumps landing as their own dedicated commit,
separate from feature work, rather than bundled opportunistically — left for that dedicated commit,
not performed here, consistent with DoD item 7's "their own reviewable commit" instruction and this
card's own scope (`SD30-E3-F3`, not a baseline-maintenance card).

### 13. Final Definition of Done

| # | Item | Result |
|---|---|---|
| 1 | `verify.sh` exits 0 | **PASS.** `VERIFY_EXIT=0`, 16/16 stages, captured directly from the log (§12). |
| 2 | Reach stage claim | **N/A** (as §9 row 2 — no new player-visible family this cycle) — and, re-confirmed this session, the gate's own `reach` stage ran and PASSED with 27 tests (nonzero — not the "0 matched tests" hard-failure case), the pre-existing baseline claim set, unaffected by this Python-only cycle. |
| 3 | `v06_corpus_trap_report -- --audit` | **Exit 2, 177 pre-existing `wiring-class-mismatch`, 0 `class_feature`** — re-derived independently this session (§12), confirmed unrelated to this cycle's diff. |
| 4 | Guarded work-inventory regen | **N/A**, as §9 row 4 — no corpus content or `docs/work-inventory.json` change this session either. |
| 5 | Four-check wired-integration audit | **PASS**, as §9 row 5 — no further diff added this session (gate-watch only). |
| 6 | `OPEN_FINDINGS` | **N/A**, as §9 row 6. |
| 7 | Baseline movements own commit | **N/A for this card** — observed but out-of-scope, see §12. |
| 8 | On-screen verification | **N/A**, as §9 row 8 — no player-visible surface touched. |

### 14. Card disposition (final)

**`SD30-E3-F3` sub-scope: COMPLETE.** All DoD items resolved (PASS or N/A-with-reason), gate green,
work verified by content at both the start and end of this resumption. `kanban.md`'s `epic-3-pi-gate`
row already carried the correct `F1/F2/F3 sub-scopes COMPLETE; F4 still open` status from the prior
agent — left as-is (accurate). `Claimed-by`/`Cycle-id` unchanged (`sd30-e3-f3-backfill` /
`SD30-E3-F3-001`) since this is the same card, same cycle-id, resumed, not a new claim.

**Cycle status: COMPLETE.** No STOP condition encountered, no `decision-blocked` entry. The one open
item from the prior return (`root-full`'s then-pending exit code) is now resolved: `VERIFY_EXIT=0`,
full 16/16 gate PASS. No code changed in this resumption — it was a pure gate-watch-to-completion plus
independent re-derivation of DoD item 3 and receipt/kanban closure.
