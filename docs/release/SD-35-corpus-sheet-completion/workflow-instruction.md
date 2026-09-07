---
canonical: true
owner: god-emporer
bundle_id: SD-35
status: planning
date: 2026-09-07
authored_from: ../../governance/workflow-instruction-template.md
---

# SD-35 Workflow Instruction — Workflow-Orchestrated Dispatch

Authored from `../../governance/workflow-instruction-template.md`, not from a prior bundle's
copy. States the current dispatch procedure only.

## 0. Bundle at a glance

- **Branch:** `tranche/15` — **not yet cut.** Cut from `develop` after SD-34's closure PR merges
  (§1 item 3); the cut is §1 item 8. Documented deferral, resolution point in `decisions.md §10`.
- **Board:** local-file `./kanban.md` (Hermes board retired 2026-08-01)
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop
- **Epics / criteria:** 7 / 29 (28 kanban rows)
- **First concrete build value:** `0.15.0`, stamped at the `tranche/15` cut
  (`decisions.md §10`)

## 1. Pre-launch checklist

Every command below is run for real at the `tranche/15` cut with its output pasted under it
before the bundle is marked `planning-ready`. **Unrun at authoring** — SD-34 is still in
progress, so items 3 and 8 cannot run yet; items 1, 2, 5, 6, 7, 9, 11, 12 can be rehearsed now
but are re-run at the cut, and only the cut's output counts.

1. **Board reachable.** `test -f docs/release/SD-35-corpus-sheet-completion/kanban.md && echo KANBAN_PRESENT`

   **Output:** *(at the cut)*
2. **Version source of truth read.** Both of
   `python3 -c "import json;print(json.load(open('apps/desktop/package.json'))['version'])"`
   and the same for `apps/desktop/src-tauri/tauri.conf.json`. Expect `0.15.0` after the cut
   (`0.14.0` at authoring, both files).

   **Output:** *(at the cut)*
3. **SD-34's closure PR merged to `develop`.** `gh pr view <sd34-pr-number> --json state,mergedAt,mergeCommit`
   plus `git log origin/develop --oneline | head -3`. **Tier-1 launch gate.** SD-34 was at wave
   51, kanban rows 13/14/15/17/20/26/27 open, on 2026-09-07.

   **Output:** *(at the cut)*
4. **SD-34's retrospective exists and is cited.** `test -f docs/retro/sd34-book-completion-retrospective.md && grep -c sd34-book-completion-retrospective docs/release/SD-34-book-completion/references/README.md`.
   Expect the file present and a count ≥ 1. SD-34's `## Open blockers` empty; its open
   deferrals enumerated by `python3 scripts/retro.py summary --since 2026-08-27 --json`, none
   deferring SD-34 DoD scope.

   **Output:** *(at the cut)*
5. **Working tree clean on the bundle branch.** `git status --porcelain`

   **Output:** *(at the cut)*
6. **Doctrine gates.** `ls docs/governance/no-stub-mvp-doctrine.md docs/doctrine-external/identifier-discipline.md docs/governance/blocker-closure-doctrine.md`

   **Output:** *(at the cut)*
7. **Oracle pin present and readable.** `grep -E "^[A-Z_]+=" scripts/pcgen-oracle-pin.env`.
   **`~/workspace/repos/pcgen` is forbidden as an oracle path.**

   **Output:** *(at the cut)*
8. **`tranche/15` cut from `develop` and pushed.** `git ls-remote --heads origin tranche/15`

   **Output:** *(at the cut)*
9. **Inherited instruments live.** `python3 scripts/completion_atlas.py --check` exits 0 with
   `unclassified=0 overlap=0 citation_failures=0`; `python3 scripts/shape_engine_boundary.py --check`
   and `python3 scripts/missing_engine_tables.py --check` exit 0 (both were stale at HEAD before
   wave 51 — run them, do not assume); `python3 scripts/box_ledger.py --check` exits 0;
   `cargo run --locked --bin corpus_literal_sweep` 0 findings.

   **Output:** *(at the cut)*
