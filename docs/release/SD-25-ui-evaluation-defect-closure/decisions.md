---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-07-21; bundle authored from /governance/loop-instruction-template.md + skill workflow-orchestrated-dispatch)
date: 2026-07-21
canonical_branch: tranche/5-3 (operator directive 2026-07-21)
kanban_board: codex-tranche-5 (reused after SD-24 closure PR lands)
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
---

# SD-25 — Decision Record

## 1. SD-25 scope is bundle-of-four-loads (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-25 ships four loads in one bundle because the operator's stated time pressure (~15 hours from operator pre-cursor on 2026-07-21 17:48:31; Anthropic cycle restart at 5am 2026-07-22) makes split-bundles expensive relative to one bundle with eight epics:

1. **Hub-of-Hubs refactor** (Epic 3) — `RuleSystemAdapter` trait + Pf1 extraction + StubAdapter + Tauri command routing. Per operator directive 2026-07-21 17:39:26 ("character hub as a hub of hubs so that each rule system can operate independently").
2. **PCGen Runner Scaffolding** (Epic 4) — Bash + Gradle + Python wrapper for one case. Per operator directive 2026-07-21 18:04:18 ("the generation of the gradle based character output seems like something that we could script"). SD-26 builds the library on top.
3. **Corpus Ingest Diagnostic Sketch** (Epic 5) — Tauri command + UI panel route returning per-book ingest status. Sketch shape; SD-26 fans out the full status table + flags + ETA.
4. **UI-Evaluation Discovered Backend Defects** (Epic 6) — discovery-driven. Per operator directive 2026-07-21 17:59:09 ("I'm going to assume we will find more defects to address in SD-25. Much of that will probably be a backend work that surfaces through my next evaluation of the UI").

Plus E1 + E2 (canonical governance + gating), E7 (deferred per-class residue), E8 (closure epilogue).

## 2. SD-25 inherits the Workflow-orchestrated dispatch (operator directive 2026-07-21)

**Decision:** SD-25's dispatch shape is the `Workflow` orchestrator per `/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`.

**Reasoning:**

1. **The legacy launch form doesn't run unattended.** The previous pattern (`/loop 60m /batch /goal <file>`) is documented in the new template as incompatible with unattended operation. SD-25 cannot ship a launch form that requires a human per invocation.
2. **Workflow is the working mechanism.** Author-once orchestrator at `scripts/workflow-dispatch.sh`, fans agents out in parallel where files are genuinely disjoint, serializes where they aren't. Per `decisions.md §3` for SD-25's per-epic concurrency map.
3. **Subagent tiering follows the template.** Default Sonnet; Haiku for E8.3 release-notes + E8.4 version-bump; Opus for E8's adversarial-verify step.

**Future bundles inherit by default.** `AGENTS.md §7` + `CLAUDE.md` + skill `workflow-orchestrated-dispatch` make this the canonical shape. SD-26 will drop its package against the same template.

## 3. Per-epic concurrency + tiering map

| Epic | Parallel? | Subagent tier | Notes |
|---|---|---|---|
| E1 Identifier Cleanup | no | Sonnet | Single cycle |
| E2 Operator Pre-Launch | no | Sonnet | Gating epic; cycle-by-cycle |
| E3 Hub-of-Hubs | yes (3.1, 3.2, 3.3, 3.5); no (3.4) | Sonnet | 3.4 = Tauri command-files, serial |
| E4 PCGen Runner | yes (4.1, 4.2, 4.3); no (4.4) | Sonnet | 4.4 = verification, serial |
| E5 Corpus Ingest Diagnostic | no | Sonnet | Single Tauri command + panel |
| E6 UI-Eval Defects | no | Sonnet | Dynamic queue; serial cycles |
| E7 Per-class residue | no | Sonnet | Dynamic queue; serial cycles |
| E7 Equipment/spell corpus intake (added 2026-07-21) | yes (CRB-description, APG-description, APG-spell-text, Bestiary-1) | Sonnet | SD-24 carry-forward; disjoint file-touch per item; `isolation: worktree` |
| E8 Closure Epilogue | no | Haiku (8.3, 8.4); Sonnet (8.1, 8.5); Opus (8.2 + final adversarial-verify) | Per-criterion tiering override |

**Per-`parallel: yes` row:** orchestrator script invokes the cycle with `isolation: 'worktree'` so concurrent cycles don't step on each other's working-directory state.

## 4. Build counter inheritance

**Decision (per `/governance/loop-instruction-template.md §1 item 7`):** SD-25's first concrete build value lands at **`0.5.98`** (develop is at `0.5.97` per `apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json` on develop). The `tranche/5-3` branch carries tranche-base=5; the build counter increments on every merge; major=0 until first main-publish.

