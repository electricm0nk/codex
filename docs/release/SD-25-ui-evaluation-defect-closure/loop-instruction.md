# SD-25 Loop Instruction — UI-Evaluation Defect Closure, Hub-of-Hubs Refactor, PCGen Runner, Ingest Diagnostic Sketch

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> This file is the per-cycle procedure manual. **The dispatcher is the in-harness `Workflow` tool, driven from a live session** — not `/loop 60m /batch /goal ./loop-instruction.md` (does not run unattended) and not a headless `scripts/workflow-dispatch.sh` process (verified 2026-07-21: its `claude code --profile … --task …` invocation does not exist in the live CLI; `claude --help` has no `code` subcommand, no `--profile`/`--task` flags — see `decisions.md §10`). `scripts/workflow-dispatch.sh` remains as the deterministic per-epic concurrency/tiering **spec** (its `EPIC_PARALLEL`/`EPIC_SUBAGENT`/`PARALLEL_OVERRIDE`/`SUBAGENT_OVERRIDE` maps) that each `Workflow` call reads from and honors — it is not itself run as a process. Per `/governance/loop-instruction-template.md` + skill `workflow-orchestrated-dispatch`.

## 0. Bundle at a glance

- **Branch:** `tranche/5-3`
- **Board:** `codex-tranche-5`
- **First concrete build:** `0.5.98` (develop at `0.5.97` per template §1 §7)
- **Epics / criteria:** 8 / 26 declarative (+ ~8–15 dynamic; see `epic-breakdown.md` quick reference)
- **SD-24 carry-forward:** `./sd24-carry-forward-register.md` — full custody of SD-24's 41 `## DISCOVERED` entries + 4 `## TODO` remainders; the dispatchable ones land in Epic 3 (A2–A5) and Epic 7 (A1, A6, A8–A17)
- **Dispatch concurrency map:** see `decisions.md §3` and the per-epic concurrency table below

## 1. Pre-launch checklist (verified during drafting; see `README.md §1`)

The template's 8-item checklist is enforced, not just listed. See `README.md §1` for the actual command output captured at package-construction time.

## 2. Orchestration mode (per `/governance/loop-instruction-template.md §2`)