10. **Widest build scope green** (SD-34 `decisions.md §10`): `cargo test --locked --no-run`
    exits 0; `cargo test --locked --no-fail-fast -j 6` with targets executed counted;
    `cd apps/desktop/src-tauri && cargo test --locked`. Re-derive the inherited failing-suite
    set at the cut and record it as SD-35's baseline (`technical-requirements.md §3`). **Also
    record the cold `cargo test --locked --no-run` wall time** — AT-35-E1-003's "before".

    **Output:** *(at the cut)*
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

    **Output (2026-09-07, authoring, at `tranche/14` `5f6b18f4e3`; re-run at the cut):**
    ```
    files_checked=16
    violations=0
    ```

## 2. Orchestration mode

- **Dispatch mechanism:** the in-harness `Workflow` tool from a live session. Not `/loop /batch`.
- **Default subagent model:** Sonnet.
- **Tiering:** housekeeping (release notes, version bump, lint) → Haiku; adversarial
  verification / final completeness scan → Opus; everything else → Sonnet. **Set `model` on
  every `agent()` call.**
- **Concurrency shape:** fixed per epic in §3, at authoring time. **Never more than 3 lanes
  building at once** (`decisions.md §9` L11).

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

// Built at launch from epic-breakdown.md: epicNCriteria = [{id, scope, prompt}, ...].
// `scope` is the cycle_scope_gate.py argument string for that criterion's cycle; the
// prompt embeds §6 verbatim plus the criterion text plus the scope string.

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
const halted = r => !r || r.status === 'blocked-escalated'

// A `partial` result re-dispatches the SAME criterion with the refused-token remainder as its
// scope — the loop below continues until the criterion's population is zero or a cycle halts.
async function runCriterionToZero(title, c, maxCycles = 12) {
  for (let i = 0; i < maxCycles; i++) {
    const r = await agent(cycleProcedurePrompt(c, i), { model: 'sonnet', phase: title, schema: CYCLE_SCHEMA })
    if (halted(r)) return { halted: c.id, result: r }
    if (r.status === 'complete') return null
    c = withRemainderScope(c, r.refused_tokens)   // next cycle: the named remainder, still ≥ 500 or whole-remainder
  }
  return { halted: c.id, result: 'max cycles reached — escalate' }
}

async function runEpic(title, criteria) {
  phase(title)
  for (const c of criteria) {
    const h = await runCriterionToZero(title, c)
    if (h) return h
  }
  // Epic wrap-up (§10): full verify.sh ONCE here, retro summary, worktree sweep.
  const wrap = await agent(epicWrapUpPrompt(title), { model: 'sonnet', phase: title, schema: CYCLE_SCHEMA })
  return halted(wrap) ? { halted: `${title} wrap-up`, result: wrap } : null
}

let h
// Epic 1: E1-003 (tests/) in a worktree beside E1-001+E1-002+E1-005 (scripts/) — parallel: yes, 2 lanes.
phase('Epic 1 — Tax cut')
const e1 = await parallel([
  () => agent(cycleProcedurePrompt(epic1Criteria.find(c => c.id === 'AT-35-E1-003')), { model: 'sonnet', phase: 'Epic 1 — Tax cut', isolation: 'worktree', schema: CYCLE_SCHEMA }),
  () => pipeline(epic1Criteria.filter(c => ['AT-35-E1-001', 'AT-35-E1-002', 'AT-35-E1-005'].includes(c.id)),
                 c => agent(cycleProcedurePrompt(c), { model: 'sonnet', phase: 'Epic 1 — Tax cut', isolation: 'worktree', schema: CYCLE_SCHEMA })),
])
if (e1.flat().some(halted)) return { halted: 'Epic 1', result: e1 }
if ((h = await runEpic('Epic 1 — Tax cut', epic1Criteria.filter(c => c.id === 'AT-35-E1-004')))) return h
if ((h = await runEpic('Epic 2 — Sheet rule', epic2Criteria))) return h
if ((h = await runEpic('Epic 3 — Place and surface', epic3Criteria))) return h
if ((h = await runEpic('Epic 4 — Resolve and verify', epic4Criteria))) return h
if ((h = await runEpic('Epic 5 — Residues', epic5Criteria))) return h
if ((h = await runEpic('Epic 6 — PCGen exit', epic6Criteria))) return h   // oracle parity before + after; gate --closure at E6-004

