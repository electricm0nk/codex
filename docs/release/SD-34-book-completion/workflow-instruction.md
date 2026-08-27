---
canonical: true
owner: god-emporer
bundle_id: SD-34
status: planning-ready
date: 2026-08-26
authored_from: ../../governance/workflow-instruction-template.md
---

# SD-34 Workflow Instruction — Workflow-Orchestrated Dispatch

Authored from `../../governance/workflow-instruction-template.md`, not from a prior
bundle's copy. States the current dispatch procedure only.

## 0. Bundle at a glance

- **Branch:** `tranche/14` — **not yet cut.** SD-33's closure PR #377 merged to `develop`
  2026-08-27 (`ea2b3396f2`); the cut is §1 item 8. Documented deferral, resolution point named
  in `decisions.md §11`.
- **Board:** local-file `./kanban.md` (Hermes board retired 2026-08-01,
  `SD-30-.../decisions.md` Decision 14a)
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop
- **Epics / criteria:** 6 / 27 (26 kanban rows)
- **First concrete build value:** `0.14.0`, stamped at the `tranche/14` cut
  (`decisions.md §11`)

## 1. Pre-launch checklist

**Run 2026-08-27 at `tranche/14` cut SHA `571307724f6d865d744fb025760ed8ab58a26229`.** Items 1-9, 11, 12 below carry their pasted output. Item 10 (widest build scope + inherited test baseline) is run by its own lane and pasted when it lands; until it is, the bundle is NOT launch-ready. `git fetch origin` first — a stale local `origin/develop` makes item 3 fail falsely.

1. **Board reachable.** `test -f docs/release/SD-34-book-completion/kanban.md && echo KANBAN_PRESENT`

   **Output:**
   ```
   KANBAN_PRESENT
   ```
2. **Version source of truth read.** Both of:
   `python3 -c "import json;print(json.load(open('apps/desktop/package.json'))['version'])"`
   and the same for `apps/desktop/src-tauri/tauri.conf.json`. Expect `0.14.0` after the cut.

   **Output:**
   ```
   0.14.0
   0.14.0
   ```
3. **Predecessor's closure PR merged to develop.** `gh pr view 377 --json state,mergedAt,mergeCommit`
   (expect `MERGED`, `2026-08-27T01:35:37Z`, `ea2b3396f2…`)
   plus `git log origin/develop --oneline | head -3`, confirming the merge commit is in
   `origin/develop`'s ancestry. **Tier-1 launch gate.**

   **Output:**
   ```
   {"mergeCommit":{"oid":"ea2b3396f2fde9223dde93522bd2288b463a21ee"},"mergedAt":"2026-08-27T01:35:37Z","state":"MERGED"}
   ea2b3396f2 Merge pull request #377 from electricm0nk/tranche/13
   9a00662f22 docs(sd33): record this cycle's verify.sh denominator-gate run in the retro log
   73dcfc21f6 docs(sd33): fold-docs -- re-derive closure documents for the operator's fold, PR #377 body updated
   ```
4. **SD-33's instrument debt closed inside SD-33.** SD-33 closed at
   `oracle_disagreement=0`, `## Open blockers` empty, and its own deferrals enumerated.
   Re-derive: `python3 scripts/retro.py summary --since 2026-08-24 --json` and enumerate
   open deferrals; confirm none defers SD-33 DoD scope. **Expect exactly 3** (two
   `sd33-e4-unknown`, one `sd33-r6-skillcombat`) — each carried in `forward-scope-register.md`
   under "Carried forward from SD-33" with its revisit condition. A different count is a finding
   to explain, not to paste.

   **Output:**
   ```
   open= 3
   1787633115006-sd33-e4-unknown-136912
   1787633121875-sd33-e4-unknown-58d073
   1787667636036-sd33-r6-skillcombat-3dee2d
   ```
5. **Working tree clean on the bundle branch.** `git status --porcelain`

   **Output:**
   ```
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-closure-epilogue.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-dispatch.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-fold-fix.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-fold-recovered-work.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-2.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-3.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-4.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-5.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-6.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-7.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-8.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation-9.workflow.js
   ?? docs/release/SD-33-computed-value-verification/artifacts/sd-33-remediation.workflow.js
   ```
