---
canonical: true
owner: god-emporer
bundle_id: SD-35
status: planning-ready
date: 2026-09-07
authored_from: ../../governance/workflow-instruction-template.md
---

# SD-35 Workflow Instruction — Workflow-Orchestrated Dispatch

Authored from `../../governance/workflow-instruction-template.md`, not from a prior bundle's
copy. States the current dispatch procedure only.

## 0. Bundle at a glance

- **Branch:** `tranche/15` — **cut and pushed 2026-09-07** at `4c6c57eb9f` ("version bump 0.15.0"),
  directly on `fe5ae6cd4a`, the merge of SD-34's PR #383 to `develop` (`decisions.md §10`).
- **Board:** local-file `./kanban.md` (Hermes board retired 2026-08-01)
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop
- **Epics / criteria:** 7 / 30 (29 kanban rows)
- **First concrete build value:** `0.15.0`, stamped at the `tranche/15` cut in `4c6c57eb9f`
  (`decisions.md §10`)

## 1. Pre-launch checklist

Every command below was run for real at the `tranche/15` cut on 2026-09-07 (cut commit
`4c6c57eb9f`; run at HEAD `c15e64bc3e`, four docs-only litter commits above it, inventory
unchanged), with its output pasted under it. The first draft of this file (authored the same
morning, while SD-34 was at wave 51) marked items 1–10 "at the cut"; the cut landed that
evening and the launch-readiness audit re-ran every item.

1. **Board reachable.** `test -f docs/release/SD-35-corpus-sheet-completion/kanban.md && echo KANBAN_PRESENT`

   **Output:**
   ```
   KANBAN_PRESENT
   ```
2. **Version source of truth read.** Both of
   `python3 -c "import json;print(json.load(open('apps/desktop/package.json'))['version'])"`
   and the same for `apps/desktop/src-tauri/tauri.conf.json`. Expect `0.15.0`.

   **Output:**
   ```
   0.15.0
   0.15.0
   ```
3. **SD-34's closure PR merged to `develop`.** `gh pr view 383 --json state,mergedAt,mergeCommit`
   plus `git log origin/develop --oneline | head -3`. **Tier-1 launch gate.**

   **Output:**
   ```
   {"mergeCommit":{"oid":"fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47"},"mergedAt":"2026-09-07T22:26:24Z","state":"MERGED"}
   fe5ae6cd4a Merge pull request #383 from electricm0nk/tranche/14
   46cd0b56a6 docs: record the persistent `test` branch (allow-only-test-into-main gate; self-healing; never delete)
   358a71516f docs(sd34): record the recovered Monk Ki Pool census fix before deleting the wave-13 salvage branches
   ```
