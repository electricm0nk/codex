# SD-24 Loop Instruction — Beta Readiness + Multiclass + Equipment Completeness

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
>
> Operating form: `/loop 1m /batch /goal ./loop-instruction.md`.
>
> This file is the supervisor's goal document. The loop ticks every 1 minute; each tick reads `## TODO` + `## DISCOVERED`, dispatches the highest-priority unclaimed item, and resolves one cycle. `/batch` is the concurrency primitive — file-touch partition enforces one cycle per file at a time.
>
> **Per operator directive 2026-07-21**: Claude Code handles looping and dispatching (the `/loop /batch` form is the dispatch driver). Kanban is the durable receipt layer only — cards are minted on `codex-tranche-5` only with a done receipt; kanban does NOT dispatch work. The loop-instruction file is the operator-edited boot-and-maintain manual.
>
> **Per operator directive 2026-07-21**: cycle dispatch model is **deterministic-seeded-then-dynamic**. The cycle walks the 35-criterion deterministic list during cycles 1-N; as `## DISCOVERED` entries accrue, they priority-bump into the dispatcher queue. The picker reads `## TODO` + `## DISCOVERED` together.

## 0. Bundle at a glance

- **Branch:** `tranche/5-2` (operator directive 2026-07-21)
- **Board:** `codex-tranche-5` (reused)
- **Cadence:** `/loop 1m /batch`
- **Test scope (multiclass):** Fighter + Wizard only, advancing to level 10
- **Test corpus (equipment):** full PF1 core rules + APG + ACG + Bestiary 1
- **Epics / criteria:** 8 / 35
- **First concrete build value:** `0.5.<current_build_at_launch>` (per the `<major>.<tranche-base>.<build>` scheme)

## 1. Pre-launch checklist (operator action only, before cycle 1)

Verify before the loop's first launch — each item below is a hard precondition. The loop refuses to dispatch a cycle if any item fails:

1. **`codex-tranche-5` kanban board is reachable.** Run `hermes kanban list-boards`; confirm `codex-tranche-5` is in the list.
2. **`tranche/5-2` branch is on origin.** Per operator directive 2026-07-21, another agent creates `tranche/5-2`; the operator verifies it exists and is pushed.
3. **SD-23 closure PR merged to develop.** Per duracon 2026-07-21 09:24:59, SD-23 is the SD-24 launch-gate dependency. Run `git log origin/develop --oneline | head -5` and confirm the SD-23 closure commit is HEAD of develop. If not, the loop does NOT start.
4. **Classic PAT present at `~/.config/gh/.claude_gh_token`** (per the kanban dispatcher's respawn-guard footgun doctrine in duracon 2026-07-04 12:41:37).
5. **Working tree clean:** `git status --porcelain` returns empty on `tranche/5-2`.
6. **Doctrines loaded:** `wired-integration-discipline` and `identifier-discipline` skills are in the loop's skill list. Verify with `hermes skills --profile god-emporer --list` (or whatever the operator's skill-loader equivalent is).
7. **Build counter captured in `decisions.md` §3** from develop's `Cargo.toml` workspace version.
8. **Artifact directories verified:** `artifacts/{epic_1,epic_2,epic_3,epic_4,epic_5,epic_6,epic_7,epic_8}/` exist and are empty. The first cycle of each epic writes its receipt there.

Then launch with:

```bash
cd docs/release/SD-24-beta-readiness-and-multiclass
/loop 1m /batch /goal ./loop-instruction.md
```

## 2. Per-cycle mechanics

### 2.1 Cadence

- `/loop 1m /batch /goal ./loop-instruction.md` — supervisor tick every 1 minute.
- One cycle per tick. The cycle either completes (`complete`) or returns to the cycle-backlog with a `## Open blockers` entry.
- `/batch` partitions cycles across worker batches by file-touch — one worker per file at a time.

### 2.2 Eligibility — what gets dispatched next

The dispatcher reads `./progress.md` `## TODO` + `## DISCOVERED` and selects the highest-priority unclaimed item. The priority rule:

1. Items from `## TODO` ordered by epic number (Epic 1 → Epic 8), criterion number within epic (e.g. criterion 3.2 before 3.5).
2. Items from `## DISCOVERED` ordered by insert timestamp (most recent first); within the same timestamp, smaller epic-number-first.
3. Tied items broken by: (a) cycles with the smaller epic-failure-count first; (b) cycles whose artifacts already exist; (c) cycles with shorter expected runtime.

A cycle is eligible when ALL of:

