# SD-26 Loop Instruction — Ingest Strategy Revision + Rule-System Plumbing

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> Per-cycle procedure manual. **Dispatcher is `scripts/workflow-dispatch.sh`**, not `/loop 60m /batch /goal ./loop-instruction.md` (the latter requires a human per invocation; not unattended-runnable). Per `/governance/loop-instruction-template.md` + skill `workflow-orchestrated-dispatch`.

## 0. Bundle at a glance

- **Branch:** `tranche/5-4`
- **Board:** `codex-tranche-5`
- **First concrete build:** `0.5.99` (develop at `0.5.97`; SD-25 closure lands at `0.5.98`; SD-26 → `0.5.99`)
- **Epics / criteria:** 6 / 17 declarative + 21 dynamic (38 total)
- **Dispatch concurrency map:** see `decisions.md §3` and the table below

## 1. Pre-launch checklist

Per `/governance/loop-instruction-template.md §1`. The 8 items are captured in `README.md §1` with verbatim command output. Bundle author runs these for real during drafting.

## 2. Orchestration mode (per template §2)

- **Dispatch mechanism:** `Workflow` orchestrator at `scripts/workflow-dispatch.sh`. Per-criterion tier + concurrency encoded in the orchestrator's per-epic lookup tables.
- **Default subagent model:** Sonnet.
- **Tiering overrides** (per `decisions.md §3`):
  - **Haiku:** E6.3 release-notes + E6.4 version-bump (housekeeping).
  - **Opus:** E6.2 architecture closure-pipeline (template §2's adversarial-verify rule).
  - **Everything else:** Sonnet.
- **Concurrency shape:** per-epic, decided at authoring time. E3 (JSON cache build, per-book) is the primary parallel surface; E3 criteria 3.1–3.4 each touch a different book and run with `isolation: 'worktree'`.

## 3. Per-epic parallel/sequential map

| Epic | Criteria | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| E1 Identifier Cleanup | 1.1 | no | one cycle | none |
| E2 Oracle-Harness Comparator | 2.1–2.5 | no | one comparator + normalize + comparison logic across `src/oracle_validation/` | E6.2 dispatch (or E6 closure pre-step) |
| E3 JSON Cache Build (4 in-scope books) | 3.1–3.4 | yes (3.1, 3.2, 3.3, 3.4) | 3.1 = `data/corpus/core_rulebook/`; 3.2 = `data/corpus/advanced_players_guide/`; 3.3 = `data/corpus/advanced_class_guide/`; 3.4 = `data/corpus/beastiary/`. Disjoint per-book | E2 (the comparator reads from cache + library together) |
| E4 Book Stub Manifest (21 future-state books) | 4.1 (research epic output) | yes after E4.1 | post-E4.1, one cycle per book — each cycle writes an entry to `governance/wired-integration-stubs-registry.md` and a stub manifest at `data/stubs/<book>.json` | E3 (E4's per-book entries can run in parallel with E3.2-E3.4 after their respective schemas land) |
| E5 Doctrine-Cost Reduction | 5.1 | no | one audit cycle that walks the per-class cycle floor | E3 + E4 |
| E6 Closure Epilogue | 6.1–6.5 | no | final scan + architecture closure-pipeline + release-notes (Haiku) + version-bump (Haiku, `0.5.99`) + PR + merge | E5 |

**`parallel: yes` rows get `isolation: 'worktree'`** in the orchestrator script (template §3).

## 4. File-touch verification (template §4)

Verified during drafting by `ls`/`find` against the live repo on 2026-07-21:

- `apps/desktop/src/characterHub/` — verified, 30+ files including `characterHubRuntime.ts`, `CharacterHubPage.tsx`, `CharacterSheet.tsx`, `ItemPickerModal.tsx`, `LoadCharacterScreen.tsx`, `CreateCharacterForm.tsx`.
- `apps/desktop/src-tauri/src/character_hub.rs` — verified, single Rust backend file.
- `apps/desktop/src-tauri/src/rule_system_adapter.rs`, `pf1_adapter.rs`, `corpus_ingest_diagnostic.rs` — **NOT YET ON DISK.** Created by SD-25's E3 + E5 cycles. SD-26's Epic 2 + Epic 3 reference these by path; SD-25 closure PR is the precondition.
- `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs` — verified, Oracle-harness schema.
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` — verified pilot case.
- `tests/fixtures/rules_core/pf1_*_level*_*.txt` — verified ~30 deterministic inputs.
- `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` — verified 26 book directories.
- `~/workspace/repos/pcgen/gradlew` + `code/testsuite/base-xml.ftl` — verified per SD-25/SD-26 PCGen runner.
- `governance/wired-integration-stubs-registry.md` — verified, format includes operator-pinned metadata fields for `book_stub` entries (the new kind SD-26 E4 introduces).

## 5. Concurrent-write protocol (canonical; template §5)

```bash
git fetch origin tranche/5-4 && git rebase origin/tranche/5-4 && git push origin HEAD:tranche/5-4
```

On non-fast-forward rejection: retry up to 5 times. Then `CLAIM-EXISTS` blocker + stop. No force-push. Applies to code commits and `progress.md` / `receipts.md` / `data/corpus/*.json` / `data/stubs/*.json` (the JSON cache entries are shared-state files that concurrent cycles may edit).

## 6. Per-cycle procedure (template §6)

1. **Ensure on `tranche/5-4`.** §5 fetch+rebase.
2. **Define `BASE_BRANCH=$(git merge-base HEAD origin/develop)`** *once, before either grep*.

   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)

   # Identifier audit — bundle-tag leaks in diff
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- \
     'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' \
     'scripts/**/*.sh' 'scripts/**/*.py' \
     'data/**/*.json' 'governance/wired-integration-stubs-registry.md' \
     ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b' \
     || echo 'OK_NO_BUNDLE_TAGS'

   # Wired-integration four-check audit — forbidden patterns in shipping code
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- \
     'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' \
     ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' \
     || echo 'OK_NO_TOKENS'
   ```

3. Implement criterion TDD-style: RED → confirm fails for the intended reason → GREEN → run the relevant test suite.
4. Re-run dual-audit gate. Both `OK_*`. Single-token violations self-healable inline.
5. Write cycle receipt to `artifacts/<epic>/<cycle-id>_cycle_receipt.md`.
6. Commit + push via §5.
7. Update `progress.md` in place via §5.
8. Mint kanban card as done-receipt on `codex-tranche-5`.
9. Report.

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

Carry forward the self-healable / non-self-healable split from `decisions.md §3` + `risks-and-open-questions.md §1–§2`. Same shape as SD-25.

## 9. Pre-cycle assumption checks

Before cycle 1 of any epic, verify the epic's prerequisites in `epic-breakdown.md §Dependencies` are met. E3 + E4 depend on SD-25's Hub-of-Hubs closure (E5 path) — verify `tranche/5-3 → develop` carries the SD-25 closure commit before SD-26's Tier-1 launch-gate check fires.

## 10. Cross-references

- `/governance/loop-instruction-template.md` — canonical template.
- `governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline`.
- `governance/identifier-discipline.md` + skill `identifier-discipline`.
- `governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions (E4 entries land here).
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator procedure.
- `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/` — receipt schema.
- `decisions.md` — bundle-specific ADRs.
- `epic-breakdown.md` — per-cycle stories.
- `progress.md` — live cycle state.
- `scripts/workflow-dispatch.sh` — the dispatcher itself.
- `./cycles/<epic>_<criterion>.md` — per-criterion task documents.
