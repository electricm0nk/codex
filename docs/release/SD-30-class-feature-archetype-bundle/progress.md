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