6. **Doctrine gates.** The live gate is §6's inline grep pair plus the doctrine docs of
   record — `../../governance/no-stub-mvp-doctrine.md`,
   `../../doctrine-external/identifier-discipline.md`. The `~/.hermes/` profile-path check
   is moot post-2026-08-01.

   **Output:**
   ```
   docs/doctrine-external/identifier-discipline.md
   docs/governance/no-stub-mvp-doctrine.md
   ```
7. **Oracle pin present and readable.** `grep -E "^[A-Z_]+=" scripts/pcgen-oracle-pin.env`
   **`~/workspace/repos/pcgen` is forbidden as an oracle path** — `fetch-pcgen-oracle.sh`'s
   default `--dest` resolves there and a `preflight-oracle` PASS against it fails silently.
   Use the repo-local slot and pass `--dest` explicitly.

   **Output:**
   ```
   PCGEN_ORACLE_REPO=https://github.com/PCGen/pcgen.git
   PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6
   PCGEN_ORACLE_SHA_DATE=2026-06-17            # informational: upstream commit date of the pin
   PCGEN_ORACLE_SPARSE_PATHS="data/pathfinder system/gameModes/Pathfinder"
   ```
8. **`tranche/14` cut from `develop` and pushed.** `git ls-remote --heads origin tranche/14`

   **Output:**
   ```
   4ff9e08a921e33a4efc27c7789c370a87fa47757	refs/heads/tranche/14
   ```
9. **Inherited instruments live.** All of these ship from SD-33 and SD-34 depends on them:
   `scripts/verify.sh --only denominator-gate` exits 0; `python3 scripts/box_ledger.py --check`
   exits 0; `scripts/oracle_harness/` present; `cargo run --locked --bin corpus_literal_sweep`
   reports 0 findings.

   **Output:**
   ```
   box_ledger: uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
   box_ledger EXIT=0
   oracle_harness/: __init__.py __pycache__ campaign_key.py charbuild-remainder.txt.ftl charbuild_remainder_generate.py charbuild_remainder_run_one.sh compare.py derive_spell_casting_ability_mapping.py eqm-fixtures oracle_export.py run.py spell_casting_ability_mapping.json weapon-family.txt.ftl
   verify.sh --only denominator-gate: PASS (files_checked=70 violations=0)
   corpus_literal_sweep: 48699 records examined of 51473 read, 413288 tokens compared (9 synthesized), 51460 digests checked, 0 findings CLEAN
   ```
10. **Widest build scope green** (`decisions.md §10`): `cargo test --locked --no-run` exits 0;
    `cargo test --locked --lib`; `cd apps/desktop/src-tauri && cargo test --locked`. SD-33
    closed with **29 of 599** workspace suites carrying **46 of 8,034** failures **proven
    pre-existing** at the `tranche/13` cut and registered forward (corrected from 31 / 49 of
    8,026 by its retrospective §5). Re-derive that set at the
    `tranche/14` cut and record it as SD-34's inherited baseline — **a failure outside that
    set is SD-34's.**
11. **Artifact directories exist, one per epic.** See `artifacts/README.md`. Each holds a
    `.gitkeep` so the directory survives the commit.

    **Output:**
    ```
    epic-1-atlas/: .gitkeep
    epic-2-tables/: .gitkeep
    epic-3-core-rulebook/: .gitkeep
    epic-4-ultimate-campaign/: .gitkeep
    epic-5-forward-plan/: .gitkeep
    epic-6-closure/: .gitkeep
    ```
12. **Denominator gate pointed at THIS package.** The gate's default scope is SD-33's folder
    (`decisions.md §3`). Run
    `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`
    and paste `files_checked=N violations=0`.

    **Output:**
    ```
    files_checked=15
    violations=0
    ```

## 2. Orchestration mode

- **Dispatch mechanism:** the in-harness `Workflow` tool from a live session. Not `/loop /batch`.
- **Default subagent model:** Sonnet.
- **Tiering:** housekeeping (release notes, version bump, lint) → Haiku; adversarial
  verification / final completeness scan → Opus; everything else → Sonnet.
- **Concurrency shape:** fixed per epic in §3, at authoring time.

### 2.1 Agent environment setup

