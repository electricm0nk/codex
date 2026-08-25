---
canonical: true
owner: god-emporer
bundle_id: SD-33
status: planning-ready
date: 2026-08-24
authored_from: ../../governance/workflow-instruction-template.md
---

# SD-33 Workflow Instruction — Workflow-Orchestrated Dispatch

Authored from `../../governance/workflow-instruction-template.md`, not from a prior bundle's copy. States the current dispatch procedure only.

## 0. Bundle at a glance

- **Branch:** `tranche/13` @ `f652db7ac7` (cut from `develop` after PR #376, pushed)
- **Board:** local-file `./kanban.md` (Hermes board retired 2026-08-01, `SD-30-.../decisions.md` Decision 14a)
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop
- **Epics / criteria:** 6 / 21
- **First concrete build value:** `0.13.0` (`apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json`), resolved on `tranche/13` cut per `decisions.md §3`.

## 1. Pre-launch checklist

Commands run for real on 2026-08-25 at `f652db7ac7`, output pasted. All items pass.

**1. Board reachable.** Hermes board retired; the local-file equivalent is the check.
```
$ test -f docs/release/SD-33-computed-value-verification/kanban.md && echo KANBAN_PRESENT
KANBAN_PRESENT
```

**2. Version source of truth read.**
```
$ python3 -c "import json;print(json.load(open('apps/desktop/package.json'))['version'])"
0.13.0
$ python3 -c "import json;print(json.load(open('apps/desktop/src-tauri/tauri.conf.json'))['version'])"
0.13.0
```

**3. PASS — predecessor's closure PR merged to develop.**
```
$ gh pr view 376 --json state,mergedAt,mergeCommit
{"mergeCommit":{"oid":"f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba"},"mergedAt":"2026-08-25T02:12:18Z","state":"MERGED"}
$ git log origin/develop --oneline | head -3
f53b8e32da Merge pull request #376 from electricm0nk/tranche/12
525e087c5b docs(sd32): record PR #376 in release-notes.md and the closure receipt
d527322eaa docs(sd32): closure epilogue complete — kanban row 13 -> complete, 22/22
```
PR #376 (SD-32, `tranche/12`) is `MERGED` at `f53b8e32da`, which is `origin/develop`'s HEAD.

**4. PASS — SD-32's instrument debt closed inside SD-32.**
```
$ python3 scripts/retro.py summary --since 2026-08-22 --json | python3 -c "import json,sys; d=json.load(sys.stdin)['deferrals']; print('total:', d['total']); print('open:', d['open'])"
total: 29
open: 0

$ grep -n "^EXCLUDED_BOOKS" scripts/observer/pf1e_dashboard_producer.py
3519:EXCLUDED_BOOKS: frozenset[str] = frozenset()

$ grep -n "open.*len(open_deferrals)" scripts/retro.py
772:            "open": len(open_deferrals),
```
All three named items are closed: `retro.py`'s `open` field now counts genuinely-open deferrals (`len(open_deferrals)`, not `deferrals[-limit:]`); the SD-32 window shows 29 total / **0 open**; `EXCLUDED_BOOKS` is `frozenset()` (no carve-out).

**5. Working tree clean on the bundle branch.**
```
$ git status --porcelain
 M docs/retro/events/sd31-transcribe.jsonl
```
One dirty line, `docs/retro/events/sd31-transcribe.jsonl` — belongs to another concurrent lane, not this bundle's tree; left as-is per `workflow-instruction.md §5`.

**6. Doctrine gates.** The live gate is the inline grep pair in §6 plus the doctrine docs of record — `../../governance/no-stub-mvp-doctrine.md`, `../../doctrine-external/identifier-discipline.md`. The `~/.hermes/` profile-path check is moot post-2026-08-01.

**7. Oracle pin present and readable.**
```
$ grep -E "^[A-Z_]+=" scripts/pcgen-oracle-pin.env
PCGEN_ORACLE_REPO=https://github.com/PCGen/pcgen.git
PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6
PCGEN_ORACLE_SPARSE_PATHS="data/pathfinder system/gameModes/Pathfinder"
```
**`~/workspace/repos/pcgen` is forbidden as an oracle path.** `scripts/fetch-pcgen-oracle.sh`'s default `--dest` resolves there and a `preflight-oracle` PASS against it fails silently. Use the repo-local slot.

**8. PASS — `tranche/13` cut from `develop` and pushed.**
```
$ git ls-remote --heads origin tranche/13
f652db7ac7f90466ba9d931360472da3add00733	refs/heads/tranche/13
```
`origin/tranche/13` exists and matches current HEAD `f652db7ac7`.

**9. Artifact directories exist, one per epic.** Created at package construction; see `artifacts/README.md`.

## 2. Orchestration mode

- **Dispatch mechanism:** the in-harness `Workflow` tool from a live session. Not `/loop /batch`.
- **Default subagent model:** Sonnet.
- **Tiering:** housekeeping (release notes, version bump, lint) → Haiku; adversarial verification / final completeness scan → Opus; everything else → Sonnet.
- **Concurrency shape:** fixed per epic in §3, at authoring time.

### 2.1 Agent environment setup

Every dispatched agent sets `RETRO_ACTOR` to its role name. Every agent claims its `CARGO_TARGET_DIR` immediately: `mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"`. Set `CARGO_INCREMENTAL=0` — a stale incremental cache served an SD-32 lane a stale binary inside its own private target dir.

### 2.2 Execution boundary — the launching session is the orchestrator, never the executor

§6's steps 1–9 happen **inside a dispatched `agent()`/`Workflow` call**. The orchestrating session's own `Edit`/`Write`/`Bash` calls are reserved for read-only investigation and for authoring this bundle's planning docs — **never** for `src/`, `scripts/`, `apps/`, or `data/`.

Discovering a cycle's real scope mid-investigation is a reason to **re-dispatch with the corrected scope**, never to fix it inline.

**Dispatch first, report second.** A finished wave with no next dispatch is a stall. Before ending any turn while ready work exists, dispatch it.

**Do not dispatch two lanes at one diagnosis.** Territory scopes *files*; a named defect needs a named *owner* too. Two SD-32 lanes independently produced byte-identical fixes and one was discarded on rebase — twice.

### 2.3 Retrospective event logging (every cycle)

Emit events via `scripts/retro.py` **at the moment they happen**, not batched at cycle end. `--verified-by` is required on a `correction`.

**Binding for this bundle** (`decisions.md §2`): `retro.py`'s `deferrals.open` is `deferrals[-limit:]` — the last N, not the open ones. **Never quote it as a closure figure.** If SD-32's fix has landed, use the corrected field and say so in the receipt; if not, enumerate deferrals directly and state the total.

### 2.4 Creating the Workflow script

```javascript
export const meta = {
  name: 'sd-33-dispatch',
  description: 'SD-33 — computed-value verification: oracle harness, engine coverage, classification',
  phases: [
    { title: 'Epic 1 — Instruments' },
    { title: 'Epic 2 — Oracle harness' },
    { title: 'Epic 3 — Engine coverage' },
    { title: 'Epic 4 — Unknown classification' },
    { title: 'Epic 5 — Re-verification' },
    { title: 'Epic 6 — Closure epilogue' },
  ],
}

phase('Epic 1 — Instruments')
await pipeline(
  epic1Criteria,                                     // from epic-breakdown.md
  c => agent(cycleProcedurePrompt(c), { model: 'sonnet', phase: 'Epic 1 — Instruments' }),
)

// Epics 2/3/4 are write-disjoint per §3/§4 — worktree isolation is mandatory.
await parallel([
  () => agent(cycleProcedurePrompt(e2), { model: 'sonnet', phase: 'Epic 2 — Oracle harness',        isolation: 'worktree' }),
  () => agent(cycleProcedurePrompt(e3), { model: 'sonnet', phase: 'Epic 3 — Engine coverage',       isolation: 'worktree' }),
  () => agent(cycleProcedurePrompt(e4), { model: 'sonnet', phase: 'Epic 4 — Unknown classification', isolation: 'worktree' }),
])

phase('Epic 5 — Re-verification')
// Gated on Epic 2's ruling (AT-33-E2-004): only dispatched once Epic 2's closing
// receipt records Path A or Path B. If Path B, Epic 5's throughput assumption
// changes — that is an operator decision point (decisions.md §5), not a silent
// scope reduction to "coverage only".
await pipeline(
  epic5Criteria,                                       // from epic-breakdown.md
  c => agent(cycleProcedurePrompt(c), { model: 'sonnet', phase: 'Epic 5 — Re-verification' }),
)

phase('Epic 6 — Closure epilogue')
// §11 steps, sequential — order is load-bearing (retro + sweep before the PR).
await agent(finalAcceptanceScanPrompt(), { model: 'opus', phase: 'Epic 6 — Closure epilogue' })          // §11.1
await agent(retrospectiveAndSweepPrompt(), { model: 'sonnet', phase: 'Epic 6 — Closure epilogue' })      // §11.2, §11.3
await agent(architectureDocsGraphifyPrPrompt(), { model: 'sonnet', phase: 'Epic 6 — Closure epilogue' }) // §11.4
await agent(releaseNotesVersionBumpPrompt(), { model: 'haiku', phase: 'Epic 6 — Closure epilogue' })     // §11.5
```

**Every `agent()` call sets `model` explicitly.** An omitted `model` inherits the orchestrator's — SD-31 wave 18 burned 97% of a week's Opus quota this way.

## 2.5 A dispatched agent is never resumed — never end a turn waiting

A dispatched agent gets **exactly one turn**. Nothing wakes it. Put this in every dispatch prompt:

- Wait for slow work **inside** the turn — foreground it, or poll a background job in a loop.
- **Scope test runs.** Name the targeted binaries/modules plus the workspace suites; say which sweeps *not* to run. `apps/desktop/src-tauri` is a **separate cargo workspace** — test it explicitly or not at all, but never assume a root sweep covered it.
- If something will not finish, **report what was observed and commit the work anyway.**
- **Commit and push before ending the turn**, always.

**Orchestrator's side:** never accept a lane's final message as evidence. Check `git log` and the target files.

## 3. Per-epic parallel/sequential map

| Epic | Criteria | Parallel? | File-touch set (verified §4) | Gated on |
|---|---|---|---|---|
| 1 Instruments | E1-001..004 | no | `scripts/box_ledger.py` (new), `docs/release/SD-33-computed-value-verification/THE-BOX.md` (new), `scripts/verify.sh`, `artifacts/epic-1-instruments/` | launch gates |
| 2 Oracle harness | E2-001..004 | **yes** | `scripts/oracle_harness/` (new), `artifacts/epic-2-oracle-harness/`, `THE-BOX.md` (append-only, §5) | Epic 1 |
| 3 Engine coverage | E3-001..004 | **yes** | `src/rules_core/pilot_compute/formula_interpreter.rs`, `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`, `src/bin/formula_interpreter.rs`, `artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json`, `THE-BOX.md` (append-only, §5) — (reads `docs/work-inventory.json` pinned at wave start; never writes it) | Epic 1 |
| 4 Unknown classification | E4-001..003 | **yes** | `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json` (sole writer), `artifacts/epic-4-unknown-classification/`, `THE-BOX.md` (append-only, §5) | Epic 1 |
| 5 Re-verification | E5-001..003 | no | `artifacts/epic-5-reverification/`, harness from Epic 2, `THE-BOX.md` (append-only, §5) | Epic 2 |
| 6 Closure epilogue | E6-001..003 | no | package docs, `receipts.md`, `release-notes.md` | Epics 1–5 |

`THE-BOX.md` is created by Epic 1 and then amended **append-only via §5's re-read protocol** by Epics 2–5; it is the one deliberately shared file, and §5's re-read-before-edit rule is what keeps the 2/3/4 parallel wave safe.

AT-33-E3-004 regenerates `formula_interpreter.corpus-wide.json` to
`docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json`
via `cargo run --locked --bin formula_interpreter -- --corpus-wide --output <that path>` (`--output` flag confirmed in `src/bin/formula_interpreter.rs`). The binary's default output path `artifacts/gate-2-engines/...` is SD-32's closed evidence file and must not be overwritten — always pass `--output` explicitly.

**Every agent in a `parallel: yes` call gets `isolation: 'worktree'`.** This is load-bearing: agents mutate a shared checkout and will otherwise collide even on disjoint files.

## 4. File-touch verification

**Required before §3's rows are treated as verified.** Run `ls`/`find` on every path above; correct any that does not exist as written **before** launch rather than letting a cycle discover it. Paths marked `(new)` are Epic deliverables and are expected not to exist yet — confirm the **parent directory** exists and that the name does not collide with an existing script.

**Run for real 2026-08-24 at `1d6ae1e72b`:**

```
EXISTS   src/bin/v06_work_inventory.rs
EXISTS   scripts/verify.sh
EXISTS   scripts/shape_ledger.py
EXISTS   scripts/coverage_ledger.py
EXISTS   docs/work-inventory.json
MISSING  src/rules_core/formula_interpreter.rs      <- CORRECTED, see below
free     scripts/box_ledger.py                      (Epic 1 deliverable, no collision)
free     scripts/oracle_harness                     (Epic 2 deliverable, no collision)
```

**One path was wrong at authoring time and is corrected in §3**: `src/rules_core/formula_interpreter.rs` does not exist. `find src -name "formula_interpreter*"` returns the three real paths now listed in §3's Epic 3 row. **This is the check doing its job** — a wrong path shipped into a dispatch brief is discovered by a cycle at cost, not by the author for free.

**Known hazard:** a shallow glob lies here. An SD-32 check using `data/corpus/*/*/x.json` found zero where eight existed one level deeper. Use recursive search, as the `find` above does.

## 5. Concurrent-write protocol

```bash
git fetch origin tranche/13 && git rebase origin/tranche/13 && git push origin HEAD:tranche/13
```

Retry up to 5 times on non-fast-forward. **Never force-push.** Re-read any shared file (`progress.md`, `kanban.md`, `THE-BOX.md`) immediately before editing it.

**`git status --porcelain` before EVERY git write.** SD-32 caught ten-plus destructive near-misses this way, including one staging 1,580 deletions of a sibling generator's records. **Never `git add -A`. Never `git stash` in this repo at all** — the bare form stashes the whole shared checkout even from a subdirectory.

## 6. Per-cycle procedure

Runs **inside a dispatched agent** (§2.2).

1. Rebase onto the bundle branch (§5), then **verify the base is real**:
   ```bash
   test -d docs && test -d data && test -d scripts \
     || { echo 'WRONG BASE — reset before continuing'; exit 1; }
   ```
2. Define the audit base once, then run both greps:
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)

   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
   ```
   The trailing `\b` is deliberately omitted from the first pattern — it never matches between `_` and a word character, so adding it back silently stops catching `sd19_class_catalog`.
3. Implement TDD-style: RED → confirm it fails **for the intended reason** → GREEN → run the scoped suite.
4. Re-run both audits on the final diff.
5. Write the receipt to `artifacts/<epic>/<cycle-id>_cycle_receipt.md` (§7).
6. Commit, push via §5.
7. Update `progress.md` and `kanban.md` in place via §5.
8. Mark the `kanban.md` row `complete` and append the receipt pointer to `progress.md`.
9. Report: criterion, files touched, SHAs, audit results, RED→GREEN evidence, receipt path, discoveries, next-cycle plan.

**Never hand-edit `data/corpus/**`** — guarded generator path only. **Never `--allow-stamp-loss`.**

## 7. Per-cycle receipt schema

```markdown
# Cycle <cycle-id> — <epic-name> / <criterion-id>

- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violations>
- **Wired-integration audit result:** OK_NO_TOKENS / <violations>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Figures + their re-derive commands:** <every number, with its command and denominator>
- **Status:** complete | blocked-escalated
- **Movement, four buckets:** closure / reclassification / reachability / instrument-correction
- **Notes:** <judgment calls>
- **Next-cycle plan:** <what the next cycle picks up>
```

The **figures** and **four buckets** rows are SD-33 additions, enforcing `decisions.md §2` and the SD-32 finding that a count which drops because measurement changed is not closure.

## 8. Self-heal posture

- **Self-healable:** dirty tree, single-token audit violation, unrelated test-setup breakage, build-counter out of sync.
- **Non-self-healable:** diverged tree needing manual rebase; two live cycles on conflicting files; a launch gate not actually met; RED→GREEN not preserved in the receipt; a stub, inline mock, or `"Would …"` string in shipping code.

**A `## Open blockers` entry is a request for an operator ruling — not a disposition and never a closure path.** Filing one **pauses the bundle**. Two dispositions only: **clear it** (decompose and run the cycles — a large blocker is a sequencing problem, not an exemption) or **raise your hand** and wait.

**Disk usage:** after every `parallel: yes` wave, run `df -h /` and `git worktree list`; prune merged worktrees proactively. Never remove a `locked` worktree or one carrying unmerged commits.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-33-computed-value-verification/*.md
```

Every match must resolve to a real value or be a **documented** deferral. All SD-33 placeholders resolved: `0.13.0` per `decisions.md §3`.

## 10. Epic wrap-up (after every epic)

1. `scripts/retro.py summary --since <epic-start> --json` — **read it**, and fold incident/correction/deferral counts into the epic's closing receipt.
2. Worktree sweep for **this epic's** worktrees only.
3. **No PR here.**

## 11. Bundle closure epilogue (once, as Epic 6)

1. **Final-acceptance scan.** Every criterion and every `kanban.md` card at `complete`. **Never "complete *or* filed under `## Open blockers`".** If anything is short, **stop** — no retrospective, no sweep, **no PR**; report what is short with the command that shows it.
2. **Write the retrospective** to `docs/retro/sd33-computed-value-verification-retrospective.md`, **and cite it from `references/README.md` in the same cycle.**
3. **Full worktree/branch sweep**, reporting count found vs removed.
4. **Architecture docs, graphify, PR, merge-conflict resolution** — `../template/template.md §6`.
5. **Release notes and version bump.**

Steps 2 and 3 happen **before** step 4 opens the PR.

## 12. Standing lessons — each names its enforcing command, or is marked UNENFORCED

Per `decisions.md §4`, a lesson without a mechanism is a quote. Every row below states what makes it fail.

| # | Lesson | Enforced by |
|---|---|---|
| 1 | Recurring incidents get a mechanical control, not a better-worded warning | §6 step 1's base check (nonzero exit) |
| 2 | Every figure carries its re-derive command **and its denominator** | `scripts/verify.sh --only denominator-gate` (AT-33-E1-004) + §7's figures row |
| 3 | Dispatch first, report second | **UNENFORCED** — orchestrator discipline; close by making it a receipt field |
| 4 | Sum the piles, always | `scripts/box_ledger.py --check` → `uncovered=0 overlap=0` (AT-33-E1-001) |
| 5 | A deferral must name its revisit condition, and it must be **checked, not remembered** | AT-33-E6-001's scan + the corrected `retro.py` field. **Was UNENFORCED in SD-32 and cost 19 unchecked deferrals.** |
| 6 | Measurement waves that bank zero units are legitimate deliverables | §7's four-buckets row |
| 7 | A headline figure written before the wave establishing it is provisional, and says so | `scope-draft.md §3` + the denominator gate |
| 8 | Carve-out sweeps grep **code**, not only prose | `decisions.md §6` corollary. **UNENFORCED** — close by adding a `verify.sh` stage that greps closure scripts for hardcoded exclusion lists |
| 9 | "Cannot be verified" is a visible bucket, never folded into done | `box_ledger.py` condition 4 (AT-33-E1-002) |
| 10 | A count that drops because measurement changed is not closure | §7's four-buckets row |

**Rows 3 and 8 are marked UNENFORCED deliberately.** Under `decisions.md §4` that marking is itself a defect to close, tracked in `risks-and-open-questions.md` — not a permitted resting state.

## Cross-references

- `../../governance/workflow-instruction-template.md` — the template this is authored from.
- `../../governance/blocker-closure-doctrine.md` — enforced by §8 and §11 step 1.
- `../../governance/deferral-revisit-doctrine.md` — the sibling rule for a planned capability deferral.
- `../../retro/sd32-compute-library-and-cause-closure-retrospective.md` — the source of §12 rows 2, 5, 8, 9, 10.
- `.claude/skills/stc-authoring/SKILL.md` — the skill this package was authored with.