4. **SD-34's closure state recorded, and folded.** SD-34 merged (item 3) **without running its
   closure epilogue**: `test -f docs/retro/sd34-book-completion-retrospective.md` fails and
   `grep -cE '\| (not-started|in-progress|partial) \|' docs/release/SD-34-book-completion/kanban.md`
   counts its open rows. Operator ruling 2026-09-07 (`decisions.md §12`): the retrospective and
   the open rows are **folded into SD-35 as AT-35-E1-006**, not a precondition. This item's
   evidence is that ruling recorded, the open-row count captured, and SD-34's `## Open blockers`
   empty.

   **Output:**
   ```
   RETRO_ABSENT
   open_rows=17 of 37
   SD-34 ## Open blockers: 1 entry, `### AT-34-E3-001 — RESOLVED 2026-08-27 by orchestrator ruling` — no active entry
   decisions.md §12 recorded; AT-35-E1-006 on kanban.md row 6
   retro.py open deferrals since 2026-08-27: 29 (all SD-34-tagged) — AT-35-E1-006 dispositions each
   ```
5. **Working tree clean on the bundle branch.** `git status --porcelain`

   **Output:** the 38 entries present at the cut (33 SD-33/SD-34 dispatch scripts, 3 retro
   event logs, 2 audit notes — no code) were committed by operator ruling in `a14f178c16`,
   `1fd63ed93b`, `4c4d07f89d`, `c15e64bc3e`. At the moment of this run the only entries were
   this audit's own 13 package edits, committed together with this paste:
   ```
   13   (all docs/release/SD-35-corpus-sheet-completion/*.md — this audit's edits)
   0    after the audit commit
   ```
6. **Doctrine gates.** `ls docs/governance/no-stub-mvp-doctrine.md docs/doctrine-external/identifier-discipline.md docs/governance/blocker-closure-doctrine.md`

   **Output:**
   ```
   docs/doctrine-external/identifier-discipline.md
   docs/governance/blocker-closure-doctrine.md
   docs/governance/no-stub-mvp-doctrine.md
   ```
7. **Oracle pin present and readable.** `grep -E "^[A-Z_]+=" scripts/pcgen-oracle-pin.env`.
   **`~/workspace/repos/pcgen` is forbidden as an oracle path.**

   **Output:**
   ```
   PCGEN_ORACLE_REPO=https://github.com/PCGen/pcgen.git
   PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6
   PCGEN_ORACLE_SHA_DATE=2026-06-17            # informational: upstream commit date of the pin
   PCGEN_ORACLE_SPARSE_PATHS="data/pathfinder system/gameModes/Pathfinder"
   ```
8. **`tranche/15` cut from `develop` and pushed.** `git ls-remote --heads origin tranche/15`

   **Output:**
   ```
   c15e64bc3eb22ebe07b37776320d8f07894cf949	refs/heads/tranche/15
   (cut commit 4c6c57eb9f "feat(sd35): version bump 0.15.0 for tranche/15", on fe5ae6cd4a)
   ```
9. **Inherited instruments live.** `python3 scripts/completion_atlas.py --check` exits 0 with
   `unclassified=0 overlap=0 citation_failures=0`; `python3 scripts/shape_engine_boundary.py --check`
   and `python3 scripts/missing_engine_tables.py --check` exit 0 (both were stale at HEAD before
   wave 51 — run them, do not assume); `python3 scripts/box_ledger.py --check` exits 0;
   `cargo run --locked --bin corpus_literal_sweep` 0 findings.

   **Output (run at `4c4d07f89d`, one litter commit below `c15e64bc3e`; inventory identical):**
   ```
   completion_atlas.py --check:   population=49438 buckets=10 unclassified=0 overlap=0
                                  DONE 26123 / A 449 / B 11589 / C 4180 / D 1982 / M 4334 / V 392 / U 202 / X 168 / Z 19
                                  done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0  EXIT=0
   shape_engine_boundary.py:      magnitude_bearing=26396 not_held_by_engine=8784 citation_ok=True  EXIT=0
   missing_engine_tables.py:      population=449 kinds=2 (companion 28, power 421) citation_failures=0  EXIT=0
   test_shape_engine_boundary.py: Ran 12 tests OK
   verify.sh --only denominator-gate:  RESULT: PASS
   verify.sh --only figure-provenance: RESULT: PASS
   box_ledger.py --check:         EXIT=1 — uncovered=27502 overlap=0 population=49438; eight "THE-BOX.md needs
                                  re-deriving" warnings (it pins SD-33's status vocabulary, e.g. not-ingested=26,002;
                                  live not-ingested=0 since SD-34 AT-34-E1-005's rename). RETIRED as a standing gate
                                  by decisions.md §13 — the atlas is the partition. NOT a launch blocker.
   retro.py summary --since 2026-08-27: open deferrals = 29 (SD-34 recorded 3 at its cut; the other 26 are
                                  SD-34 wave deferrals) — enumerated and dispositioned by AT-35-E1-006.
   corpus_literal_sweep:          CLEAN — 48706 of 51476 records examined, 0 findings (item 10)
   ```
10. **Widest build scope green** (SD-34 `decisions.md §10`): `cargo test --locked --no-run`
    exits 0; `cargo test --locked --no-fail-fast -j 6` with targets executed counted;
    `cd apps/desktop/src-tauri && cargo test --locked`. Re-derive the inherited failing-suite
    set at the cut and record it as SD-35's baseline (`technical-requirements.md §3`). **Also
    record the cold `cargo test --locked --no-run` wall time** — AT-35-E1-003's "before".

    **Output (run at `4c4d07f89d`, `CARGO_TARGET_DIR=/tmp/cargo-sd35-launch` emptied first, `CARGO_INCREMENTAL=0`, `-j 6`, 24 cores / 143 GB, load 0.4 at start):**
    ```
    cargo test --locked --no-run -j 6  (cold)     Elapsed (wall clock) 2:44.51   Max RSS 2,209,604 kB   EXIT=0
    cargo test --locked --no-fail-fast -j 6        590 targets executed, 8656 passed, 0 failed, 0 failing suites   (22:53Z → 00:03Z, 70 min)
    cd apps/desktop/src-tauri && cargo test        573 passed, 0 failed, 0 ignored (89 s)   DESKTOP_EXIT=0
    cargo run --locked --bin corpus_literal_sweep  48706 records examined of 51476 read, 413314 tokens compared, 0 findings, CLEAN   SWEEP_EXIT=0
    ```
    **SD-35's inherited test baseline is therefore EMPTY**: the 29 of 599 suites / 46 of 8,034
    tests SD-33 and SD-34 carried are all green at the cut (SD-34's waves fixed them; 590 targets
    now, 11 obsolete probe bins deleted by the fable review). A failure anywhere in the workspace
    from here on is SD-35's. **The cold build is 2 min 45 s on this box** — the "30-minute build"
    in the operator's complaint was the full-suite wall time (70 min) plus the per-cycle gate,
    not the compile; AT-35-E1-003's target is the 70-minute figure as much as the compile.
11. **Artifact directories exist, one per epic**, each with a `.gitkeep`. See `artifacts/README.md`.

    **Output (2026-09-07, authoring):**
    ```
    epic-1-tax-cut/: .gitkeep
    epic-2-sheet-rule/: .gitkeep
    epic-3-place-and-surface/: .gitkeep
    epic-4-resolve-and-verify/: .gitkeep
    epic-5-residues/: .gitkeep
    epic-6-pcgen-exit/: .gitkeep
    epic-7-closure/: .gitkeep
    ```
12. **Denominator gate pointed at THIS package.**
    `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md'`
    → `files_checked=N violations=0`.

    **Output (authoring at `5f6b18f4e3`, and again at the cut, `c15e64bc3e`, after the audit's edits):**
    ```
    files_checked=16
    violations=0
    ```

## 2. Orchestration mode

- **Dispatch mechanism:** the in-harness `Workflow` tool from a live session. Not `/loop /batch`.
- **Orchestrator model:** **Opus** — the live session that launches this script runs on Opus
  (`/model opus` before launch; the authoring session was Fable).
- **Lane model (operator ruling 2026-09-07, `decisions.md §14`): `fable` on every dispatched
  lane until the Fable quota runs dry, then `opus`.** The script carries one constant,
  `LANE_MODEL`, set from `args.laneModel` at launch. When `agent()` returns `null` on a terminal
  API error (the quota-exhausted shape), the script halts with the criterion id; the
  orchestrator rebuilds the criteria lists from `kanban.md` (complete rows excluded) and
  relaunches with `args.laneModel = 'opus'`. Housekeeping steps (release notes, version
  confirmation) stay on Haiku. **Set `model` on every `agent()` call** — an omitted `model`
  inherits the orchestrator's.
- **Concurrency shape:** fixed per epic in §3, at authoring time. **Never more than 3 lanes
  building at once** (`decisions.md §9` L11) — the read-only worker below counts as one.
- **Worker split (operator, 2026-09-07, `decisions.md §3`):** cycle work runs **local, warm, and
  visible** — the shared checkout, the cycle agent's own `CARGO_TARGET_DIR`. The two long
  read-only jobs run on an **isolated worker** that **pushes nothing**: the epic-end
  `scripts/verify.sh` gate (§10 step 0) and every oracle-harness run (AT-35-E2-005, E4-001's
  per-cycle parity, E4-002, E6-001/E6-004 parity). The worker is an `agent()` with
  `isolation: 'worktree'` on this box; a cloud session may stand in when the box is loaded,
  under the same rule. It returns its outputs (gate log path, parity JSON, receipt text) to the
  orchestrator, which hands them to the next cycle agent to commit. **Two writers on one branch
  is what this rule prevents** — the worker never commits.

### 2.1 Agent environment setup

```bash
export RETRO_ACTOR="<lane-role-name>"
export CARGO_TARGET_DIR="/tmp/cargo-sd35-<lane-role-name>"
export CARGO_INCREMENTAL=0
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
```

### 2.2 Execution boundary — the launching session is the orchestrator, never the executor

§6's steps happen **inside a dispatched `agent()`/`Workflow` call**. The orchestrating
session's own `Edit`/`Write`/`Bash` calls are reserved for read-only investigation and for this
bundle's planning docs — **never** for `src/`, `scripts/`, `apps/`, `tests/`, or `data/`.

Discovering a cycle's real scope mid-investigation is a reason to **re-dispatch with the
corrected scope** — and to re-run `cycle_scope_gate.py` on it — never to fix inline.

**Corollary:** `kanban.md` rows and `progress.md` entries are written **inside the dispatched
agent** (§6 steps 7–8), never from the orchestrating session's own `Bash`/`Edit` calls.

**Dispatch first, report second.** A finished wave with no next dispatch is a stall.

**Do not dispatch two lanes at one diagnosis.**

### 2.3 Retrospective event logging (every cycle)

Emit via `scripts/retro.py` **at the moment it happens**. `--verified-by` is required on a
`correction`. Every cycle that closes fewer units than its scoped population emits a
`deferral`-typed event naming the refused token type(s) and counts — that is how the
retrospective's "what was left, by mechanism" section is grounded.

### 2.4 Creating the Workflow script

```javascript
export const meta = {
  name: 'sd-35-dispatch',
  description: 'SD-35 — every remaining unit to a rendered sheet line, no PCGen on the live side; one mechanism per cycle, 500 minimum, one build',
  phases: [
    { title: 'Epic 1 — Tax cut' },
    { title: 'Epic 2 — Sheet rule' },
    { title: 'Epic 3 — Place and surface' },
    { title: 'Epic 4 — Resolve and verify' },
    { title: 'Epic 5 — Residues' },
    { title: 'Epic 6 — PCGen exit' },
    { title: 'Epic 7 — Closure epilogue' },
  ],
}

// BUILT AT LAUNCH by the orchestrating session, not defined in this file:
//   epic1Criteria..epic6Criteria = [{id, scope, prompt}, ...] from epic-breakdown.md — `scope` is
//     the cycle_scope_gate.py argument string; `prompt` embeds §6 verbatim + the criterion text.
//   cycleProcedurePrompt(c, i)      — §6 + criterion c, cycle index i
//   withRemainderScope(c, refused)  — c with its scope widened to the refused-token remainder,
//                                     bundled with the criterion's next family until the floor passes
//   epicWrapUpPrompt(title)         — §10 steps 0–3 for that epic
//   finalAcceptanceScanPrompt()     — acceptance-and-verification.md §3 + §3a
//   retrospectiveAndSweepPrompt()   — §11 steps 2–3
//   architectureDocsGraphifyPrPrompt() — §11 step 4 (template.md §6)
//   releaseNotesVersionBumpPrompt() — §11 step 5

const LANE_MODEL = (args && args.laneModel) || 'fable'   // 'fable' until dry, then relaunch with args.laneModel='opus' (decisions.md §14)

const CYCLE_SCHEMA = {
  type: 'object',
  required: ['criterion', 'status', 'commit_sha', 'scope_gate_output', 'receipt_output', 'receipt_path'],
  properties: {
    criterion: { type: 'string' },
    status: { type: 'string', enum: ['complete', 'partial', 'blocked-escalated'] },
    commit_sha: { type: 'string' },
    scope_gate_output: { type: 'string' },   // literal cycle_scope_gate.py --min 500 line, verdict=PASS*
    receipt_output: { type: 'string' },      // literal cycle_scope_gate.py --receipt rows (incl. pcgen_live_files)
    receipt_path: { type: 'string' },
    refused_tokens: { type: 'string' },      // "<type>=<count>, ..." or "none"
  },
}
// `!r` (null) is also the quota-exhausted shape: agent() returns null after a terminal API
// error. The halt names the criterion so the orchestrator can relaunch on the next tier.
const halted = r => !r || r.status === 'blocked-escalated'

// Wrap-up and closure agents are not per-criterion cycles: no scope gate, but the residue line
// and a receipt are still required.
const WRAP_SCHEMA = {
  type: 'object',
  required: ['status', 'residue_line', 'receipt_path'],
  properties: {
    status: { type: 'string', enum: ['complete', 'blocked-escalated'] },
    residue_line: { type: 'string' },        // literal pcgen_residue_gate.py --check line
    receipt_path: { type: 'string' },
    commit_sha: { type: 'string' },
    notes: { type: 'string' },
  },
}

// A `partial` result re-dispatches the SAME criterion with the refused-token remainder as its
// scope — the loop below continues until the criterion's population is zero or a cycle halts.
async function runCriterionToZero(title, c, maxCycles = 12) {
  for (let i = 0; i < maxCycles; i++) {
    const r = await agent(cycleProcedurePrompt(c, i), { model: LANE_MODEL, phase: title, schema: CYCLE_SCHEMA })
    if (halted(r)) return { halted: c.id, result: r }
    if (r.status === 'complete') return null
    c = withRemainderScope(c, r.refused_tokens)   // next cycle: the named remainder, still ≥ 500 or whole-remainder
  }
  return { halted: c.id, result: 'max cycles reached — escalate' }
}

// The epic-end gate (§10 step 0) runs on an ISOLATED, READ-ONLY worker and OVERLAPS the next
// epic's first cycle (decisions.md §3): it is started, not awaited, when an epic's criteria are
// done, and awaited before the next epic's SECOND cycle. A red gate stops the bundle there — at
// most one cycle of rework is the accepted cost of the overlap.
let pendingWrap = null
async function runEpic(title, criteria) {
  phase(title)
  let first = true
  for (const c of criteria) {
    const h = await runCriterionToZero(title, c)
    if (h) return h
    if (first && pendingWrap) {                       // previous epic's gate must be green before cycle 2
      const wrap = await pendingWrap; pendingWrap = null
      if (halted(wrap)) return { halted: `${wrap.epic} wrap-up gate`, result: wrap }
    }
    first = false
  }
  if (pendingWrap) {                                  // a one-criterion epic: still await before leaving
    const wrap = await pendingWrap; pendingWrap = null
    if (halted(wrap)) return { halted: `${wrap.epic} wrap-up gate`, result: wrap }
  }
  // Epic wrap-up (§10): full verify.sh ONCE, retro summary, worktree sweep — on the worker, pushes nothing.
  pendingWrap = agent(epicWrapUpPrompt(title), { model: LANE_MODEL, phase: title, isolation: 'worktree', schema: WRAP_SCHEMA })
    .then(r => Object.assign(r || { status: 'blocked-escalated' }, { epic: title }))
  return null
}

let h
// Epic 1: three file-disjoint lanes (§3) — E1-003 (tests/) ‖ E1-001+E1-002+E1-005 (scripts/) ‖
// E1-006 (docs/retro + two references/README.md; SD-34's fold). 3 lanes = the cap.
phase('Epic 1 — Tax cut')
const e1 = await parallel([
  () => agent(cycleProcedurePrompt(epic1Criteria.find(c => c.id === 'AT-35-E1-003')), { model: LANE_MODEL, phase: 'Epic 1 — Tax cut', isolation: 'worktree', schema: CYCLE_SCHEMA }),
  () => pipeline(epic1Criteria.filter(c => ['AT-35-E1-001', 'AT-35-E1-002', 'AT-35-E1-005'].includes(c.id)),
                 c => agent(cycleProcedurePrompt(c), { model: LANE_MODEL, phase: 'Epic 1 — Tax cut', isolation: 'worktree', schema: CYCLE_SCHEMA })),
  () => agent(cycleProcedurePrompt(epic1Criteria.find(c => c.id === 'AT-35-E1-006')), { model: LANE_MODEL, phase: 'Epic 1 — Tax cut', isolation: 'worktree', schema: CYCLE_SCHEMA }),
])
if (e1.flat().some(halted)) return { halted: 'Epic 1', result: e1 }
if ((h = await runEpic('Epic 1 — Tax cut', epic1Criteria.filter(c => c.id === 'AT-35-E1-004')))) return h
if ((h = await runEpic('Epic 2 — Sheet rule', epic2Criteria))) return h
if ((h = await runEpic('Epic 3 — Place and surface', epic3Criteria))) return h
if ((h = await runEpic('Epic 4 — Resolve and verify', epic4Criteria))) return h
if ((h = await runEpic('Epic 5 — Residues', epic5Criteria))) return h
if ((h = await runEpic('Epic 6 — PCGen exit', epic6Criteria))) return h   // oracle parity before + after; gate --closure at E6-004

phase('Epic 7 — Closure epilogue')
if (pendingWrap) {                                    // Epic 6's gate must be green before the scan
  const wrap = await pendingWrap; pendingWrap = null
  if (halted(wrap)) return { halted: `${wrap.epic} wrap-up gate`, result: wrap }
}
const SCAN_SCHEMA = { type: 'object', required: ['gate', 'status'], properties: {
  gate: { type: 'string', enum: ['PASS', 'FAIL'] }, status: { type: 'string' }, short: { type: 'string' } } }
const scan = await agent(finalAcceptanceScanPrompt(), { model: LANE_MODEL === 'fable' ? 'fable' : 'opus', phase: 'Epic 7 — Closure epilogue', schema: SCAN_SCHEMA })
if (!scan || scan.gate !== 'PASS') return { halted: 'AT-35-E7-001', result: scan }
// §11 order is load-bearing: retro + sweep BEFORE the PR. Every step is checked; a blocked step
// stops the epilogue — the PR never opens over a failed sweep (blocker-closure-doctrine.md).
const retro = await agent(retrospectiveAndSweepPrompt(), { model: LANE_MODEL, phase: 'Epic 7 — Closure epilogue', schema: WRAP_SCHEMA })
if (halted(retro)) return { halted: 'AT-35-E7-002', result: retro }
const pr = await agent(architectureDocsGraphifyPrPrompt(), { model: LANE_MODEL, phase: 'Epic 7 — Closure epilogue', schema: WRAP_SCHEMA })
if (halted(pr)) return { halted: 'AT-35-E7-003 (arch docs / graphify / PR)', result: pr }
const notes = await agent(releaseNotesVersionBumpPrompt(), { model: 'haiku', phase: 'Epic 7 — Closure epilogue', schema: WRAP_SCHEMA })
if (halted(notes)) return { halted: 'AT-35-E7-003 (release notes)', result: notes }
return { closed: 'claimed — verify against the repo before relaying (decisions.md §9 L7)' }
```

**Oracle runs are the same shape**: `cycleProcedurePrompt` for AT-35-E2-005, E4-001, E4-002,
E6-001 and E6-004 tells the cycle agent to dispatch the harness to a worker (`agent(...,
{ isolation: 'worktree' })` from the orchestrator, or a cloud session) and to commit the
returned parity JSON itself; the worker pushes nothing.

**The gate-failure check reads the scan's own JSON fields, never a substring.**
**`partial` is a legitimate cycle outcome** (SD-34 `decisions.md §15`): it names the remainder
by refused token type and the next cycle takes it. **`partial` is never allowed to shrink the
next cycle under the floor** — `withRemainderScope` bundles the remainder with the criterion's
next family until `cycle_scope_gate.py` passes.

## 2.5 A dispatched agent is never resumed — never end a turn waiting

A dispatched agent gets **exactly one turn**. Put this in every dispatch prompt:

- Wait for slow work **inside** the turn — foreground it, or poll a background job in a loop.
- **Scope test runs.** Name the targeted binaries plus the workspace suites; `apps/desktop/src-tauri`
  is a **separate cargo workspace** and is tested at epic cadence (`decisions.md §3`) unless the
  cycle touched it.
- **Measure before a population-scoped run.** AT-35-E2-005's corpus-wide pass timing is the
  standing measurement; a cycle re-running the pass states the projected wall time first.
- If something will not finish, **report what was observed and commit the work anyway.**
- **Commit and push before ending the turn**, always.

**Orchestrator's side:** never accept a lane's final message as evidence. Check `git log`, the
target files, and the `cycle_scope_gate.py --receipt` rows re-run against the committed
inventories.

## 3. Per-epic parallel/sequential map

| Epic | Criteria | Parallel? | File-touch set (verified §4) | Gated on |
|---|---|---|---|---|
| 1 Tax cut | E1-001..006 | **yes, 3 lanes**: E1-003 (`tests/`, `scripts/verify-baselines.env`) ‖ E1-006 (`docs/retro/sd34-book-completion-retrospective.md` new, `../SD-34-book-completion/references/README.md`, `references/README.md`, `decisions.md` — docs only) ‖ E1-001+E1-002+E1-005 (`scripts/cycle_scope_gate.py` new, `scripts/pcgen_residue_gate.py` new, `scripts/pcgen-residue-baseline.env` new, their tests, `scripts/completion_atlas.py`, `scripts/shape_engine_boundary.py`, `scripts/missing_engine_tables.py`, `scripts/tests/test_shape_engine_boundary.py`, `scripts/verify.sh`, `scripts/verify-baselines.env` — **the baselines file is the one collision**: E1-003 changes the binary/test floor, E1-002/E1-005 the stage count; the second lane to land rebases and re-derives). E1-004 after all three. | launch gates |
| 2 Sheet rule | E2-001..005 | no | **converter side:** `src/bin/sheet_rule_convert.rs` (new), `src/pcgen_import/sheet_rule/` (new), `data/sheet_rules/**` (new, generated), `scripts/token_coverage.py` (new) + test, `src/bin/v06_work_inventory.rs` (status + rung), `scripts/completion_atlas.py`, every status consumer; **live side:** `src/rules_core/sheet_rule.rs` (new — no PCGen), `src/rules_core/mod.rs`, `src/rules_core/corpus_loader.rs` (load `data/sheet_rules/`), `src/rules_core/pilot_compute/mod.rs` (the `SheetLine` on the computation result), `apps/desktop/src/characterHub/CharacterSheet.tsx` + tests, `apps/desktop/src-tauri/src/reach_gate.rs`; `docs/work-inventory.json` | Epic 1 |
| 3 Place and surface | E3-001..004 | no | `src/pcgen_import/sheet_rule/` (`applies` derivation), `src/rules_core/class_feature_pool_catalog.rs` (holdings lookup over `applies`), `src/bin/v06_work_inventory.rs` (C rung), `data/sheet_rules/**`, `docs/work-inventory.json`, `artifacts/epic-3-place-and-surface/` | Epic 2 |
| 4 Resolve and verify | E4-001..003 | no | `src/pcgen_import/sheet_rule/` (mapping rows), `data/sheet_rules/**`, `scripts/oracle_harness/`, `docs/work-inventory.json`, `artifacts/epic-4-resolve-and-verify/` | Epic 3 |
| 5 Residues | E5-001..005 | no | `src/rules_core/` (power/companion loaders over `applies`, class chassis), `apps/desktop/src-tauri/src/class_feature_pool_picker.rs`, `apps/desktop/src/characterHub/LevelUpDialog.tsx`, `data/corpus/beginner_box/**` (guarded generator only), `data/sheet_rules/**`, `docs/work-inventory.json`, `artifacts/epic-5-residues/` | Epic 4 |
| 6 PCGen exit | E6-001..004 | no | `src/rules_core/pilot_compute/formula_interpreter*.rs`, `bonus_stack_reader.rs`, `pre_tokens.rs`, `racial_sla.rs`, `trait_effects.rs`, `feat_effects.rs`, `wiring_class.rs`, `src/rules_core/cache_gen/**` → `src/pcgen_import/**`; every live `PcgenFormulaEvaluator` / `render_pcgen_desc` / `raw_tokens` call site (78 files by coarse grep); `apps/desktop/src-tauri/src/*_catalog.rs`, pickers, `class_feature_feat_bridge.rs`, `reach_gate.rs`; `src/bin/gen_*`, `src/bin/enrich_*` import paths; `artifacts/epic-6-pcgen-exit/` | Epic 5 |
| 7 Closure epilogue | E7-001..003 | no | package docs, `receipts.md`, `release-notes.md`, `docs/architecture/`, `docs/retro/` | Epics 1–6 |

**Epics 2–6 are sequential because every one of them writes `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json`, or `src/rules_core/`** (`decisions.md §9` L12). Within an epic, cycles are sequential
for the same reason. Concurrency in this bundle comes from **batch size**, not lane count.

**Every parallel agent gets `isolation: 'worktree'`.** Never more than 3 lanes building at once.

## 4. File-touch verification

**Run for real 2026-09-07 at `tranche/14` `5f6b18f4e3` (authoring) and re-run at the `tranche/15`
cut `4c6c57eb9f` (launch-readiness audit) — identical results, plus the three paths the first
run had not checked:**

```
EXISTS   src/bin/v06_work_inventory.rs
EXISTS   src/rules_core/mod.rs
EXISTS   src/rules_core/wiring_class.rs
EXISTS   src/rules_core/feat_prereqs.rs                    (module file)
EXISTS   src/rules_core/feat_prereqs/pre_tokens.rs        (the first draft listed a flat path that does not exist)
EXISTS   src/rules_core/class_feature_pool_catalog.rs
EXISTS   apps/desktop/src-tauri/src/class_feature_pool_picker.rs
EXISTS   scripts/fetch-pcgen-oracle.sh
EXISTS   src/rules_core/feat_effects.rs
EXISTS   src/rules_core/trait_effects.rs
EXISTS   src/rules_core/race_resolver.rs
EXISTS   src/rules_core/racial_sla.rs                       (wave 51; its formula is one renderer arm's test case)
EXISTS   src/rules_core/pilot_compute/mod.rs
EXISTS   src/rules_core/pilot_compute/formula_interpreter.rs
EXISTS   src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs
EXISTS   src/rules_core/pilot_compute/bonus_stack_reader.rs
EXISTS   src/rules_core/pilot_compute/race_trait_formula_binding.rs
EXISTS   apps/desktop/src/characterHub/CharacterSheet.tsx   ("Not computed" lane at :2167)
EXISTS   apps/desktop/src/characterHub/LevelUpDialog.tsx
EXISTS   apps/desktop/src-tauri/src/reach_gate.rs
EXISTS   scripts/verify.sh                                  (ALL_STAGES at :110, 40 stages)
EXISTS   scripts/verify-baselines.env
EXISTS   scripts/completion_atlas.py
EXISTS   scripts/shape_engine_boundary.py
EXISTS   scripts/missing_engine_tables.py
EXISTS   scripts/denominator_gate.py                        (BUNDLE_DIR at :101 and DEFAULT_GLOBS at :121 still point at SD-33 — never advanced to SD-34; AT-35-E1-004 widens to SD-33 + SD-34 + SD-35)
EXISTS   scripts/tests/test_completion_atlas.py
EXISTS   scripts/tests/test_shape_engine_boundary.py
EXISTS   scripts/tests/test_denominator_gate.py
EXISTS   scripts/oracle_harness/run.py
EXISTS   scripts/pcgen-oracle-pin.env
EXISTS   scripts/retro.py
EXISTS   docs/work-inventory.json
EXISTS   docs/architecture/rules-data-tables.md             (wiring_class at :447-463)
EXISTS   docs/architecture/status.md
EXISTS   src/pcgen_import/                                  (mod.rs, ir_converter.rs, lst_parser/, corpus_traps.rs — the converter side)
EXISTS   src/rules_core/cache_gen/                          (generators on the WRONG side; relocate in AT-35-E6-002)
EXISTS   src/rules_core/corpus_loader.rs
EXISTS   src/oracle_validation/
EXISTS   data/corpus/  data/class_feature_grants/  data/stubs/
free     data/sheet_rules/                                  (Epic 2 deliverable, generated, no collision)
free     src/rules_core/sheet_rule.rs                       (Epic 2 deliverable, no collision)
free     src/pcgen_import/sheet_rule/                       (Epic 2 deliverable, no collision)
free     src/bin/sheet_rule_convert.rs                      (Epic 2 deliverable, no collision)
free     scripts/cycle_scope_gate.py                        (Epic 1 deliverable, no collision)
free     scripts/pcgen_residue_gate.py                      (Epic 1 deliverable, no collision)
free     scripts/pcgen-residue-baseline.env                 (Epic 1 deliverable, no collision)
free     scripts/token_coverage.py                          (Epic 2 deliverable, no collision)
free     scripts/tests/test_cycle_scope_gate.py
free     scripts/tests/test_pcgen_residue_gate.py
free     scripts/tests/test_token_coverage.py
```

**Re-run this block at launch**, not at authoring time. Known hazards, all previously hit: a
shallow glob lies under `data/corpus/`; a status field's name is not its meaning; `find -newermt`
lies on this box.

## 5. Concurrent-write protocol

```bash
git fetch origin tranche/15 && git rebase origin/tranche/15 && git push origin HEAD:tranche/15
```

Retry up to 5 times on non-fast-forward. **Never force-push.** **SD-35's shared files are
six:** `progress.md` (prepend-only), `kanban.md` (one row per criterion, plus one per extra
cycle), `docs/work-inventory.json` (regenerated once per cycle, sequentially),
`data/sheet_rules/**` (regenerated whole by the converter, sequentially),
`artifacts/epic-2-sheet-rule/token-coverage.json` (re-derived, never appended), and
`scripts/pcgen-residue-baseline.env` (rewritten only by `--rebaseline` after a reduction).

**`git status --porcelain` before EVERY git write. Never `git add -A`. Never `git stash`.**

## 6. Per-cycle procedure

Runs **inside a dispatched agent** (§2.2). Steps 0 and 3 are where SD-35 differs from SD-34.

0. **Rebase first, then verify the base is real, then pin the cycle's start.** The batch is
   proved against the rebased tree — a scope gate or a "before" inventory taken before the
   rebase measures a tree that no longer exists.
   ```bash
   git fetch origin tranche/15 && git rebase origin/tranche/15   # §5
   test -d docs && test -d data && test -d scripts \
     || { echo 'WRONG BASE — reset before continuing'; exit 1; }
   CYCLE_START_SHA=$(git rev-parse HEAD)
   ```
1. **Prove the batch and check the residue baseline before touching anything.**
   ```bash
   python3 scripts/cycle_scope_gate.py --min 500 <this cycle's scope flags> \
     || { echo 'UNDER FLOOR — bundle more mechanisms or take the whole remainder'; exit 1; }
   python3 scripts/pcgen_residue_gate.py --check \
     || { echo 'PCGEN RESIDUE ABOVE BASELINE AT START — the tree is already wrong; stop'; exit 1; }
   cp docs/work-inventory.json /tmp/wi-before-$RETRO_ACTOR.json
   ```
   **Floor exemption** (`decisions.md §2`, and SD-34 `workflow-instruction.md §12` row 6):
   gate-building cycles (Epic 1 rows 1, 2, 5, 6), epic wrap-up fix cycles, and Epic 6 cycles
   close zero units by design and skip the scope gate, writing `SCOPE_GATE: EXEMPT (<reason>)`
   in the receipt instead. **Nothing is exempt from the residue check.**
   The literal output line goes in the receipt. A cycle that cannot pass the floor does not
   start; it reports the scoped population and stops (a correct outcome — the orchestrator
   re-scopes).
2. Define the audit base once, then run both greps:
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)

   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
   ```
   `<scoped paths>` is this epic's file-touch set from §3, space-separated.
3. **Do ALL the cycle's work, then verify ONCE** (`decisions.md §3`):
   - Write every converter mapping row / `applies` widening / classifier change the scope needs
     — **on the converter side**. Nothing new on the live side reads a PCGen token. Write the
     per-kind converter gate, not per-unit fixtures (`decisions.md §4`).
   - Regenerate `data/sheet_rules/` once: `cargo run --locked --bin sheet_rule_convert`, then
     `-- --check`. Then the inventory once, guarded: `corpus_literal_sweep` (if corpus records
     changed) then `cargo run --locked --bin v06_work_inventory`. Never `--allow-stamp-loss`.
   - If the cycle added a `Number` mapping, run the oracle comparison on the fixture roster and
     name every disagreement.
   - Run, in this order, after the last figure-moving commit:
     ```bash
     cargo test --locked --no-run -j 6 ; echo NO_RUN_EXIT=$?
     cargo test --locked --lib -j 6
     cargo test --locked --no-fail-fast -j 6            # required when src/ or the classifier changed
     cargo run --locked --bin corpus_literal_sweep      # only when corpus records changed; 0 findings
     python3 scripts/pcgen_residue_gate.py --check      # must not have risen; --closure from AT-35-E6-004
     cargo run --locked --bin sheet_rule_convert -- --check   # from AT-35-E2-001 onward
     grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   # must print 0
     python3 scripts/completion_atlas.py --check
     python3 scripts/token_coverage.py --check          # from AT-35-E2-004 onward
     python3 scripts/shape_engine_boundary.py --check
     python3 scripts/missing_engine_tables.py --check
     python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'
     python3 scripts/denominator_gate.py --check-provenance   # DIFFERENT flag from --check above; nonzero exit BLOCKS the push
     ./scripts/publish-site-dashboard.sh --check-pin          # nonzero exit BLOCKS the push; run the script without the flag and commit the feed
     scripts/verify.sh --only pi-sweep
     ```
     **Why both `denominator_gate.py` lines are here, and why neither substitutes for the other:**
     `--check` and `--check-provenance` are different checks. `--check` scans for unsourced
     figures the gate can reach by path; `--check-provenance` is the flag `verify.sh`'s
     `figure-provenance` stage runs (no path arguments — it uses its own default paths), and it
     enforces that every figure *inside a "Figures + their re-derive commands" section* carries a
     re-derive command **on its own line**. Until 2026-09-09 only `--check` ran per-cycle, so a
     `--check-provenance` violation could not be caught by any cycle and instead surfaced only at
     the ~90-minute epic wrap-up gate — which is exactly how the Epic 2, Epic 3 and Epic 4
     wrap-ups each went red on `figure-provenance` (incident key
     `figure-provenance-command-on-next-line`, 3 occurrences). This line is the mechanical control
     for that key: it is Python only, needs no build, and runs in seconds.
     **Why `publish-site-dashboard.sh --check-pin` is here, and why it is not the full `--check`:**
     the same shape, one incident key later. `site/dashboard/PF1e-dashboard.json` is derived from
     `docs/work-inventory.json`, and a cycle that regenerates the inventory without republishing
     the feed leaves the two diverged. The full `--check` catches that correctly but costs ~15
     minutes of real producer time (measured 904 s, 2026-09-10, at HEAD `00e44eee02`), so it only
     ever ran at the ~90-minute epic wrap-up — by which point the offending cycle had pushed. That
     is incident key `site-dashboard-json-stale-after-inventory-move`, 3 firings and 7 failing runs
     of the `site-dashboard-check` stage, and the disposition every time was "regenerate it in the
     wrap-up correction cycle" — a chore, which `AGENTS.md` rule 8 says is not a control.
     `--check-pin` re-hashes that one input against the pin a real publish records in
     `site/dashboard/inventory-pin.json`: milliseconds, no producer, no build, so it belongs in
     every cycle's push gate. **It does not replace the full check.** The pin watches one input; a
     feed made stale by a unit-ledger or owner-state change hashes clean here and is caught only by
     `site-dashboard-check`. Both stages are in `verify.sh` (`site-dashboard-pin` beside it) for
     exactly that reason.
     **Note the gate checks that a command is present and resolvable, not that it runs.** After
     adding or editing a figure row, actually execute its command and confirm it prints the value
     you wrote; a green gate over a broken command is the failure rule 9 exists to prevent.
     The desktop crate and frontend run here **only if the cycle touched `apps/`**; otherwise
     they run at the epic wrap-up (§10). `cargo clippy --locked --tests -j 6` on the touched
     targets; fix warnings in the same cycle.
   - If a count changed (test binaries, records, stages), grep old **and** new numbers across
     `tests/`, `src/`, `apps/`, `scripts/` before commit.
4. Re-run both audits on the final diff.
5. **Compute the receipt rows mechanically:**
   ```bash
   python3 scripts/cycle_scope_gate.py --receipt --since "$CYCLE_START_SHA" \
     --before /tmp/wi-before-$RETRO_ACTOR.json --after docs/work-inventory.json
   ```
   (`--since` bounds the `git diff --stat` that produces `rust_lines_changed` to this cycle.)
   Write the receipt to `artifacts/<epic-dir>/<criterion-id>_cycle<N>_receipt.md` (§7).
6. Commit, push via §5.
7. Update `progress.md` and `kanban.md` in place via §5. **Row hygiene:** the kanban Notes
   column is a pointer, never a story.
8. **Set your status from the receipt rows** (SD-34 `decisions.md §4`): `complete` only when
   the criterion's population is zero at HEAD; `partial` with the refused-token remainder named
   and summing; `blocked-escalated` only for §8's non-self-healable list.
9. Report: criterion, scope-gate line, files touched, SHAs, audit results, receipt rows,
   refused tokens, receipt path, next-cycle scope.

**Never hand-edit `data/corpus/**`.** **Never write a fixture whose expected value is
hand-derived for one record** — a per-kind gate that reads the live corpus directory is the
shape (`decisions.md §4`).

## 7. Per-cycle receipt schema

```markdown
# Cycle <cycle-id> — <epic-name> / <criterion-id>

- **Commit SHA:** <sha>
- **Scope gate:** <literal `cycle_scope_gate.py --min 500 ...` output line — verdict=PASS or PASS_WHOLE_REMAINDER>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violations>
- **Wired-integration audit result:** OK_NO_TOKENS / <violations>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Receipt rows (mechanical):** <literal `cycle_scope_gate.py --receipt` output: closed, relabeled, rust_lines_changed, ratio, builds_recorded, pcgen_live_files>
- **PCGen residue:** <literal `pcgen_residue_gate.py --check` line — never above the previous receipt's>
- **Oracle parity:** <compared=N agree=N disagree=N with PCGEN_ORACLE_SHA, when a Number mapping was added or Epic 6 touched a live path; else N/A>
- **Movement, four buckets:** closure (into DONE, by id-set) / relabel (bucket to bucket) / reachability / instrument-correction
- **Refused tokens:** <token_type=count, ...> or none
- **Discoveries:** <a token type, kind, or remaining-step category that token-coverage.json / the atlas did not predict> or none — each also emitted as a `correction` retro event
- **Figures + their re-derive commands:** <every number, with its command and denominator>
- **Build scope verified:** <--no-run exit, workspace result with targets executed, desktop crate result or "epic cadence", run at SHA <sha>>
- **Sweep population:** <corpus_literal_sweep examined before → after, or N/A>
- **Oracle pin:** <PCGEN_ORACLE_SHA, if any figure came from the pinned corpus>
- **Status:** complete | partial | blocked-escalated
- **Notes:** <judgment calls — expected to be short; a long Notes row is a smell>
- **Next-cycle scope:** <the cycle_scope_gate.py flags for the remainder, or "criterion at zero">
```

## 8. Self-heal posture

- **Self-healable:** dirty tree; single-token audit violation; unrelated test-setup breakage;
  build-counter out of sync; a stage-count or binary-count assertion moved by this cycle's own
  deliberate change (update it in the same commit).
- **Non-self-healable:** diverged tree needing manual rebase; two live cycles on conflicting
  files; a launch gate not actually met; RED→GREEN not preserved; a stub, inline mock, or
  `"Would …"` string in shipping code; **a cycle under the floor that is not the whole
  remainder** (report and stop; the orchestrator re-scopes); **a cycle that raised
  `pcgen_live_files`** (revert the live-side read; the fix belongs on the converter side);
  **more than 10 distinct refused token types surfacing in one cycle** (the converter's mapping
  table is behind the corpus — re-scope, do not grind).

**The template's `## DISCOVERED` queue is superseded in this bundle**, not dropped: a
mechanism-shaped discovery is a refused token type in `token-coverage.json` (the next cycle's
scope); an atlas-shaped discovery (a remaining-step category the atlas did not predict) is a
`correction` retro event plus an atlas re-derivation (SD-34 `decisions.md §2`); a scope-shaped
discovery goes to `forward-scope-register.md` only if it was **not** in the Definition of Done
at launch. §7's `Discoveries` row is where a cycle names which.

**A `## Open blockers` entry pauses the bundle** and is a request for an operator ruling. Two
dispositions only: clear it, or raise your hand and wait. **Under the sheet rule, "the engine
cannot model X" is not a blocker** — the record renders as words and the unit is done; a
subsystem someone wants later is that later bundle's scope.

**Disk usage:** after every parallel wave, `df -h /` and `git worktree list`; prune merged
worktrees. Never remove a `locked` worktree or one carrying unmerged commits.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[A-Za-z0-9_ -]*>' docs/release/SD-35-corpus-sheet-completion/ --include='*.md'
```

(The class is case- and digit-inclusive so `<N>` and `<sd34-pr-number>`-shaped literals are
caught.) Every match must resolve to a real value or be a **documented** deferral. SD-35 has
**no open deferral** after the launch-readiness audit — the cut, the stamp, and §1's outputs all
landed 2026-09-07. Receipt-schema and command-shape placeholders in §6/§7 (`<scoped paths>`,
`<sha>`, `<cycle-id>`, `<epic-name>`, `<this cycle's scope flags>`, `<tranche-15-cut-sha>`, `<bundle-launch-date>`, `<epic-start>`, `<lane-role-name>`,
`<epic-dir>`, `<criterion-id>`, `<epic>`, `<nnn>`, `<N>`, `<n>`, `<x>`, `<list>`, `<violations>`,
`<reason>`, `<filter>`, `<inventory>`, `<scope flags>`, `<type>=<count>`, `<lines per unit>`,
`<exact source text>`, `<enclosing fn name>`, `<class>` in `ground_<class>` / `CL=<class>`,
`<kind>` in per-kind gates, `<form>` in `sheet_rule_rendered:<form>`, `<key>` in
`data/sheet_rules/<book>/<kind>/<key>.json`, `<book>`, `<cycle-start-sha>`, and Rust generics
such as `Option<Expr>` / `Box<Expr>` / `Vec<Expr>` in `technical-design.md`'s code blocks) are
template literals a lane fills at run time, not values. The same list governs
`artifacts/README.md`, `technical-design.md §4`, and `epic-breakdown.md`.

## 10. Epic wrap-up (after every epic)

0. **The full gate, once, on the isolated worker:** `scripts/verify.sh` — every stage — in a
   worktree with its own `CARGO_TARGET_DIR`, **overlapping the next epic's first cycle**
   (`decisions.md §3`). The worker pushes nothing; it returns the log path and the stage table.
   Record wall time. Any red stage is fixed in a wrap-up correction cycle before the next epic's
   **second** cycle dispatches; the fix cycle is exempt from the batch floor (`decisions.md §2`'s
   exemption), never from the residue check.
1. `python3 scripts/retro.py summary --since <epic-start> --json` — **read it**. Fold
   incident/correction/deferral counts into the epic's closing receipt. **Any `incident` key
   that fired 3+ times must produce a mechanical control** or a named escalation. **Name every
   cycle whose `rust_lines_changed / units_closed` exceeded 3.0 and state what the lines bought**
   (`decisions.md §4`).
2. Worktree sweep for **this epic's** worktrees only.
3. **No PR here.**

## 11. Bundle closure epilogue (once, as Epic 7)

1. **Final-acceptance scan** (AT-35-E7-001). Every criterion and every `kanban.md` card at
   `complete`. **Never "complete *or* filed under `## Open blockers`".** If anything is short,
   **stop** — no retrospective, no sweep, **no PR**.
2. **Write the retrospective** to `docs/retro/sd35-corpus-sheet-completion-retrospective.md`
   **and cite it from `references/README.md` in the same cycle.**
3. **Full worktree/branch sweep**, counts found vs removed.
4. **Architecture docs, graphify, PR, merge-conflict resolution** — `../template/template.md §6`.
   `docs/architecture/rules-engine.md`, `rules-data-tables.md`, `corpus-ingest.md`,
   `desktop-app.md`, `status.md`, `testing.md` are the topics this bundle touches; each is
   re-verified, and the converter/live boundary (`technical-design.md §0`) is written into
   `overview.md` as an architecture fact.
5. **Release notes and version confirmation** (`0.15.0`; tranche digit unchanged at closure).

Steps 2 and 3 happen **before** step 4 opens the PR. The operator merges.

## 12. Standing lessons — each names its enforcing command

SD-34's 26 rows (`../SD-34-book-completion/workflow-instruction.md §12`) are inherited
unchanged and not re-listed. SD-35 adds:

| # | Lesson | Enforced by |
|---|---|---|
| 27 | A cycle is a mechanism corpus-wide, 500 units minimum | `scripts/cycle_scope_gate.py --min 500`, §6 step 0, nonzero exit |
| 28 | DONE is a rendered sheet line, not an observed pipeline delta | `sheet-complete` rung (AT-35-E2-003) + per-kind on-screen tests (AT-35-E2-002) |
| 29 | No per-unit proof machinery | §7's ratio row from `cycle_scope_gate.py --receipt`; §10 step 1's named review |
| 30 | One build per cycle; the full gate once per epic | §6 step 3 + §10 step 0; `builds_recorded` in the receipt rows |
| 31 | Citation pins are content anchors, not line numbers | AT-35-E1-002; the three `--check`s as `verify.sh` stages |
| 32 | A new status is wired into every consumer in the same cycle | AT-35-E2-003's grep census before/after |
| 33 | The remainder is named by token type, and the token sums are checked | `scripts/token_coverage.py --check` (AT-35-E2-004) |
| 34 | Build time is a measured figure, not a complaint | `artifacts/epic-1-tax-cut/build-time.json` (AT-35-E1-003), before and after |
| 35 | PCGen is read by the converter and the test oracle only; the live-side count never rises and is zero at closure | `scripts/pcgen_residue_gate.py --check` in §6 step 0 and step 3; `--closure` from AT-35-E6-004; `pcgen_live_files` in every receipt |
| 36 | Our data files carry none of the source format | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0, in §6 step 3 |
| 39 | Lane model is a launch argument, Fable first then Opus; the orchestrator runs on Opus | `LANE_MODEL` from `args.laneModel`; `halted(null)` → relaunch on the next tier (`decisions.md §14`) |
| 38 | Long read-only jobs run on an isolated worker that pushes nothing; cycle work stays local and visible | §2's worker split; §2.4's `pendingWrap`; `isolation: 'worktree'` on every gate/oracle agent |
| 37 | A predecessor's unrun closure is folded into the successor, not forgotten | AT-35-E1-006 (SD-34's retrospective written and cited; its open rows mapped to SD-35 criteria) |

**No `UNENFORCED` row.** Row 29 is partially mechanical (the ratio) and partially a reading
(what the lines bought); `risks-and-open-questions.md §10` tracks it.

## Cross-references

- `../../governance/workflow-instruction-template.md` — the template this is authored from.
- `../../governance/blocker-closure-doctrine.md` — enforced by §8 and §11 step 1.
- `../SD-34-book-completion/` — the predecessor; its atlas, tables, and fable review are SD-35's inputs.
- `../SD-34-book-completion/workflow-instruction.md §12` — the 26 inherited standing lessons.
- `docs/retro/sd35-corpus-sheet-completion-retrospective.md` — written at closure (§11 step 2).
- `.claude/skills/stc-authoring/SKILL.md` — the skill this package was authored with.