Every dispatched agent sets `RETRO_ACTOR` to its role name. Every agent claims its
`CARGO_TARGET_DIR` immediately:
`mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"`.
Set `CARGO_INCREMENTAL=0` — a stale incremental cache served an SD-32 lane a stale binary
inside its own private target dir.

### 2.2 Execution boundary — the launching session is the orchestrator, never the executor

§6's steps 1–9 happen **inside a dispatched `agent()`/`Workflow` call**. The orchestrating
session's own `Edit`/`Write`/`Bash` calls are reserved for read-only investigation and for
authoring this bundle's planning docs — **never** for `src/`, `scripts/`, `apps/`, or
`data/`.

Discovering a cycle's real scope mid-investigation is a reason to **re-dispatch with the
corrected scope**, never to fix it inline.

**Dispatch first, report second.** A finished wave with no next dispatch is a stall. Before
ending any turn while ready work exists, dispatch it.

**Do not dispatch two lanes at one diagnosis.** Territory scopes *files*; a named defect
needs a named *owner* too.

### 2.3 Retrospective event logging (every cycle)

Emit events via `scripts/retro.py` **at the moment they happen**, not batched at cycle end.
`--verified-by` is required on a `correction`.

`retro.py`'s `deferrals.open` field is trustworthy as of SD-32's fix — confirm with
`grep -n 'len(open_deferrals)' scripts/retro.py` and say so in the receipt that quotes it.

### 2.4 Creating the Workflow script

```javascript
export const meta = {
  name: 'sd-34-dispatch',
  description: 'SD-34 — completion atlas, proven on two books, then the 35-book forward plan',
  phases: [
    { title: 'Epic 1 — Completion Atlas' },
    { title: 'Epic 2 — Build 8 of 9 tables' },
    { title: 'Epic 3 — Core Rulebook to zero' },
    { title: 'Epic 4 — Ultimate Campaign to zero' },
    { title: 'Epic 5 — Price 35 books' },
    { title: 'Epic 6 — Closure epilogue' },
  ],
}

// Built at launch from epic-breakdown.md: epicNCriteria = [{id, prompt}, ...], where
// prompt embeds §6's procedure verbatim plus the criterion's own text. Nothing below
// is defined by this file; the launching session constructs it.

// Every cycle returns THIS shape. The gate check reads fields — never a substring
// (decisions.md §12 L4). `schema` forces the agent to return it.
const CYCLE_SCHEMA = {
  type: 'object',
  required: ['criterion', 'status', 'commit_sha', 'row_count_command_output', 'receipt_path'],
  properties: {
    criterion: { type: 'string' },
    status: { type: 'string', enum: ['complete', 'blocked-escalated'] },
    commit_sha: { type: 'string' },
    row_count_command_output: { type: 'string' },
    receipt_path: { type: 'string' },
    discoveries: { type: 'string' },
  },
}
const halted = r => !r || r.status !== 'complete'

// FULLY SEQUENTIAL (§3). Each epic's output is the next one's input: the atlas
// names the tables, the tables unblock the books, the books measure the rates,
// the rates price the plan. Epics 3 and 4 MAY run concurrently only if §4's
// disjointness check proved it at launch — then each gets isolation: 'worktree'.

async function runEpic(title, criteria) {
  phase(title)
  for (const c of criteria) {
    const r = await agent(cycleProcedurePrompt(c), { model: 'sonnet', phase: title, schema: CYCLE_SCHEMA })
    if (halted(r)) return { halted: c.id, result: r }   // a blocked card PAUSES the bundle (§8)
  }
  return null
}

let h
if ((h = await runEpic('Epic 1 — Completion Atlas', epic1Criteria))) return h     // AT-34-E1-001..007
if ((h = await runEpic('Epic 2 — Build 8 of 9 tables', epic2Criteria))) return h  // AT-34-E2-001..004
if ((h = await runEpic('Epic 3 — Core Rulebook to zero', epic3Criteria))) return h  // AT-34-E3-001..006, one bucket per cycle, cheapest-first
if ((h = await runEpic('Epic 4 — Ultimate Campaign to zero', epic4Criteria))) return h  // AT-34-E4-001..003
if ((h = await runEpic('Epic 5 — Price 35 books', epic5Criteria))) return h       // AT-34-E5-001..004

phase('Epic 6 — Closure epilogue')
// §11 steps, sequential — order is load-bearing (retro + sweep before the PR).
const SCAN_SCHEMA = { type: 'object', required: ['gate', 'status'], properties: {
  gate: { type: 'string', enum: ['PASS', 'FAIL'] }, status: { type: 'string' }, short: { type: 'string' } } }
const scan = await agent(finalAcceptanceScanPrompt(), { model: 'opus', phase: 'Epic 6 — Closure epilogue', schema: SCAN_SCHEMA })
if (!scan || scan.gate !== 'PASS') return { halted: 'AT-34-E6-001', result: scan }
await agent(retrospectiveAndSweepPrompt(), { model: 'sonnet', phase: 'Epic 6 — Closure epilogue', schema: CYCLE_SCHEMA })
await agent(architectureDocsGraphifyPrPrompt(), { model: 'sonnet', phase: 'Epic 6 — Closure epilogue', schema: CYCLE_SCHEMA })
await agent(releaseNotesVersionBumpPrompt(), { model: 'haiku', phase: 'Epic 6 — Closure epilogue', schema: CYCLE_SCHEMA })
return { closed: 'claimed — verify against the repo before relaying (decisions.md §12 L3)' }
```

