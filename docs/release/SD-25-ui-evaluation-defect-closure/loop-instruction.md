# SD-25 Loop Instruction — UI-Evaluation Defect Closure, Hub-of-Hubs Refactor, PCGen Runner, Ingest Diagnostic Sketch

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> This file is the per-cycle procedure manual. **The dispatcher is `scripts/workflow-dispatch.sh`**, not `/loop 60m /batch /goal ./loop-instruction.md` — the latter does not run unattended. Per `/governance/loop-instruction-template.md` + skill `workflow-orchestrated-dispatch`.

## 0. Bundle at a glance

- **Branch:** `tranche/5-3`
- **Board:** `codex-tranche-5`
- **First concrete build:** `0.5.98` (develop at `0.5.97` per template §1 §7)
- **Epics / criteria:** 8 / ~24
- **Dispatch concurrency map:** see `decisions.md §3` and the per-epic concurrency table below

## 1. Pre-launch checklist (verified during drafting; see `README.md §1`)

The template's 8-item checklist is enforced, not just listed. See `README.md §1` for the actual command output captured at package-construction time.

## 2. Orchestration mode (per `/governance/loop-instruction-template.md §2`)

- **Dispatch mechanism:** `Workflow` orchestrator at `scripts/workflow-dispatch.sh`, run continuously.
- **Default subagent model:** Sonnet.
- **Tiering overrides:**
  - **Haiku:** Epic 8's `release-notes.md` cycle; Epic 8's version-bump housekeeping (Cargo.toml + package.json + tauri.conf.json edits are mechanical and don't need judgment).
  - **Opus:** Epic 8's adversarial-verification step (the canonical "judge-panel" passage on the closure-pipeline sub-steps at template §6); final completeness scan for the closure-readiness report.
  - **Everything else (Sonnet):** the criterion's actual RED → GREEN → re-audit work.
- **Concurrency shape:** decided explicitly per epic in `decisions.md §3` and the table below, at authoring time — not derived live.

## 3. Per-epic parallel/sequential map

This is the per-epic concurrency decision; the orchestrator script's parallel-y/n flags derive from this table. File paths verified by `ls`/`find` against the live repo before publish (template §4).

| Epic | Criteria | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| E1 Identifier Cleanup | 1.1 | no | one cycle; reads source, writes audit summary | none |
| E2 Operator Pre-Launch | 2.1–2.5 | no | one cycle per check; gating epic | none |
| E3 Hub-of-Hubs Refactor | 3.1–3.5 | yes (3.1, 3.2, 3.3, 3.5); no (3.4) | 3.1 = new `apps/desktop/src-tauri/src/rule_system_adapter.rs`; 3.2 = extract `apps/desktop/src-tauri/src/pf1_adapter.rs` from `apps/desktop/src-tauri/src/character_hub.rs`; 3.3 = new stub; 3.5 = `apps/desktop/src/characterHub/` UI files. 3.4 = Tauri command-files (`append_to_character.rs`, `recompute_character.rs`, `re_save_character.rs`) — multi-file, serial | none |
| E4 PCGen Runner Scaffolding | 4.1–4.4 | yes (4.1, 4.2, 4.3); no (4.4) | 4.1 = `scripts/pcgen-run-character.sh`; 4.2 = `scripts/pcgen-normalize-output.py`; 4.3 = `tests/oracle_validation/pcgen_runner_smoke.rs`; 4.4 = verification cycle that runs all three | E3.4 |
| E5 Corpus Ingest Diagnostic Sketch | 5.1 | no | one Tauri command + one UI panel route | E3.4 |
| E6 UI-Evaluation Discovered Backend Defects | 6.1 + 6.N (dynamic) | no | one defect → one cycle → one cycle-receipt; queue grows from operator's UI-eval session | E5 |
| E7 Deferred Per-Class Work & Coverage Backlog | 7.1 + 7.N (dynamic) | no | one class-feature → one cycle; queue grows from `## DISCOVERED` | E6 |
| E8 Closure Epilogue | 8.1–8.4 | no | final scan + architecture-truth-up + graphify-update + release-notes (Haiku) + version-bump (Haiku) + PR + merge-conflict-resolution. 8.1 = read-only scan; 8.2 = run scripts; 8.3 = Haiku; 8.4 = Haiku; PR step serial | E7 |

**`parallel: yes` rows get `isolation: 'worktree'`** in the orchestrator script (template §3): the orchestrator spawns each parallel cycle with a worktree-clone so concurrent cycles don't step on each other's working-directory state.

## 4. File-touch verification (template §4 — required before §3 was filled in)

Verified during drafting by `ls`/`find` against the live repo on 2026-07-21:

- `apps/desktop/src/characterHub/` — verified, 30+ files including `characterHubRuntime.ts` (hub-runtime), `CharacterHubPage.tsx`, `CharacterSheet.tsx`, `ItemPickerModal.tsx`, `LoadCharacterScreen.tsx`, `CreateCharacterForm.tsx`.
- `apps/desktop/src-tauri/src/character_hub.rs` — verified, single Rust backend file (NOT `characterHub.rs`; the backend uses snake_case).
- `apps/desktop/src-tauri/src/lib.rs` (NOT `src-tauri/lib.rs`) — confirmed by `find apps/desktop/src-tauri/src -name '*.rs' | grep -E '^lib.rs$'` (will be re-verified by cycle 0 of E2).
- `tests/fixtures/rules_core/pf1_*_level*_*.txt` — verified, ~30 deterministic inputs already on disk (covered PCGen oracle inputs).
- `/home/ubuntu/workspace/repos/pcgen/gradlew` — verified, PCGen Gradle wrapper present.
- `/home/ubuntu/workspace/repos/pcgen/code/testsuite/base-xml.ftl` — verified per the legacy-route field in `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`.
- `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs` — verified (Oracle-harness schema surface; SD-26 will read/write these).
- `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml` — version-source-of-truth files.

## 5. Concurrent-write protocol (canonical; template §5)

Every cycle that commits and pushes to `tranche/5-3`:

```bash
git fetch origin tranche/5-3 && git rebase origin/tranche/5-3 && git push origin HEAD:tranche/5-3
```

On non-fast-forward rejection: repeat up to 5 times. After 5 failures, write `CLAIM-EXISTS` blocker to `progress.md` and stop. **No force-push.** Applies to code commits and shared-state files (`progress.md`, `receipts.md`).

## 6. Per-cycle procedure (template §6)

1. **Ensure the working tree/worktree is based on latest `tranche/5-3`** (§5's fetch+rebase).
2. **Define `BASE_BRANCH=$(git merge-base HEAD origin/develop)`** *once, before either grep block*.

   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)

   # Identifier audit — bundle-tag leaks in diff
   # NOTE: trailing \b deliberately omitted (found live in SD-24, 2026-07-21: \b never
   # matches between `_` and a following word char, so a trailing \b silently misses real
   # identifiers like `sd19_class_catalog` and only catches a bare standalone token).
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- \
     'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' \
     'scripts/**/*.sh' 'scripts/**/*.py' \
     ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' \
     || echo 'OK_NO_BUNDLE_TAGS'

   # Wired-integration four-check audit — forbidden patterns in shipping code
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- \
     'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' \
     ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' \
     || echo 'OK_NO_TOKENS'
   ```

3. **Implement the criterion TDD-style:** RED → confirm it fails for the intended reason → GREEN → run the relevant test suite.
4. **Re-run the dual-audit gate** on the final diff; both must show `OK_*`. Single-token violations are self-healable inline; re-audit and continue.
5. **Write the cycle receipt** at `artifacts/<epic>/<cycle-id>_cycle_receipt.md` (schema below).
6. **Commit + push** via §5's protocol.
7. **Update `progress.md` in place** via §5's protocol.
8. **Mint the kanban card** as a done-receipt on `codex-tranche-5` (per the bundle's assignee/daemon-hazard doctrine).
9. **Report:** criterion, files touched, commit SHA(s), dual-audit results, RED → GREEN evidence, receipt path, kanban card ID, discoveries, next-cycle plan.

## 7. Per-cycle receipt schema (unchanged from prior bundles)

```markdown
# Cycle <cycle-id> — <epic-name> / Criterion <n>

- **Card ID:** t_<hex>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violation list>
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS / <violation list>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Status:** complete | returned-to-backlog | DISCOVERED-forked
- **Notes:** <judgment calls, deferred items, audit-exclusion requests>
- **Discovery forwards:** <list of ## DISCOVERED entries added>
- **Next-cycle plan:** <what the next cycle picks up>
```

## 8. Self-heal posture

Carry forward the self-healable / non-self-healable split:

- **Self-healable (resolve inline, exit GREEN):** dirty tree, single-token audit violation, unrelated test-setup breakage, build-counter out of sync, `## DISCOVERED` duplicates.
- **Non-self-healable (write `## Open blockers`, exit FAIL):** working tree diverged from `tranche/5-3` needs manual rebase; two live cycles on conflicting files; SD-24 closure PR not merged (Tier-1 launch gate); `## DISCOVERED` queue > 10 entries (per SD-24's inheritance); RED → GREEN not preserved in artifact; cycle finds `success: true` from a fake operation; cycle finds inline mocks in shipping modules; cycle finds "Would …" return strings in shipping code.

## 9. Pre-cycle assumption checks

Before cycle 1 of any epic, verify the epic's prerequisites in `epic-breakdown.md §Dependencies` are met. A cycle whose prerequisites are not met returns to the cycle-backlog with a "prerequisites unmet" reason.

## 10. Cross-references

- `/governance/loop-instruction-template.md` — canonical template.
- `governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline`.
- `governance/identifier-discipline.md` + skill `identifier-discipline`.
- `governance/wired-integration-stubs-registry.md`.
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator procedure.
- `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/` — receipt schema.
- `decisions.md` — bundle-specific ADRs.
- `epic-breakdown.md` — per-cycle stories.
- `progress.md` — live cycle state.
- `scripts/workflow-dispatch.sh` — the dispatcher itself.
- `./cycles/<epic>_<criterion>.md` — per-criterion task documents.
