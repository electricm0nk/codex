---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Workflow Instruction

Per-cycle launch procedure, eligibility checks, and dispatch discipline.

---

## 0. Bundle at a glance

- **Branch:** `tranche/16`
- **Epics / criteria:** 4 / 1+ (B: 9 criteria; A: 11; C: 10; D: 6)
- **Dispatch mechanism:** Workflow tool from live orchestrating session
- **First concrete build value:** 0.16.0

---

## 1. Pre-launch checklist

All commands run for real; output pasted below:

1. **Local-file kanban present.** `ls -la docs/release/SD-36-consolidation/kanban.md` → file exists and is readable.
2. **Bundle branch on origin.** `git ls-remote --heads origin tranche/16` → `<SHA> refs/heads/tranche/16` present.
3. **SD-35 closure PR merged.** `git log origin/develop --oneline | head -5` → PR #390 merge visible in ancestry.
4. **Working tree clean.** `git status --porcelain | wc -l` → `0`.
5. **Hotfix merged to develop.** `git log origin/develop --grep 'pcgen-pinned-tree-ci-guard'` → fix/pcgen-pinned-tree-ci-guard merged and CI green.
6. **Version captured.** Target version is **0.16.0** (not a placeholder). Surfaces: `.github/workflows/publish-tester-release.yml`, `apps/desktop/package.json`, `apps/desktop/src-tauri/Cargo.toml`, `apps/desktop/src-tauri/tauri.conf.json`, 7 desktop test fixtures.

---

## 2. Orchestration mode

- **Dispatch mechanism:** Workflow tool from the live session (not `/loop /batch`).
- **Default subagent model:** Sonnet (implementation + integration).
- **Exceptions:** Haiku for housekeeping (release notes, docs edits); Opus for adversarial verification (Epic D closure gate).
- **Concurrency:** Explicit per epic in §3.
- **Workflow script skeleton:** [see §2.4 below]

### 2.1 Agent environment

Every dispatched agent sets:
- `RETRO_ACTOR=<role>` (e.g., `RETRO_ACTOR=epic-b-dashboard` for Epic B)
- `mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"`

### 2.2 Orchestrator never edits shipped code

The orchestrating session reads and investigates; dispatched agents implement (RED → GREEN → receipt). If cycle scope differs from assumptions, pause and re-dispatch with corrected scope — never mutate code directly.

### 2.3 Retro event logging

Every cycle emits events via `scripts/retro.py` as work happens — corrections, incidents, deferrals, rework — not batched at cycle end.

### 2.4 Workflow script skeleton

```javascript
export const meta = {
  name: 'sd36-dispatch',
  description: 'SD-36: dashboard freeze, PCGen wall, bloat cuts, closure',
  phases: [
    { title: 'Epic B — freeze status, retire producers' },
    { title: 'Epic A — PCGen wall, crate codex-ingest' },
    { title: 'Epic C1 — source refactor' },
    { title: 'Epic C2 — test rewrite' },
    { title: 'Epic D — closure' },
  ],
}

phase('Epic B — freeze status, retire producers')
await pipeline([
  { id: 'B', prompt: 'Implement Epic B: freeze PF1e status page at 100%, retire supporting machinery (v06_work_inventory, support_state_matrix, reach_gate). See docs/release/SD-36-consolidation/epic-breakdown.md criteria B1–B9. TDD: RED (new frozen-status gate), GREEN (rewrite public status per D5, delete 8+ binaries/modules + 9 test files, desktop UI cleanup), verify (one pass: cargo test --no-run, bash scripts/verify.sh all stages green). Commit, push tranche/16.' }
], c => agent(c.prompt, { model: 'sonnet', phase: 'Epic B' }))

phase('Epic A — PCGen wall, crate codex-ingest')
await pipeline([
  { id: 'A', prompt: 'Implement Epic A: wall off PCGen in crates/codex-ingest. Move 47 tool bins, ~82 tests, pcgen_import/oracle_validation modules into crate. Route 36 src/rules_core [cfg(test)] modules to moved tests or fixture JSON. Rename SourceRef.lst_file → source_path (57 test files). Path rewrites: CARGO_MANIFEST_DIR → repo_root() (0 remaining). Desktop dev-dep only. See epic-breakdown.md criteria A1–A11. TDD + ONE pass: cargo test --no-run, bash scripts/verify.sh all green, crate-wall stage green, residue gate 0/0. Commit, push tranche/16.' }
], c => agent(c.prompt, { model: 'sonnet', phase: 'Epic A' }))

phase('Epic C1 — source refactor')
await pipeline([
  { id: 'C1', prompt: 'Implement Epic C1: split pilot_compute/mod.rs into ~36 submodules (class_*.rs, race_seams, combat, feat_pillars, skills, spellcasting, pools, companion, untabled, prestige, dispatch, unchained). New src/support/paths.rs consolidates 8 path-helper copies (repo_root, corpus_root, find_json_files). Clippy lock: add -- -D warnings. See epic-breakdown.md C1.1–C1.5. TDD + ONE pass: cargo test --list before/after identical, bash scripts/verify.sh all green. Commit, push tranche/16.' }
], c => agent(c.prompt, { model: 'sonnet', phase: 'Epic C1' }))

phase('Epic C2 — test rewrite')
await pipeline([
  { id: 'C2', prompt: 'Implement Epic C2: table-driven rewrite of tests/sd18_widening and tests/sd13_progression. Keep rosters + per-row files; add rows.rs with const ROWS; macro emits per-row [#test]. Bespoke tests (~770) stay. Proof: --list diff identical (2219 entries kept). Move tests/sd16-e5-f1/test_branch_promotion_guard.sh to tools/ci/. See epic-breakdown.md C2.1–C2.6. TDD + ONE pass: --list IDENTICAL, bash scripts/verify.sh all green. Commit, push tranche/16.' }
], c => agent(c.prompt, { model: 'sonnet', phase: 'Epic C2' }))

phase('Epic D — closure')
await pipeline([
  { id: 'D', prompt: 'Epic D closure: refresh docs/architecture/ (overview, rules-engine, desktop-app, status, testing) for touches in B/A/C. Write docs/retro/sd36-retrospective.md from scripts/retro.py summary. Update docs/release/SD-36-consolidation/release-notes.md with before/after baseline table (re-derived commands). Record receipts: architecture-truth-up, graphify-update. Then: graphify against final tree; gh pr create --base develop for tranche/16; wait for merged PR and CI green. See epic-breakdown.md D1–D6. Commit, push. Do not end turn; check git log and PR status for real.' }
], c => agent(c.prompt, { model: 'sonnet', phase: 'Epic D' }))
```