**Every `agent()` call sets `model` explicitly.** An omitted `model` inherits the
orchestrator's — SD-31 wave 18 burned nearly a full week's Opus quota this way.

**The gate-failure check reads the scan's own JSON fields, never a loose substring match.** An
SD-33 remediation wave halted spuriously because its failure regex matched the words
`blocked-escalated` inside a sentence stating rows were *not* blocked-escalated. Match the
`gate` and `status` fields themselves.


## 2.5 A dispatched agent is never resumed — never end a turn waiting

A dispatched agent gets **exactly one turn**. Nothing wakes it. Put this in every dispatch
prompt:

- Wait for slow work **inside** the turn — foreground it, or poll a background job in a loop.
- **Scope test runs.** Name the targeted binaries/modules plus the workspace suites; say
  which sweeps *not* to run. `apps/desktop/src-tauri` is a **separate cargo workspace** —
  test it explicitly or not at all, never assume a root sweep covered it.
- **Measure before a population-scoped run.** State measured per-unit cost, population, and
  projected wall time **before** launching the full run. A method proven at n=1 is not a
  method proven at n=8,330 — SD-33 Epic 5 reached 32 of 8,330 by carrying a one-character-
  per-unit method into a population, and cost four remediation waves.
- If something will not finish, **report what was observed and commit the work anyway.**
- **Commit and push before ending the turn**, always.

**Orchestrator's side:** never accept a lane's final message as evidence. Check `git log`
and the target files, and count rows in the artifact.

## 3. Per-epic parallel/sequential map

| Epic | Criteria | Parallel? | File-touch set (verified §4) | Gated on |
|---|---|---|---|---|
| 1 Completion Atlas | E1-001..007 | no | `scripts/completion_atlas.py` (new), `scripts/tests/test_completion_atlas.py` (new), `src/bin/v06_work_inventory.rs` (the rename), `docs/work-inventory.json` (**Epic 1 is its writer**), `scripts/verify.sh` + `scripts/denominator_gate.py` (E1-006 stage + default-scope widening), `scripts/verify.sh` (E1-007 trap-audit stage), `artifacts/epic-1-atlas/` | launch gates |
| 2 Build 8 of 9 tables | E2-001..004 | no | `src/bin/v06_work_inventory.rs`, `src/rules_core/`, `artifacts/epic-2-tables/` | Epic 1 |
| 3 Core Rulebook to zero | E3-001..006 | no | `src/rules_core/`, `src/bin/`, `scripts/oracle_harness/`, `data/corpus/core_rulebook/**` (guarded generator path ONLY), `docs/work-inventory.json` (regenerates; sequential with Epic 4), `artifacts/epic-3-core-rulebook/` | Epic 2 |
| 4 Ultimate Campaign to zero | E4-001..003 | **maybe** | `src/rules_core/`, `src/bin/`, `data/corpus/ultimate_campaign/**` (guarded generator path ONLY), `docs/work-inventory.json` (**only if sequential** — if run in parallel with Epic 3, Epic 4 reads a copy pinned at wave start and never writes it), `artifacts/epic-4-ultimate-campaign/` | Epic 2 |
| 5 Price 35 books | E5-001..004 | no | `artifacts/epic-5-forward-plan/` (read-only against the rest of the repo) | Epics 3 and 4 |
| 6 Closure epilogue | E6-001..003 | no | package docs, `receipts.md`, `release-notes.md`, `docs/architecture/` | Epics 1–5 |