**Version-source-of-truth files** (canonical, NOT root `Cargo.toml`):
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`
- `apps/desktop/src-tauri/Cargo.toml` (Cargo.lock refreshes on next `cargo check`)

## 5. Publish mode is move-not-copy (operator directive 2026-07-21)

**Decision:** SD-25 publishes by moving, not copying. Workspace-side planning directory deleted on the publish commit.

**Reason:** per operator directive 2026-07-21 13:05:29 ("SD-24 package placement, files physically move into the codex repo at `programs/codex/requirements/`"). The "second copy = stale source-of-truth = wrong answers from the harness" pattern is doctrine-of-record.

## 6. Tier-1 launch-gate dependency

**Decision:** SD-25 cannot dispatch Epic 3+ until SD-24 closure PR is merged to develop. Per duracon 2026-07-21 09:24:59 (the SD-N closure-to-develop pattern is the SD-(N+1) launch-gate dependency). The tier-1 gate is enforced by E2's criterion 2.3 (`## Status matrix` row).

## 7. Override flags (operator-pinned)

| Flag | Default | Set behavior |
|---|---|---|
| FLAG-A: STRICT-STOP-AT-DEADLINE | unset (grace-tail) | strict stop at operator's deadline |
| FLAG-B: DEFERRAL-TO-SD-26 | unset | defer all UI-discovered defects to SD-26 |
| FLAG-C: STUB-BOOKS-OFF | unset (post-grant per `wired-integration-stubs-registry.md`) | forbid stub bookings |

## 8. Operator-deferred shape decisions

- **Dispatch mechanism.** Workflow orchestrator is canonical per `/governance/loop-instruction-template.md §2`; `/loop /batch` is not used. Override only by operator-pinning `DISPATCH_MODE=loop-batch` env var (currently ignored by `scripts/workflow-dispatch.sh`).
- **Files in this folder.** 16 canonical files plus `scripts/` and `cycles/` per the Workflow shape.
- **Release notes shape.** Required sections per template: Summary, User-Visible Changes, Defects Fixed, Operational Notes, Verification Evidence, Known Issues, Update Eligibility.
- **Operator identity in commits.** `Todd Hintzmann <todd@hintzmann.net>` per memory (operator preference for git identity).

## 10. Dispatch is session-driven Workflow-tool orchestration, not a headless script (operator-confirmed 2026-07-21)

**Decision:** `scripts/workflow-dispatch.sh`'s `claude code --profile … --task …` invocation does not exist in the live CLI (`claude --help` shows no `code` subcommand and no `--profile`/`--task` flags). As shipped, the script's dispatch step would fail silently and loop forever on its `sleep`/no-op branch — precisely the failure mode the `workflow-orchestrated-dispatch` skill's v1.1.0 warning anticipated. **SD-25 dispatches via the in-harness `Workflow` tool, driven from this session**, not via the shell script running unattended. The script remains in the repo as the deterministic per-epic concurrency/tiering spec (`EPIC_PARALLEL`, `EPIC_SUBAGENT`, `PARALLEL_OVERRIDE`, `SUBAGENT_OVERRIDE` maps) that the session's `Workflow` calls read from and honor — it is a reference implementation, not the live dispatcher.

**Reasoning:** confirmed the concrete CLI gap rather than assume the skill's warning was stale; re-verifying `claude --help` at the point of first live dispatch (per skill v1.1.0 checklist item 1) surfaced the exact failure the skill predicted. Fixing the headless daemon form was assessed as higher-risk (opaque subprocess supervision, no live view — per memory `cloud-loop-orchestration-lessons`) than session-driven `Workflow` orchestration for a bundle of this size.

**Consequence:** `~/.hermes/.../workflow-orchestrated-dispatch/templates/orchestrator.sh` + `references/prior-bundle-ship-defect.md` should be updated to match (operator's out-of-repo profile; non-gating for SD-25's own launch).

## 11. Carry-forward A5 resolved: fold revision-advancing into `mutate_saved_character_at_root` (operator-confirmed 2026-07-21)

**Decision:** per `sd24-carry-forward-register.md` item A5, every character-mutation command routed through `mutate_saved_character_at_root` (`level_up_character`, `add_equipment_selection`, `add_spell_selection`, `appendToCharacter`) will advance `revision_id`, not just the dedicated `reSaveCharacter` command. This lands as part of criterion 3.2's `Pf1Adapter` extraction — a behavior change, operator-approved rather than deferred.

## 12. Carry-forward A1/Q5 deferred this bundle (operator-confirmed 2026-07-21)

**Decision:** the GE-07 `load_pilot_shell_snapshot` real-implementation design question (open question Q5, `risks-and-open-questions.md §4`) stays unanswered for SD-25. Criterion 7.O dispatches the design-decision request only; the hardcoded-fixture no-stub violation remains documented, not remediated, this bundle.

## 9. Cross-references

- `/governance/loop-instruction-template.md` (REPO-LOCAL CANONICAL).
- `/governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline`.
- `docs/doctrine-external/identifier-discipline.md` + skill `identifier-discipline` (path corrected 2026-07-21 — `/governance/identifier-discipline.md` does not exist).
- `/governance/wired-integration-stubs-registry.md` — Epic 3 StubAdapter entries land here.
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/SKILL.md`.
- `../docs/release/SD-24-beta-readiness-and-multiclass/decisions.md` — closed predecessor (Tier-1 gate).