phase('Epic 7 — Closure epilogue')
const SCAN_SCHEMA = { type: 'object', required: ['gate', 'status'], properties: {
  gate: { type: 'string', enum: ['PASS', 'FAIL'] }, status: { type: 'string' }, short: { type: 'string' } } }
const scan = await agent(finalAcceptanceScanPrompt(), { model: 'opus', phase: 'Epic 7 — Closure epilogue', schema: SCAN_SCHEMA })
if (!scan || scan.gate !== 'PASS') return { halted: 'AT-35-E7-001', result: scan }
await agent(retrospectiveAndSweepPrompt(), { model: 'sonnet', phase: 'Epic 7 — Closure epilogue', schema: CYCLE_SCHEMA })
await agent(architectureDocsGraphifyPrPrompt(), { model: 'sonnet', phase: 'Epic 7 — Closure epilogue', schema: CYCLE_SCHEMA })
await agent(releaseNotesVersionBumpPrompt(), { model: 'haiku', phase: 'Epic 7 — Closure epilogue', schema: CYCLE_SCHEMA })
return { closed: 'claimed — verify against the repo before relaying (decisions.md §9 L7)' }
```

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
| 1 Tax cut | E1-001..005 | **yes, 2 lanes**: E1-003 (`tests/`, `scripts/verify-baselines.env`) ‖ E1-001+E1-002+E1-005 (`scripts/cycle_scope_gate.py` new, `scripts/pcgen_residue_gate.py` new, `scripts/pcgen-residue-baseline.env` new, their tests, `scripts/completion_atlas.py`, `scripts/shape_engine_boundary.py`, `scripts/missing_engine_tables.py`, `scripts/tests/test_shape_engine_boundary.py`, `scripts/verify.sh`, `scripts/verify-baselines.env` — **the baselines file is the one collision**: E1-003 changes the binary/test floor, E1-002/E1-005 the stage count; the second lane to land rebases and re-derives). E1-004 after all. | launch gates |
| 2 Sheet rule | E2-001..005 | no | **converter side:** `src/bin/sheet_rule_convert.rs` (new), `src/pcgen_import/sheet_rule/` (new), `data/sheet_rules/**` (new, generated), `scripts/token_coverage.py` (new) + test, `src/bin/v06_work_inventory.rs` (status + rung), `scripts/completion_atlas.py`, every status consumer; **live side:** `src/rules_core/sheet_rule.rs` (new — no PCGen), `src/rules_core/mod.rs`, `src/rules_core/corpus_loader.rs` (load `data/sheet_rules/`), `src/rules_core/pilot_compute/mod.rs` (the `SheetLine` on the computation result), `apps/desktop/src/characterHub/CharacterSheet.tsx` + tests, `apps/desktop/src-tauri/src/reach_gate.rs`; `docs/work-inventory.json` | Epic 1 |
| 3 Place and surface | E3-001..004 | no | `src/pcgen_import/sheet_rule/` (`applies` derivation), `src/rules_core/class_feature_pool_catalog.rs` (holdings lookup over `applies`), `src/bin/v06_work_inventory.rs` (C rung), `data/sheet_rules/**`, `docs/work-inventory.json`, `artifacts/epic-3-place-and-surface/` | Epic 2 |
| 4 Resolve and verify | E4-001..003 | no | `src/pcgen_import/sheet_rule/` (mapping rows), `data/sheet_rules/**`, `scripts/oracle_harness/`, `docs/work-inventory.json`, `artifacts/epic-4-resolve-and-verify/` | Epic 3 |
| 5 Residues | E5-001..005 | no | `src/rules_core/` (power/companion loaders over `applies`, class chassis), `apps/desktop/src-tauri/src/class_feature_pool_picker.rs`, `apps/desktop/src/characterHub/LevelUpDialog.tsx`, `data/corpus/beginner_box/**` (guarded generator only), `data/sheet_rules/**`, `docs/work-inventory.json`, `artifacts/epic-5-residues/` | Epic 4 |
| 6 PCGen exit | E6-001..004 | no | `src/rules_core/pilot_compute/formula_interpreter*.rs`, `bonus_stack_reader.rs`, `pre_tokens.rs`, `racial_sla.rs`, `trait_effects.rs`, `feat_effects.rs`, `wiring_class.rs`, `src/rules_core/cache_gen/**` → `src/pcgen_import/**`; every live `PcgenFormulaEvaluator` / `render_pcgen_desc` / `raw_tokens` call site (78 files by coarse grep); `apps/desktop/src-tauri/src/*_catalog.rs`, pickers, `class_feature_feat_bridge.rs`, `reach_gate.rs`; `src/bin/gen_*`, `src/bin/enrich_*` import paths; `artifacts/epic-6-pcgen-exit/` | Epic 5 |
| 7 Closure | E7-001..003 | no | package docs, `receipts.md`, `release-notes.md`, `docs/architecture/`, `docs/retro/` | Epics 1–6 |

**Epics 2–6 are sequential because every one of them writes `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json`, or `src/rules_core/`** (`decisions.md §9` L12). Within an epic, cycles are sequential
for the same reason. Concurrency in this bundle comes from **batch size**, not lane count.

**Every parallel agent gets `isolation: 'worktree'`.** Never more than 3 lanes building at once.

## 4. File-touch verification

**Run for real 2026-09-07 at `tranche/14` `5f6b18f4e3`:**

```
EXISTS   src/bin/v06_work_inventory.rs
EXISTS   src/rules_core/mod.rs
EXISTS   src/rules_core/wiring_class.rs
EXISTS   src/rules_core/pre_tokens.rs
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
EXISTS   scripts/denominator_gate.py                        (DEFAULT_GLOBS at :121, SD-34 scope)
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

`src/rules_core/class_feature_pool_catalog.rs`, `src/rules_core/feat_prereqs.rs`,
`apps/desktop/src-tauri/src/class_feature_pool_picker.rs` are named by SD-34 `decisions.md §17`
and the SD-33 register; **re-verify at the cut** (not listed above because this authoring
session did not `ls` them).

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

0. **Prove the batch and record the residue baseline before touching anything.**
   ```bash
   python3 scripts/cycle_scope_gate.py --min 500 <this cycle's scope flags> \
     || { echo 'UNDER FLOOR — bundle more mechanisms or take the whole remainder'; exit 1; }
   python3 scripts/pcgen_residue_gate.py --check \
     || { echo 'PCGEN RESIDUE ABOVE BASELINE AT START — the tree is already wrong; stop'; exit 1; }
   cp docs/work-inventory.json /tmp/wi-before-$RETRO_ACTOR.json
   ```
   (Epic 6 cycles and the Epic 1 gate-building cycles are exempt from the floor — they close
   zero units by design, `decisions.md §9` L6 — but not from the residue check.)
   The literal output line goes in the receipt. A cycle that cannot pass this does not start;
   it reports the scoped population and stops (a correct outcome — the orchestrator re-scopes).
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
     python3 scripts/pcgen_residue_gate.py --check      # must not have risen; --closure from AT-35-E6-004
     cargo run --locked --bin sheet_rule_convert -- --check   # from AT-35-E2-001 onward
     grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   # must print 0
     python3 scripts/completion_atlas.py --check
     python3 scripts/token_coverage.py --check          # from AT-35-E2-004 onward
     python3 scripts/shape_engine_boundary.py --check
     python3 scripts/missing_engine_tables.py --check
     python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'
     scripts/verify.sh --only pi-sweep
     ```
     The desktop crate and frontend run here **only if the cycle touched `apps/`**; otherwise
     they run at the epic wrap-up (§10). `cargo clippy --locked --tests -j 6` on the touched
     targets; fix warnings in the same cycle.
   - If a count changed (test binaries, records, stages), grep old **and** new numbers across
     `tests/`, `src/`, `apps/`, `scripts/` before commit.
4. Re-run both audits on the final diff.
5. **Compute the receipt rows mechanically:**
   ```bash
   python3 scripts/cycle_scope_gate.py --receipt \
     --before /tmp/wi-before-$RETRO_ACTOR.json --after docs/work-inventory.json
   ```
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
  `pcgen_live_files`** (revert the live-side read; the fix belongs on the converter side).

**A `## Open blockers` entry pauses the bundle** and is a request for an operator ruling. Two
dispositions only: clear it, or raise your hand and wait. **Under the sheet rule, "the engine
cannot model X" is not a blocker** — the record renders as words and the unit is done; a
subsystem someone wants later is that later bundle's scope.

**Disk usage:** after every parallel wave, `df -h /` and `git worktree list`; prune merged
worktrees. Never remove a `locked` worktree or one carrying unmerged commits.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_ -]*>' docs/release/SD-35-corpus-sheet-completion/ --include='*.md'
```

Every match must resolve to a real value or be a **documented** deferral. SD-35's documented
deferrals: the `tranche/15` cut, the `0.15.0` stamp, and §1's pasted outputs (`decisions.md
§10`, resolved at §1 item 8 and at the cut). Receipt-schema and command-shape placeholders in
§6/§7 (`<scoped paths>`, `<sha>`, `<cycle-id>`, `<this cycle's scope flags>`,
`<sd34-pr-number>`, `<tranche-15-cut-sha>`, `<bundle-launch-date>`, `<epic-start>`, `<lane-role-name>`,
`<epic-dir>`, `<criterion-id>`, `<epic>`, `<nnn>`, `<N>`, `<n>`, `<x>`, `<list>`, `<violations>`,
`<reason>`, `<filter>`, `<inventory>`, `<scope flags>`, `<type>=<count>`, `<lines per unit>`,
`<exact source text>`, `<enclosing fn name>`, `<class>` in `ground_<class>` / `CL=<class>`,
`<kind>` in per-kind gates, `<form>` in `sheet_rule_rendered:<form>`, `<key>` in
`data/sheet_rules/<book>/<kind>/<key>.json`, `<book>`) are
template literals a lane fills at run time, not values. The same list governs
`artifacts/README.md`, `technical-design.md §4`, and `epic-breakdown.md`.

## 10. Epic wrap-up (after every epic)

0. **The full gate, once:** `scripts/verify.sh` — every stage. Record wall time. Any red stage
   is fixed in a wrap-up correction cycle before the next epic dispatches; the fix cycle is
   exempt from the batch floor (it closes zero units by design, `decisions.md §9` L6).
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

**No `UNENFORCED` row.** Row 29 is partially mechanical (the ratio) and partially a reading
(what the lines bought); `risks-and-open-questions.md §10` tracks it.

## Cross-references

- `../../governance/workflow-instruction-template.md` — the template this is authored from.
- `../../governance/blocker-closure-doctrine.md` — enforced by §8 and §11 step 1.
- `../SD-34-book-completion/` — the predecessor; its atlas, tables, and fable review are SD-35's inputs.
- `../SD-34-book-completion/workflow-instruction.md §12` — the 26 inherited standing lessons.
- `docs/retro/sd35-corpus-sheet-completion-retrospective.md` — written at closure (§11 step 2).
- `.claude/skills/stc-authoring/SKILL.md` — the skill this package was authored with.