**Epics 3 and 4 are the only pair that could run concurrently** — different books, disjoint
corpus subtrees, both gated only on Epic 2. That is why Epic 4's row reads `maybe`: it is
decided **at launch** by §4's per-file check, not assumed here. Both epics touch
`src/rules_core/` and `src/bin/`, so **unless that check proves per-file disjointness they run
sequentially, Core Rulebook first.** If they do run in parallel, every agent in that call gets
`isolation: 'worktree'`.

**Every other boundary is a real dependency.** The atlas names the tables, the tables unblock
the books, the books measure the rates, the rates price the plan. Inventing concurrency across
those would build work on a denominator that had not landed yet.

**Epic 3 runs one bucket per cycle**, cheapest-first, so a long epic still banks measured rates
as it goes rather than delivering nothing until it finishes.


## 4. File-touch verification

**Required before §3's rows are treated as verified.** Run `ls`/`find` on every path above;
correct any that does not exist as written **before** launch. Paths marked `(new)` are epic
deliverables — confirm the **parent directory** exists and that the name does not collide.

**Run for real 2026-08-27 at `tranche/14` `571307724f` (version bump commit):**

```
EXISTS   src/bin/v06_work_inventory.rs
EXISTS   src/bin/formula_interpreter.rs
EXISTS   src/rules_core/pilot_compute/formula_interpreter.rs
EXISTS   scripts/verify.sh
EXISTS   scripts/box_ledger.py
EXISTS   scripts/oracle_harness/run.py
EXISTS   scripts/oracle_harness/compare.py
EXISTS   scripts/pcgen-oracle-pin.env
EXISTS   docs/work-inventory.json
EXISTS   src/rules_core/corpus_literal_sweep.rs
EXISTS   scripts/denominator_gate.py
EXISTS   scripts/tests/                    (parent of the new test file)
free     scripts/completion_atlas.py       (Epic 1 deliverable, no collision)
free     scripts/tests/test_completion_atlas.py  (Epic 1 deliverable, no collision)
```

**Epic 3 ‖ Epic 4 disjointness check — the decision §3 defers here.** Run after Epic 2 closes,
before Epic 4 is dispatched:

```bash
git diff --name-only <epic-2-close-sha>..HEAD -- src/ scripts/ > /tmp/e3_touched.txt   # Epic 3 so far
# Epic 4's planned set = §3 row 4 minus artifacts/ and data/corpus/ultimate_campaign/
grep -Ff /tmp/e3_touched.txt <(printf 'src/rules_core/\nsrc/bin/\ndocs/work-inventory.json\n') && echo OVERLAP || echo DISJOINT
```

`OVERLAP` → sequential, Core Rulebook first (the default). `DISJOINT` at the **file** level (not
the directory level) → Epic 4 may run in a worktree in parallel, reading a pinned inventory copy.
Directory-level disjointness is not enough; the check must resolve to real file paths.

**Re-run this block at launch**, not at authoring time — SD-33 shipped one wrong path into a
dispatch brief and a cycle discovered it at cost.

**Known hazard:** a shallow glob lies here. A `data/corpus/<book>/equipment/*.json` pattern
returns zero where thousands exist one level deeper under a subdirectory. This mistake was made
and caught during this package's own authoring. Use recursive search and state the search used.

**Second known hazard, and the more expensive one:** a status field's **name is not its
meaning**. `not-ingested` means "the engine does not hold this record" — every one of its
26,002 of 26,002 units carries a real `source_file` and `source_line`. Reading the name instead of the
code that writes it produced a wrong headline reported to the operator during this package's
first draft. Read the writing code before quoting any field.


## 5. Concurrent-write protocol

```bash
git fetch origin tranche/14 && git rebase origin/tranche/14 && git push origin HEAD:tranche/14
```