### 2.5 Never resume a dispatched agent

Run to completion inside the turn — commit and push before ending. No "waiting for background job" messages; they lose entire cycles.

---

## 3. Per-epic parallel/sequential map

| Epic | Criteria | Parallel? | File sets | Gated on |
|---|---|---|---|---|
| B (freeze + retire) | B1–B9 | no | `src/bin/`, `src/rules_core/`, `apps/desktop/src-tauri/src/`, `apps/desktop/src/`, `scripts/`, `tests/` | none |
| A (PCGen wall) | A1–A11 | no | `crates/` (new), `src/pcgen_import/`, `src/oracle_validation/`, `src/rules_core/` (test module eviction), `tests/`, `apps/desktop/src-tauri/` | B complete |
| C1 (source refactor) | C1.1–C1.5 | no | `src/rules_core/pilot_compute/` (split), `src/support/` (new), `src/bin/`, `scripts/verify.sh` | A complete |
| C2 (test rewrite) | C2.1–C2.6 | no | `tests/sd18_widening/`, `tests/sd13_progression/`, `tests/common/`, `tools/ci/` | C1 complete |
| D (closure) | D1–D6 | no | `docs/architecture/`, `docs/retro/`, `docs/release/SD-36-consolidation/`, GitHub PR | C2 complete |

---

## 6. Per-cycle procedure (inside dispatched agent)

Every cycle follows: RED (gate), implement (TDD), verify, commit.

1. **Pre-flight:** `git status --porcelain` (clean); `git branch -a | grep $CARGO_TARGET_DIR` (only this worktree writes).
2. **RED gate:** Criterion's acceptance command should fail (or not exist yet).
3. **Implement TDD-style:** Write test, confirm it fails for the right reason, implement smallest change, run scoped test suite.
4. **Per-cycle verify:** `cargo test --locked --no-run -j 6`, scoped suites (`--lib` + touched module binaries), Python gates (cyclopent, residue, etc. as applicable).
5. **Commit:** `git add <files>` (explicit, not `-A`), `git status --porcelain` (verify staging), `git commit -m "..."`, `git push -u origin <branch>`.
6. **Receipt:** One row per cycle in `docs/release/SD-36-consolidation/artifacts/<epic>/cycle-N-receipt.md`: what was done, lines changed, test counts, gate result, any rework.

---

## 10. Epic wrap-up (end of each epic, before dispatch next)

1. Confirm all criteria green (acceptance commands pass).
2. Commit any remaining pending work (baselines updated, receipts written).
3. Record epic completion timestamp and notes in `progress.md` and `kanban.md`.
4. If this is not the last epic (C2 → D), **dispatch the next epic immediately** (no turn-ending summary; summary describes work already done).

---

## 11. Bundle closure (Epic D, after all work is complete)

1. **Retrospective:** `scripts/retro.py summary --since <launch date>` → `docs/retro/sd36-retrospective.md` (recorded lessons, by-actor breakdown, incident summary).
2. **Worktree sweep:** `git worktree list`, delete all (none should exist; each epic's lane cleaned itself). `git branch | grep -v develop | wc -l` should match traced branches. Confirm `tranche/15` is deleted (merged, no longer needed).
3. **Architecture-truth-up:** `python3 ~/.hermes/.../architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-36-consolidation/receipts.md --bundle SD-36` (edits touched docs in place).
4. **Graphify:** `python3 ~/.hermes/.../update_graphify.py --integration-target develop --receipts-md docs/release/SD-36-consolidation/receipts.md --bundle SD-36` (runs graphify against final tree).
5. **PR and merge:** `gh pr create --base develop --title 'SD-36 consolidation' --body '...'` (PRdescription from release-notes + forward-scope-register); operator merges.
6. **CI green:** `gh run watch <latest>` → "Publish tester release" succeeds; `gh pr list --state merged --base develop | grep tranche/16` (confirmed merged).
7. **Done.** Record closure timestamp and all gate results in `receipts.md`.

---