- Progress doc exists (`./progress.md` was created on cycle 0 by the loop itself).
- The criterion-row in `## TODO` is `pending` or `ready` (not `complete`, not `in-progress`, not `claimed-by-other-cycle`).
- The criterion-row's `touched_files` set does not overlap any in-flight cycle's `touched_files` set.
- No operator-pinned override flag suppresses the criterion (per `./risks-and-open-questions.md` Override Flags).
- The criterion-row's hard-stop conditions are met (e.g. Epic 4 cycle requires Epic 3's `## DONE` entry; Epic 5 cycle requires Epic 4's per-class coverage matrix is committed).

### 2.3 Per-cycle steps

1. **Read the prior-cycle context.** Open `./progress.md` `## Cycle log` last entry; pick up any open questions / partial work.
2. **Pick the next criterion** per §2.2's priority rule. Read `./epic-breakdown.md` for the criterion's verbatim acceptance text + per-cycle story.
3. **Verify the working tree is on `tranche/5-2` and clean:**
   ```bash
   git fetch origin tranche/5-2
   git checkout tranche/5-2
   git pull --rebase origin tranche/5-2
   git status --porcelain | wc -l   # expect 0; if non-zero, exit CLAIM-EXISTS
   ```
4. **Run the dual-audit gate** (per the operator-pinned 2026-07-20 directive — both skills load together):
   ```bash
   # Identifier audit (skill: identifier-discipline) — bundle-tag leaks in diff
   git diff --unified=0 "${BASE_BRANCH}...HEAD" \
     -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' \
     ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})\b' \
     || echo 'OK_NO_BUNDLE_TAGS'

   # Wired-integration four-check audit (skill: wired-integration-discipline) — forbidden patterns in shipping code
   BASE_BRANCH=$(git merge-base HEAD origin/develop)
   git diff --unified=0 "${BASE_BRANCH}...HEAD" \
     -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' \
     ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.tsx' ':!**/*.test.rs' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' \
     || echo 'OK_NO_TOKENS'
   # (Checks 2-4 from the wired-integration skill are run identically to SD-23's pattern.)
   ```
5. **Implement the cycle per the per-cycle story** in `./epic-breakdown.md`. TDD is mandatory per repo `AGENTS.md` §"Non-Negotiable Rules." RED → GREEN → re-audit, in that order.
6. **Re-run the dual-audit gate** on the updated diff after GREEN lands.
7. **Capture the cycle artifact** at `./artifacts/<epic>/<cycle>_cycle_receipt.md` following the schema in §3 below. Include: cycle id, criterion touched, files touched, dual-audit output (both gates), RED → GREEN evidence, discovered-work items, next-cycle plan.
8. **Commit with cycle SHA capture:**
   ```bash
   git add <cycle-touched-files> artifacts/<epic>/<cycle>_cycle_receipt.md
   git -c user.name='Todd Hintzmann' -c user.email='todd@hintzmann.net' \
       commit -m "feat(sd24): <criterion> (<row transition>)"
   git push origin tranche/5-2
   ```
9. **Update `./progress.md` in place** (do not rewrite from scratch):
   - Append a new entry to the `## Cycle log`.
   - Update the criterion-row in `## Status matrix` to `complete` (or `returned-to-backlog`).
   - If the cycle generated `## DISCOVERED` entries, append them to the `## DISCOVERED` block.
   - Move the criterion-row out of `## TODO` and into `## DONE` (with the criterion-row still visible in `## Status matrix` for traceability).
10. **Mint the kanban card on `codex-tranche-5`** (the ONE manual step where kanban is touched):
    ```bash
    hermes kanban --board codex-tranche-5 create \
      "SD24 <criterion> (<epic-section>) [cycle <cycle-id>]" \
      --assignee god-emporer \
      --workspace done-receipt \
      --initial-status done \
      --created-by claude-code \
      --priority 3 \
      --body "<card body per kanban-claude-code-execution-receipt/SKILL.md schema>"
    ```
    The card exists *because* the cycle's receipt was already written (at step 7). Per operator directive 2026-07-21: **cards are created on kanban only with a done receipt**. Kanban is not the dispatcher.
11. **Exit the cycle.** Print the standard 7-line report and end the cycle.

The supervisor restarts the cycle 60 seconds later (1-minute floor per `/loop 1m`). The next cycle reads `## TODO` + `## DISCOVERED` and picks the next criterion.

### 2.4 File-touch partition

Per the SD-22 / SD-23 doctrine, every cycle owns one file (or one file set under the same logical module). Two cycles in parallel must touch disjoint files.