Retry up to 5 times on non-fast-forward. **Never force-push.** Re-read any shared file
immediately before editing it. **SD-34's shared files are exactly three:** `progress.md`
(prepend-only cycle log), `kanban.md` (one row per cycle), and `docs/work-inventory.json`
(written by Epic 1; regenerated by Epics 3 and 4 **sequentially**, see §3). There is no
`THE-BOX.md` in this bundle; the atlas artifacts under `artifacts/epic-1-atlas/` are written by
Epic 1 and re-derived — not appended — by later epics.

**`git status --porcelain` before EVERY git write.** SD-32 caught ten-plus destructive
near-misses this way, including one staging 1,580 deletions of a sibling generator's
records. **Never `git add -A`. Never `git stash` in this repo at all** — the bare form
stashes the whole shared checkout even from a subdirectory.

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
   The trailing `\b` is deliberately omitted from the first pattern — it never matches
   between `_` and a word character, so adding it back silently stops catching
   `sd19_class_catalog`. **`<scoped paths>` is a template literal:** substitute this epic's
   file-touch set from §3, space-separated, before running. It is not resolved by any script.
3. Implement TDD-style: RED → confirm it fails **for the intended reason** → GREEN → run
   the scoped suite, **and verify at the widest build scope** (`decisions.md §10`):
   `cargo test --locked --no-run` must exit 0, and `apps/desktop/src-tauri` is tested
   explicitly if touched. **Run this after the last commit in the cycle that can move a
   figure an assertion depends on** (`decisions.md §12` L7) — an inventory regeneration after
   the test run un-verifies the cycle. If the cycle added or regenerated corpus records, report
   `corpus_literal_sweep`'s examined-count before and after; the delta must equal the record
   delta (`decisions.md §12` L8).
4. Re-run both audits on the final diff.
5. Write the receipt to `artifacts/<epic-dir>/<criterion-id>_cycle_receipt.md` (§7).
6. Commit, push via §5.
7. Update `progress.md` and `kanban.md` in place via §5.
8. **Count your own artifact and set your status from that count** (`decisions.md §4`).
   Put the literal command output in the report. Mark the `kanban.md` row `complete` only
   if the count says so.
9. Report: criterion, files touched, SHAs, audit results, RED→GREEN evidence, receipt path,
   row-count command output, discoveries, next-cycle plan.

**Never hand-edit `data/corpus/**`** — guarded generator path only. **Never
`--allow-stamp-loss`.** Regeneration hazards, all previously observed here: license/PI
metadata and `raw_tokens` can be destroyed; a record-count change compiles clean while
leaving other files' hardcoded assertions red (grep old **and** new numbers across `tests/`,
`src/`, `apps/`, `scripts/`).

## 7. Per-cycle receipt schema

```markdown
# Cycle <cycle-id> — <epic-name> / <criterion-id>

- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violations>
- **Wired-integration audit result:** OK_NO_TOKENS / <violations>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Figures + their re-derive commands:** <every number, with its command and denominator>
- **Row-count command output:** <literal output of the count on this cycle's own artifact>
- **Build scope verified:** <--no-run exit, workspace result, desktop crate result, run at SHA <sha>>
- **Sweep population:** <corpus_literal_sweep examined before → after, delta vs records added; or N/A>
- **Oracle pin:** <PCGEN_ORACLE_SHA from scripts/pcgen-oracle-pin.env, if any figure came from the pinned corpus>
- **Status:** complete | blocked-escalated
- **Movement, four buckets:** closure / reclassification / reachability / instrument-correction
- **Notes:** <judgment calls>
- **Next-cycle plan:** <what the next cycle picks up>
```

The **row-count** and **build scope** rows are SD-34 additions, enforcing `decisions.md §4`
and `§10`. The **figures** and **four buckets** rows are inherited from SD-33.

## 8. Self-heal posture

- **Self-healable:** dirty tree, single-token audit violation, unrelated test-setup
  breakage, build-counter out of sync.
- **Non-self-healable:** diverged tree needing manual rebase; two live cycles on conflicting
  files; a launch gate not actually met; RED→GREEN not preserved in the receipt; a stub,
  inline mock, or `"Would …"` string in shipping code.

**A `## Open blockers` entry is a request for an operator ruling — not a disposition and
never a closure path.** Filing one **pauses the bundle**. Two dispositions only: **clear it**
(decompose and run the cycles — a large blocker is a sequencing problem, not an exemption,
and a fix that lives in another subsystem is still a fix) or **raise your hand** and wait.