- **Dispatch mechanism:** the in-harness `Workflow` tool, driven session-by-session; `scripts/workflow-dispatch.sh` is the concurrency/tiering spec it reads, not a standalone process (decisions.md §10 — the script's own subprocess-invocation form does not exist in the live CLI).
- **Default subagent model:** Sonnet.
- **Tiering overrides:**
  - **Haiku:** Epic 8's `release-notes.md` cycle; Epic 8's version-bump housekeeping (Cargo.toml + package.json + tauri.conf.json edits are mechanical and don't need judgment).
  - **Opus:** Epic 8's adversarial-verification step (the canonical "judge-panel" passage on the closure-pipeline sub-steps at template §6); final completeness scan for the closure-readiness report.
  - **Everything else (Sonnet):** the criterion's actual RED → GREEN → re-audit work.
- **Concurrency shape:** decided explicitly per epic in `decisions.md §3` and the table below, at authoring time — not derived live.
- **Known-broken tooling for 8.2 (register C3):** `~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py`'s `parse_source_dirs_index` regex requires literal `||` cell delimiters, but the real `docs/architecture/README.md` index uses single-pipe rows — the script always returns an empty path→doc mapping and reports a false-negative "no architecture impact". Criterion 8.2 must perform the truth-up manually (as SD-24's closing session did) or fix the script first; never trust its no-impact output as-is.

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
| E7 Per-class residue | 7.1 + 7.2..7.M (dynamic) | no | one class-feature → one cycle; queue grows from `## DISCOVERED` | E6 |
| E7 Equipment/spell corpus intake (SD-24 carry-forward; register A8, A10–A17) | 7.N (4 items) | yes (CRB-description, APG-description, APG-spell-text, Bestiary-1) | 4 disjoint file-touch cycles, one per-book equipment/spell table module each; `isolation: 'worktree'` per `decisions.md §3` | E6 |
| E7 GE-07 pilot-shell-snapshot real implementation (SD-24 carry-forward; register A1) | 7.O | no | `load_pilot_shell_snapshot` Tauri command + `apps/desktop/src/boundary/loadPilotShellSnapshot.ts` | Open question Q5 (`risks-and-open-questions.md §4`) — operator design decision must land before the implementation cycle dispatches |
| E7 SD-24 documentation-staleness batch (register §B) | 7.P | no | docs-only, one batched cycle (Haiku): `../SD-24-beta-readiness-and-multiclass/{content-unit-inventory,technical-design,epic-breakdown,acceptance-and-verification}.md` — items B1–B4, B6–B7, B9–B11, B14 (B5/B8/B12 already corrected in-cycle; B13 = open question Q6, default no action) | none |
| E8 Closure Epilogue | 8.1–8.5 | no | final scan + architecture-truth-up + graphify-update + release-notes (Haiku) + version-bump (Haiku) + PR + merge-conflict-resolution. 8.1 = read-only scan (Sonnet); 8.2 = closure pipeline (Opus; see §2's C3 warning); 8.3 = Haiku; 8.4 = Haiku; 8.5 = `tranche/5-3 → develop` PR + merge (Sonnet, serial) | E7 |

**`parallel: yes` rows get `isolation: 'worktree'`** in the orchestrator script (template §3): the orchestrator spawns each parallel cycle with a worktree-clone so concurrent cycles don't step on each other's working-directory state.

**Register §B disposition (resolved 2026-07-21, operator delegated):** the documentation-staleness batch is criterion **7.P** — Epic 7 chosen over Epic 1 because Epic 7 is the SD-22/SD-24 coverage-backlog epic (the register already routes its dispatchable A-items there) and Epic 1 is deliberately a single-cycle governance gate that fires first. Haiku-tiered per the housekeeping doctrine; corrections target SD-24's historical planning docs, not SD-25's own.

## 4. File-touch verification (template §4 — required before §3 was filled in)

Verified during drafting by `ls`/`find` against the live repo on 2026-07-21:

- `apps/desktop/src/characterHub/` — verified, 30+ files including `characterHubRuntime.ts` (hub-runtime), `CharacterHubPage.tsx`, `CharacterSheet.tsx`, `ItemPickerModal.tsx`, `LoadCharacterScreen.tsx`, `CreateCharacterForm.tsx`.
- `apps/desktop/src-tauri/src/character_hub.rs` — verified, single Rust backend file (NOT `characterHub.rs`; the backend uses snake_case).
- `apps/desktop/src-tauri/src/main.rs` — the crate entrypoint where Tauri commands are registered. **There is no `lib.rs` in this crate** (confirmed by `find apps/desktop/src-tauri -name 'lib.rs'` returning nothing, 2026-07-21); an earlier draft of this row claimed `src/lib.rs` on the strength of a `find | grep -E '^lib.rs$'` pipeline that can never match (`find` emits full paths) — the empty result was misread as confirmation. Will be re-verified by cycle 0 of E2.
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

**File-touch grant (register C1):** every cycle's write scope comes from that criterion's own `./cycles/<epic>_<criterion>.md` doc and its own `epic-breakdown.md` row — never copied forward from the previous cycle's block. SD-24's invocation generator templated criterion 5.1's grant verbatim across 4 subsequent cycles before it was caught; the dispatcher must read each grant fresh.

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

   **Corpus-text collisions (register A9):** real ingested SRD text can legitimately contain forbidden tokens — SD-24 hit `hack` inside CRB's *Plant Growth* spell text. During Epic 7 corpus-intake cycles, a raw-grep hit inside an ingested corpus-text field is not automatically a violation: exclude it via the standing wired-integration audit test's named-bucket mechanism (the same way test files are already excluded) and document the exclusion in the cycle receipt — never edit real rules text to satisfy the grep.

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
- **Operator-monitored, outside cycle scope (register C2):** dense parallel-worktree phases (E3, E4, E7's corpus intake) accumulate sibling `target/` directories (up to ~10G each); SD-24 hit a linker Bus-error crash when the shared build volume reached 100%. Monitor disk during those phases and prune merged worktrees promptly after each parallel phase completes — SD-24's closing session freed 36GB from 9 stale worktrees by hand.

## 9. Pre-cycle assumption checks

Before cycle 1 of any epic, verify the epic's prerequisites in `epic-breakdown.md §Dependencies` are met. A cycle whose prerequisites are not met returns to the cycle-backlog with a "prerequisites unmet" reason.

## 10. Cross-references

- `/governance/loop-instruction-template.md` — canonical template.
- `governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline`.
- `docs/doctrine-external/identifier-discipline.md` + skill `identifier-discipline` (`~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/`). (Path corrected 2026-07-21 — an earlier draft cited `governance/identifier-discipline.md`, which does not exist.)
- `governance/wired-integration-stubs-registry.md`.
- `./sd24-carry-forward-register.md` — full custody of SD-24's 41 `## DISCOVERED` + 4 `## TODO` items; dispatchable assignments live in `epic-breakdown.md` Epics 3 and 7 (§B's documentation batch = criterion 7.P).
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator procedure.
- `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/` — receipt schema.
- `decisions.md` — bundle-specific ADRs.
- `epic-breakdown.md` — per-cycle stories.
- `progress.md` — live cycle state.
- `scripts/workflow-dispatch.sh` — the dispatcher itself.
- `./cycles/<epic>_<criterion>.md` — per-criterion task documents.