| File / Directory | Cycle Owner | Concurrency |
|---|---|---|
| `src/rules_core/pilot_compute.rs` | Epic 5 (Multiclass) cycles | one cycle at a time per file |
| `src/rules_core/rules_tables/crb/class_fighter.rs`, `class_wizard.rs` | Epic 4 (Per-class audit) | one cycle per file |
| `src/rules_core/rules_tables/apg/*.rs`, `acg/*.rs` | Epic 4 (Per-class audit) on APG/ACG classes | one cycle per file; Epic 4 cycles serialize against Epic 5's multiclass-table changes |
| `src/rules_core/rules_tables/equipment/*.rs`, `armor/*.rs`, `spells/*.rs` | Epic 6 (Equipment) | one cycle per file |
| `apps/desktop/src-tauri/src/characterHub/characterHubRuntime.ts` | Epic 7 (Tauri command-surface) | one cycle at a time |
| `apps/desktop/src/characterHub/**/*.{ts,tsx}` | Epic 7 (Picker modals + Add Weapon/Armor/Spell onClick) | one cycle per file |
| `apps/desktop/src-tauri/src/<command>.rs` | Epic 7 (Per-command repair) | one cycle per file |
| `Cargo.toml`, `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json` | Epic 8 (Build version increment) | one cycle at a time per file |
| `artifacts/<epic>/<cycle>_cycle_receipt.md` | Per cycle, written by the cycle's own Step 7 | one cycle per file |
| `./progress.md` | Per cycle, written by the cycle's own Step 9 | append-only; each cycle appends to its own section |

Cross-epic serialization is mechanical: Epic 5's `pilot_compute.rs` changes cannot land while Epic 4's Wizard-class audit is in flight, because Epic 5's multiclass depends on Epic 4's coverage matrix being committed first (per the epic-priority sequencing rule — Epic 4 fires before Epic 5).

### 2.5 Discovery forwarding

A cycle that finds work outside the deterministic list (a stub pattern; a missing equipment field; a broken Tauri command) writes the discovery to:

```
./progress.md ## DISCOVERED
<ISO-8601 timestamp> | <epic-of-origin> | <criterion-of-origin> | <priority-bump-tag> | <description> | <suggested-epic-and-criterion>
```

The next cycle's picker reads `## DISCOVERED` first (priority-bump-tag ordering). On dispatch, the picker either lands the discovery as a new `## TODO` item (with the suggested epic/criterion) or skips it if the operator-pinned override flag suppressed it. `## DISCOVERED` grows during the bundle; `## TODO` reflects the live dispatch queue.

Discovery threshold: when `## DISCOVERED` exceeds 10 entries, the cycle writes `## Open blockers` with "DISCOVERED queue > 10 — operator override required" and pauses. Operator clears the queue or adjusts the priority-bump-tag ordering to restore dispatch.

## 3. Per-cycle artifact schema

Each cycle writes `./artifacts/<epic>/<cycle-id>_cycle_receipt.md` with the following structure:

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
- **Discovery forwards:** <list of `## DISCOVERED` entries added by this cycle, with suggested epic-and-criterion>
- **Next-cycle plan:** <what the next cycle should pick up>
```

The artifact is the durable receipt. The kanban card references it; the receipt is the truth.

## 4. Self-heal posture

The loop self-heals wherever the failure is mechanically resolvable. The operator returns from a multi-day run to a list of problems — not a stopped loop.

### 4.1 Self-healable conditions (resolve inline, exit GREEN)

| Condition | Self-heal |
|---|---|
| Working tree dirty from a prior failed cycle | `git checkout -- <file>` or `git reset --hard HEAD~1` |
| Identifier audit finds a single `sd24_*` etc. leak | rename inline; re-audit; commit as part of the same cycle |
| Wired-integration audit finds a single forbidden token | remove the token; re-audit; commit as part of the same cycle |
| Cycle's tests fail for an unrelated reason (broken test setup, missing fixture) | fix the test setup; do not carry the cycle forward |
| Cycle finds a stub the operator did not design | record in Stubs Registry as accidental debt; remediate in the same cycle or a follow-on; defer to `risks-and-open-questions.md` if the fix is non-trivial |
| Build counter out of sync with develop | re-read develop's `Cargo.toml`; update `decisions.md` §3 |
| Cycle's criterion-row in `## Status matrix` disagrees with `## DONE` | reconcile by re-running the cycle artifact generation |

### 4.2 Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Action |
|---|---|
| Working tree diverged from `tranche/5-2` in a way that needs manual rebase | Write to `## Open blockers`; exit FAIL |
| Two live `claude` processes on conflicting files | First wins; second writes to `## Open blockers`; exit FAIL |
| SD-23 closure PR not merged to develop | Tier-1 launch gate; cycle refuses to start |
| Epic 4 (Per-class audit) finds APG/ACG classes are *not* fully wired | Multiclass Epic 5 scope is restricted to Fighter + Wizard only (per operator 2026-07-21); defer APG/ACG-class multiclass to a follow-on bundle; document in `risks-and-open-questions.md` |
| Epic 6 finds the strict-field-coverage threshold cannot be met within SD-24's cycle budget | Write to `## Open blockers`; operator decides on threshold relaxation or deferral |
| Cycle's RED → GREEN transition is not preserved in the artifact | Cycle is re-run with RED → GREEN captured; do not mark `complete` |
| A cycle lands a PR-URL comment into the kanban card before the commit lands at origin (the respawn-guard footgun per duracon 2026-07-04 12:41:37) | Operator override to bypass the respawn-guard rule; cycle continues |