**Disk usage:** after every `parallel: yes` wave, run `df -h /` and `git worktree list`;
prune merged worktrees proactively. Never remove a `locked` worktree or one carrying
unmerged commits.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_ -]*>' docs/release/SD-34-book-completion/ --include='*.md'
```

(The class includes a space so `<scoped paths>` is caught; the recursive form covers
`artifacts/README.md` and `references/README.md`.) Every match must resolve to a real value or
be a **documented** deferral. SD-34's one documented deferral: the `tranche/14` branch cut and
`0.14.0` stamping (`decisions.md §11`), resolved at §1 item 8. Receipt-schema and command-shape
placeholders in §6/§7 (`<scoped paths>`, `<sha>`, `<epic-start>`, `<launch>`) are template
literals a lane fills at run time, not values.

## 10. Epic wrap-up (after every epic)

1. `python3 scripts/retro.py summary --since <epic-start> --json` — **read it**, and fold
   incident/correction/deferral counts into the epic's closing receipt.
   **Check the recurrence keys.** Any `incident` key that has fired **3 or more times** must
   produce a **mechanical control** — a command with a nonzero exit — or an escalation naming
   why one is not possible. A better-worded warning does not satisfy this
   (`decisions.md §12` L5). A polluted shared checkout survived four polite work-arounds in
   SD-33 before the fifth lane committed it.
2. Worktree sweep for **this epic's** worktrees only.
3. **No PR here.**

## 11. Bundle closure epilogue (once, as Epic 6)

1. **Final-acceptance scan.** Every criterion and every `kanban.md` card at `complete`.
   **Never "complete *or* filed under `## Open blockers`".** If anything is short, **stop**
   — no retrospective, no sweep, **no PR**; report what is short with the command that shows
   it.
2. **Write the retrospective** to `docs/retro/sd34-book-completion-retrospective.md`, **and
   cite it from `references/README.md` in the same cycle.**
3. **Full worktree/branch sweep**, reporting count found vs removed.
4. **Architecture docs, graphify, PR, merge-conflict resolution** — `../template/template.md §6`.
5. **Release notes and version bump.**

Steps 2 and 3 happen **before** step 4 opens the PR.

## 12. Standing lessons — each names its enforcing command, or is marked UNENFORCED

Per `decisions.md §12`, a lesson without a mechanism is a quote. Every row states what makes
it fail.

| # | Lesson | Enforced by |
|---|---|---|
| 1 | Recurring incidents get a mechanical control, not a better-worded warning | §6 step 1's base check (nonzero exit) |
| 2 | Every figure carries its re-derive command **and its denominator** | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` (explicit path until AT-34-E1-006 widens the default — `decisions.md §3`) + §7's figures row |
| 3 | A lane's status is a mechanical function of its row count | §7's row-count row + AT-34-E6-001's set-derivation |
| 4 | Sum the piles, always | `scripts/completion_atlas.py --check` → `unclassified=0 overlap=0` (AT-34-E1-001) |
| 5 | A deferral must name its revisit condition, and it must be **checked, not remembered** | AT-34-E6-001's scan + `retro.py`'s corrected `open` field |
| 6 | Measurement waves that bank zero units are legitimate deliverables | `decisions.md §9` + §7's four-buckets row |
| 7 | A headline figure written before the wave establishing it is provisional, and says so | `decisions.md §8` + the denominator gate |
| 8 | Carve-out sweeps grep **code**, not only prose | AT-34-E6-001's instrument grep |
| 9 | "Cannot be verified" is a visible bucket, never folded into done | `completion_atlas.py` conditions 3–4 (AT-34-E1-002) |
| 10 | A count that drops because measurement changed is not closure | §7's four-buckets row |
| 11 | Measure per-unit cost before a population-scoped run | §2.5's required projection, restated in every dispatch brief |
| 12 | A method carried past its limit is corrected, and everything it judged is re-run | `decisions.md §5`; AT-34-E1-005's sweep; AT-34-E6-001's re-derivation |
| 13 | Verify at the widest build scope the repo has | §6 step 3; §7's build-scope row; AT-34-E6-001 |
| 14 | A lane's attribution of a failure is a claim, not evidence | AT-34-E6-001 re-derives attribution from `git` against the cut SHA |
| 15 | A vacuous pass is not a pass — state every gate's population | `scripts/verify.sh --only figure-provenance` (AT-34-E1-006) — a PASS line that names no population fails the stage |
| 16 | A remainder named per-mechanism is closable; "the rest" is not | AT-34-E3-003 and AT-34-E5-003's per-mechanism enumeration |

| 17 | A status field's **name is not its meaning** — read the code that writes it | AT-34-E1-002 condition 6: a bucket must cite the `file:line` emitting its evidence strings, and fails closed when that citation stops resolving. Plus AT-34-E1-005's rename. (`decisions.md §12` L1) |
| 18 | A remaining step the atlas did not predict is a **defect in the atlas**, not just new work | AT-34-E3-006's `atlas-defects.md` + a forced re-derivation |
| 19 | Never carry your own number forward — **re-derive it** | `scripts/verify.sh --only figure-provenance` (AT-34-E1-006). (`decisions.md §12` L2) |
| 20 | A dispatch script's return value is **not** a closure claim | AT-34-E6-001 re-derives every `complete` from the repo; no closure rests on a script's return. (`decisions.md §12` L3) |
| 21 | Match structured **fields**, not substrings | §2.4's dispatch-script contract — the gate check reads the scan's own `gate`/`status` fields. (`decisions.md §12` L4) |
| 22 | A repeated workaround means **clear the obstacle** | §10 step 1 + AT-34-E6-001: an `incident` recurrence key firing 3+ times must produce a mechanical control or a named escalation. (`decisions.md §12` L5) |
| 23 | A stale branch's **file count is not its value** — read its record schema against HEAD | `forward-scope-register.md §E1` (SD-33's three ruled-out branches, by name) + AT-34-E6-003's sweep diagnoses any branch outside that table schema-against-HEAD before folding. (`decisions.md §12` L6) |
| 24 | Run the suite **after the last write that can move it** | §6 step 3 — build-scope row names the SHA it ran at; a regeneration after the test run un-verifies the cycle. (`decisions.md §12` L7) |
| 25 | A gate's **examined-population must grow** when records are added | §6 step 3 + §7's sweep-population row + AT-34-E6-001: examined-count delta must equal the record delta over any corpus change. (`decisions.md §12` L8) |
| 26 | A count change compiles clean but leaves **other files' hardcoded assertions** red | §6's regeneration rule: grep old **and** new counts across `tests/`, `src/`, `apps/`, `scripts/` before commit; §7's build-scope row |

**Every row above names an enforcer. There is no `UNENFORCED` row in this table**, and adding
one would itself be a defect under `decisions.md §12`, tracked in `risks-and-open-questions.md`.

Row 15 was UNENFORCED in this package's first draft and is now closed by AT-34-E1-006. Its
origin: SD-33's corpus sweep was green on records whose `raw_tokens` were empty — its
population was "tokens the record claims", and a record claiming nothing cannot mismatch. The
green was real and meaningless.

**Rows 17 and 19–22 are this session's own lessons** (`decisions.md §12` L1–L5), each traced to a
real cost paid during SD-33's closure and this package's authoring. They share one root: a derived
artifact was trusted instead of the source it derives from. **Rows 23–25 are SD-33's retrospective
§6 fold lessons** (`decisions.md §12` L6–L8), written for this bundle. Row 26 is SD-33 retro §3
lesson g, previously folded silently into row 13.

## Cross-references

- `../../governance/workflow-instruction-template.md` — the template this is authored from.
- `../../governance/blocker-closure-doctrine.md` — enforced by §8 and §11 step 1.
- `../../governance/deferral-revisit-doctrine.md` — the sibling rule for a planned capability deferral.
- `../SD-33-computed-value-verification/` — the predecessor; its instruments are SD-34's inputs.
- `../SD-33-computed-value-verification/forward-scope-register.md §E1` — three branches ruled OUT of SD-33's fold; mirrored in this bundle's `forward-scope-register.md §E1`. **Not to be re-litigated.**
- `docs/retro/sd34-book-completion-retrospective.md` — written at closure (§11 step 2), cited from `references/README.md`.
- `.claude/skills/stc-authoring/SKILL.md` — the skill this package was authored with.