## 5. Operating posture (for the operator launching the loop)

1. **One launch command, run to closure.** `/loop 1m /batch /goal ./loop-instruction.md` — the loop runs to closure (every criterion `done` or every criterion has a real blocker in `## Open blockers`).

2. **Why 1m, not 60m.** Per operator directive 2026-07-21: "Each loop should pick up and run as soon as the previous finishes" + "I want that loop to be dynamic, and not use a timer." The 1-minute `/loop` floor is the closest Hermes gets to "no perceptible timer"; the actual cycle picker dispatches the next-best criterion as soon as the prior one exits. The 20-hour dense run to 5am-target aims for 100% utilization.

3. **Hard stop at 5am.** The operator's Anthropic cycle restarts at 5am. Grace tail: at 5am the current cycle completes (no new cycles spawn), and the harness writes a "stopped at cycle N" note to `## Open blockers`. Absolute ceiling 5:30am regardless. Strict stop at 5am is also fine if the operator prefers — both shapes are documented in `decisions.md` §5.

4. **Watch the progress doc, not the loop output.** If the log shows three cycles in a row with no landed commit, the loop is stuck on a structural problem.

5. **The kanban card is the receipt.** Each cycle mints a card on `codex-tranche-5` *after* the cycle's artifact is written. Per the operator's `complete`-receipt-only doctrine, kanban is receipt-only.

6. **Claude Code is the loop driver.** `/loop /batch /goal ./loop-instruction.md` is the engine; `./loop-instruction.md` is the operator-edited maintenance manual. The operator intervenes by editing this file or by overriding `## TODO` priority through the override flags.

7. **Override flags.** `./risks-and-open-questions.md` `## Override Flags` is the operator-pinned-rather-than-cycle-decided control surface. A flag of the form `FLAG-A: <description>` suppresses the named criterion row from dispatch until the operator clears the flag.

8. **The 20-hour dense run.** Aim for 100% utilization before 5am. With `/loop 1m` cadence and a 35-criterion seed list, the math is roughly: 35 criteria × ~20-40 minutes per cycle = 12-23 hours of work, plus dynamic discovery on top. The 1m floor is the dispatcher's event-driven heartbeat; the actual cycle runtime is whatever RED → GREEN → re-audit takes.

## 6. Cross-references

- `./scope-draft.md` — canonical handoff *what* (bundle intent, epics, criteria, cycle dispatch model)
- `./decisions.md` — bundle-specific ADRs (deterministic-then-dynamic dispatcher, multiclass scope, equipment scope, build counter inheritance, hard-stop shape)
- `./epic-breakdown.md` — 30 acceptance criteria across 8 epics, with per-cycle stories
- `./acceptance-and-verification.md` — closure gates + per-criterion artifact map
- `./risks-and-open-questions.md` — self-healable vs. non-self-healable split + override flags
- `./content-unit-inventory.md` — per-content-unit N-tuple (rust module / test fixture / cycle artifact / CommandName-or-ComponentName)
- `./technical-design.md` — architectural surface (Tauri command-surface repair, equipment-corpus delivery, multiclass dispatch shape)
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements
- `../../governance/no-stub-mvp-doctrine.md` — REPO-LOCAL CANONICAL wired-integration parent doctrine (skill: `wired-integration-discipline`)
- `../../governance/identifier-discipline.md` — REPO-LOCAL CANONICAL identifier-discipline sibling (skill: `identifier-discipline`)
- `../../governance/wired-integration-stubs-registry.md` — REPO-LOCAL CANONICAL stubs registry
- `../../governance/spec-domain-lifecycle.md` — spec-domain lifecycle routing
- `../SD-22/` — predecessor bundle (content-source ingest + DM toolkit; data source for Epic 4 + Epic 5)
- `../SD-23-character-mutation-and-wired-integration/` — active bundle on tranche/5-1 (Tier-1 launch-gate dependency)
- `../SD-21-campaign-manager-and-persistence/decisions.md §18` — operator's 2026-07-17 `<major>.<tranche-base>.<build>` build-version amendment (SD-24's first concrete value is `0.5.<current_build>`)
