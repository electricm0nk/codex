# SD-20 Rules Engine Completeness — Operator-Driven Loop Instruction

---
title: SD-20 — Rules Engine Completeness (Per-Character Tabletop-Readiness, Any Class Any Level 1-20) — Operator-Driven Loop Instruction
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
canonical_branch: tranche/4 (operator directive 2026-07-16; slash form per prior naming convention)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
mirror_of: /home/workspace/SD-20-rules-engine-completeness-scope-draft.md
kanban_board: codex-tranche-4 (operator directive 2026-07-16; new board to separate SD-20 cycles from the codex-tranche-3 chassis-lane boards; **board created 2026-07-16**; the loop's Step 10 mint uses `--board codex-tranche-4` explicitly so it works regardless of operator's default-board setting)
---

This file is the body of the goal the `/loop 60m /batch /goal ./loop-instruction.md` invocation runs. (One launch, run to closure. `/batch` enables concurrent streams for epics 2/3/4/5 against the file-touch partition; epic 1 and epics 6/7/8 are single-stream by the dependency graph.)
It is **self-sufficient**: no interactive prompts, no mid-loop questions to the
operator, no shared state with anything other than the on-disk files
named here. The loop runs it; the loop restarts every 60 minutes; the loop
dies when the operator stops it (or, for `/batch`, when the supervisor's
streams all reach closure / block).

This file is **fully self-contained**. It does not read from, look up, or
inherit procedural mechanics from any other bundle's loop-instruction. The
cycle mechanics used here are captured below in full. If a future session
needs to recover SD-20's cycle mechanics from a clean checkout, this file
is sufficient on its own.

The loop uses the **matured** operator-loop model — the same procedural shape SD-13 established (1-cycle-at-a-time, single criterion per cycle, post-mortem kanban card, atomic direct commit to `tranche/4`, no ephemeral branches). No live inheritance from any other bundle's loop file; the procedural mechanics are internalized below:
- Linear commit-to-tranche/4 (no ephemeral feature branches; no PRs; no auto-merge).
- Per-cycle kanban card as post-mortem record (on `codex-tranche-4`).
- Per-cycle progress-doc entry appended to SD-20's own progress file `./progress.md`.

## What this loop does

Ground SD-20 — per-character rules engine completeness (tabletop-readiness) — toward the load-bearing gate in `./scope-draft.md` §1.8 (any of the 11 core classes at any level 1-20 plugs into the printed-sheet cell map per the broadened acceptance criterion). Working in bounded cycles against the integration branch `tranche/4` (per operator directive 2026-07-16; SD-20's branch is `tranche/4`, NOT `tranche/3`; slash form per prior naming convention). Each cycle lands one acceptance criterion. SD-20 runs in parallel with SD-21 (campaign manager + Drive persistence + APG + ACG); the two bundles share `tranche/4` (subject to SD-21's launch-branch decision) and the `codex-tranche-4` board (separate from the chassis-lane `codex-tranche-3` boards). **Each bundle has its own progress file** — SD-20's loop writes exclusively to `./progress.md`; SD-21's loop writes exclusively to `~/workspace/SD-21-campaign-manager-and-persistence-progress.md`.

See `./decisions.md` §6 for the rationale. SD-13's mature loop model (1-cycle-at-a-time, single criterion per cycle, post-mortem kanban card, atomic direct commit to `tranche/4`) is internalized in the §Per-cycle procedure below.

## Required reading (every cycle)

### 1. Canonical handoff doc

```
cat /home/ubuntu/workspace/SD-20-rules-engine-completeness-scope-draft.md
```

This is the canonical scope doc. The 15 acceptance criteria live here by section number (§1.1 epic 1, §1.2 epic 2, ..., §1.8 epic 8, §2 promotion gate). Each criterion's acceptance criterion prose and concrete corpus/code pointers live here.

### 2. Progress doc (SD-20's own; loop's working memory)

```
cat /home/ubuntu/workspace/SD-20-rules-engine-completeness-progress.md
```

This is SD-20's own progress doc — separate from SD-18's, SD-19's, and SD-21's. Each bundle has its own progress file (per operator directive 2026-07-16: "each should use it's own progress file set"). Created on first run if missing; frontmatter mirrors SD-18's progress doc shape (`title`, `mirrors` pointing at the scope draft, `created`, `snapshot_as_of`). Loop's claim protocol lives here under a single `## SD-20 cycles` section. SD-20 is the only bundle writing to this file. Each epic maintains `done` / `in-flight` / `open` status rows with cycle-id, commit SHA, and card id.

### 3. Required reading from SD-19 (the table-store pattern SD-20 consumes)

```
grep -A 9 "Source-book subdirectories" /home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md
# and
grep -A 30 "## 3. " /home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md
```

The canonical Paizo-table store (`src/rules_core/rules_tables/crb/`) is the load-bearing authority surface for every SD-20 epic. Do not re-derive; cite these sections when explaining eligibility, route around them when picking the next cycle, and update the SD-20 progress-doc section when they are no longer the binding constraint.

### 4. Live git state

```
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/4
git log origin/tranche/4 --oneline -5
git worktree list --porcelain
```

(No `git ls-remote origin | grep -E 'loop/tranche<N>-cycle-'` check — SD-20 has no feature branches.)

### 5. In-flight detection

```
ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep
```

If any `claude` process is running with a prompt that names a specific SD-20 acceptance criterion, do NOT pick that criterion. Cycle exits with `CLAIM-EXISTS` status; loop restarts.

## Concurrency rules (read first, obey always)

These rules are structural. Two concurrent cycles that touch the same file are guaranteed to collide; the loser will be Tech-Priest (or the operator) having to reconcile.

### File-touch partition (the hard rule)

The SD-20 cycle surface is concentrated in these files:

| File | Purpose | Cycles that may touch it |
|---|---|---|
| `src/rules_core/contract.rs` | NEW; the `CharacterInput` / `PilotReceipt` types and printed-sheet cell map. Touched only by epic 1's cycles (definition) — other epics read but don't edit. | One cycle at a time (epic 1's cycles). |
| `docs/SD-20/boundary-contract.md` | NEW; the boundary contract artifact. Epic 1's cycles only. | One cycle at a time. |
| `tests/fixtures/wire/sd20/<file>.json` | NEW directory; golden JSON fixtures. Per-epic cycles add their fixture on first cycle of that epic. | One cycle per file (the file's owning criterion). |
| `src/rules_core/spellbook.rs` | NEW; the spellbook engine epic's parent module. Epic 2's cycles only. | One cycle at a time. |
| `src/rules_core/spellbook/<school>.rs` | NEW directory; per-school contribution functions (9 files). Epic 2's cycles only — one file per cycle. | One cycle per file (the file's owning school). |
| `src/rules_core/feat_prereqs.rs` | NEW; the feat prerequisite engine epic's parent module. Epic 3's cycles only. | One cycle at a time. |
| `src/rules_core/feat_prereqs/<category>.rs` | NEW directory; per-feat or per-category functions. Epic 3's cycles only — one file per cycle. | One cycle per file (the file's owning category). |
| `src/rules_core/skill_allocation.rs` | NEW; the skill-rank allocation engine epic's module. Epic 4's cycles only. | One cycle at a time. |
| `src/rules_core/equipment_effects.rs` | NEW; the equipment-effect engine epic's parent module. Epic 5's cycles only. | One cycle at a time. |
| `src/rules_core/equipment_effects/<category>.rs` | NEW directory; per-category functions (4 files). Epic 5's cycles only — one file per cycle. | One cycle per file (the file's owning category). |
| `src/rules_core/damage_total.rs` | NEW; the damage-total engine epic's module. Epic 6's cycles only (sequential after epic 5). | One cycle at a time. |
| `src/rules_core/level_up.rs` | NEW; the Level Up grant model epic's parent module. Epic 7's cycles only (after epics 2–6 close). | One cycle at a time. |
| `src/rules_core/level_up/<class>.rs` | NEW directory; per-class functions (11 files). Epic 7's cycles only — one file per cycle. | One cycle per file (the file's owning class). |
| `tests/sd20_<criterion>.rs` | Per-cycle test file. | One cycle per file (its owning criterion). |
| `tests/sd20_tabletop_readiness_integration.rs` | NEW; the integration-closure epic's end-to-end test. Epic 8 only. | One cycle at a time. |
| `tests/fixtures/wire/sd20/human_fighter_level_1_tabletop.json` | NEW; the canonical tabletop scenario fixture. Epic 8 only. | One cycle at a time. |

The chassis and corpus-aware seam files (`pilot_compute.rs`, `pilot_compute_corpus.rs`, `support_state_matrix.rs`) stay untouched by SD-20's epic capability slices — SD-19 owns the trunk file partition and SD-20 produces into new modules, not into the trunk.

### Per-cycle spawn budget (the default)

Default: **1 cycle at a time.** Reason: identical to SD-19's and SD-21's. The file-touch partition collapses any parallel attempt into a serial one for the shared parent module (e.g. `spellbook.rs` is touched by every epic-2 cycle). Two cycles in parallel means two cycles racing on the same parent module.

To run more than one cycle in parallel you must show that the second cycle touches a disjoint file set. That's possible only when the cycles are in different epics whose parent modules are disjoint (e.g. one cycle in epic 2 spellbook and one in epic 3 feat prereqs — they touch `spellbook.rs` and `feat_prereqs.rs`, which are disjoint). For any cycle that needs to touch the same parent module as another cycle, **1 cycle at a time is the rule**.

## Per-cycle procedure (the steps, in order)

### Step 1 — Pick a criterion

From the SD-20 progress doc's `## SD-20 cycles` `open` list, pick the smallest unclaimed acceptance criterion. Priority order:

1. **Epic 1 cycles first** (boundary contract + wire-fixture parity tests). Without the `CharacterInput` / `PilotReceipt` types and the parity fixtures, no other epic can produce work the engine contract demands.
2. **Epic 2 (spellbook), Epic 3 (feat prereqs), Epic 4 (skill ranks), Epic 5 (equipment effects) cycles next**, in parallel if the operator hosts multiple loop channels. These four epics all depend on SD-19's table store only and on the boundary contract.
3. **Epic 6 (damage total)** sequentially after epic 5.
4. **Epic 7 (Level Up grants)** integrates after epics 2–6 close.
5. **Epic 8 (tabletop-readiness integration closure)** is the integration milestone; it lands after every other epic.

**Eligibility check.** A criterion is eligible when:

1. The criterion has not yet reached `done` per the progress doc.
2. No live `claude` process is working on that criterion (in-flight detection above).
3. The chosen school / category / class is **actually computable** from the existing engine surface — i.e. it is a new structured-data population in the appropriate epic module, OR it is an extension to the boundary contract, NOT a new subsystem. New subsystems are trunk-level decisions, not cycle decisions.

When several criteria tie on priority above, prefer the one that has not had a cycle attempted in the last 3 cycles (read the progress doc's `## SD-20 cycles` section to check). The loop's job is to advance the **frontier**, not to retry the same criterion forever.

### Step 2 — Pick the criterion's work-unit

- **Epic 1**: one class of boundary-contract functionality per cycle (e.g. `CharacterInput` types land first, then `PilotReceipt` types, then printed-sheet cell map, then first parity fixture for boundary contract itself).
- **Epic 2**: one PF1 spell school per cycle (abjuration, then conjuration, then divination, ..., then universal). Each cycle lands one school's contribution function plus its spell-effect round-trip.
- **Epic 3**: one feat category per cycle (e.g. general feats, then combat feats, then metamagic feats, etc.). Each cycle lands one category's evaluation function.
- **Epic 4**: one skill-class category per cycle (e.g. class-skill handling, cross-class-penalty handling, untrained-use handling, max-rank-cap handling).
- **Epic 5**: one CRB equipment category per cycle (`arms_armor`, then `general`, then `magic_items`, then `equipmods`).
- **Epic 6**: one damage-class criterion per cycle (base-dice round-trip, then STR-modifier handling, then weapon-enhancement modifier, then feat-effect modifier, then critical-threat-range, then critical-multiplier).
- **Epic 7**: one core class per cycle (barbarian, then bard, ..., then wizard). Each cycle lands one class's `LevelUpPlan` for level N+1 against the published CRB table.
- **Epic 8**: not a cycle — the integration-closure epic is the single test fixture + single integration test file. It lands in one slice (or one cycle if the operator hosts it as a single cycle).

### Step 3 — Verify the working tree is on tranche/4

```bash
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/4
git checkout tranche/4
git pull origin tranche/4
git status --porcelain | wc -l   # expect 0; if non-zero, exit CLAIM-EXISTS
```

### Step 4 — Write the failing test first

Add `tests/sd20_<criterion>.rs`. Mirror the shape of the most recent sibling cycle's test file. The test must fail for the intended reason when run against `origin/tranche/4` as the base.

```
cargo test --locked --test sd20_<criterion> 2>&1 | tail -40
```

Capture the failing output. It is the RED evidence.

### Step 5 — Implement the smallest change that makes the test pass

For SD-20 cycles, the change is one of:

- **Epic 1 extension to the boundary contract types** in `src/rules_core/contract.rs`. Add fields, fix structure, document cross-references.
- **Epic 1 extension to the boundary contract artifact** at `docs/SD-20/boundary-contract.md`. Three sections (CharacterInput shapes, PilotReceipt fields, printed-sheet cell map).
- **Epic 1 new wire-fixture parity test** at `tests/fixtures/wire/sd20/<criterion>.json`. JSON format per `technical-design.md` §1.2.
- **Epic 2 spellbook engine extension** in `src/rules_core/spellbook.rs` or `src/rules_core/spellbook/<school>.rs`.
- **Epic 3 feat prerequisite engine extension** in `src/rules_core/feat_prereqs.rs` or `src/rules_core/feat_prereqs/<category>.rs`.
- **Epic 4 skill-rank allocation engine extension** in `src/rules_core/skill_allocation.rs`.
- **Epic 5 equipment-effect engine extension** in `src/rules_core/equipment_effects.rs` or `src/rules_core/equipment_effects/<category>.rs`.
- **Epic 6 damage-total engine extension** in `src/rules_core/damage_total.rs`.
- **Epic 7 Level Up grant model extension** in `src/rules_core/level_up.rs` or `src/rules_core/level_up/<class>.rs`.
- **Epic 8 integration-closure fixture + test** at `tests/fixtures/wire/sd20/human_fighter_level_1_tabletop.json` and `tests/sd20_tabletop_readiness_integration.rs`.

For all paths, the change must be in the appropriate epic file. The forbidden write scopes are documented in `./risks-and-open-questions.md`.

Run:

```
cargo test --locked --test sd20_<criterion> 2>&1 | tail -40
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
```

All three must be green. Capture the output. It is the GREEN evidence.

### Step 6 — Commit, push directly to tranche/4

```
git add src/rules_core/contract.rs \
        src/rules_core/spellbook.rs \
        src/rules_core/feat_prereqs.rs \
        src/rules_core/skill_allocation.rs \
        src/rules_core/equipment_effects.rs \
        src/rules_core/damage_total.rs \
        src/rules_core/level_up.rs \
        docs/SD-20/boundary-contract.md \
        tests/sd20_<criterion>.rs \
        tests/fixtures/wire/sd20/<file>.json
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd20): <criterion> (<row transition>)"
git push origin tranche/4
```

The commit lands directly on `tranche/4`. Capture the commit SHA — it is the durable receipt (recorded as `merge_receipt_sha` in the card body and progress doc, by analogy with SD-19's atomic-slice receipt pattern).

### Step 7 — Open the PR (NOT APPLICABLE to SD-20)

SD-20 has no PRs. Per `decisions.md` §6 (no-branches convention): every cycle commits directly to `tranche/4`. The `tranche/4 → develop` promotion PR is operator-driven only and happens once at SD-20 closure, not per cycle.

### Step 8 — Auto-merge to tranche/4 (NOT APPLICABLE to SD-20)

SD-20 has no auto-merge. The commit is already on `tranche/4` by construction.

### Step 9 — Cleanup (NOT APPLICABLE to SD-20)

SD-20 has no ephemeral branch to clean up. The next cycle's Step 3 checkout handles any stale working-tree state.

### Step 10 — Mint the kanban card (post-mortem record)

```
hermes kanban --board codex-tranche-4 create \
  "SD20 <criterion> (<epic-section>) [cycle <cycle-id>]" \
  --assignee operator \
  --workspace scratch \
  --initial-status done \
  --created-by operator \
  --priority 3 \
  --body "<card body per schema below>"
```

Card body schema:

```
epic: SD-20
criterion_section: <scope doc section reference, e.g. "§1.2 Epic 2 — Spellbook engine">
row_or_kind: contract:input | contract:receipt | contract:cell_map | contract:fixture | spellbook:<school> | feat:<category> | skill:<category> | equipment:<category> | damage:<criterion> | levelup:<class> | integration:tabletop_readiness
evidence_tier_before: <previous matrix row state>
evidence_tier_after: <new matrix row state after this commit>
merge_receipt_sha: <commit SHA on tranche/4>
cycle_id: <ISO-8601 timestamp>
cargo_test_summary: <test summary string>
clippy_signal: clean | dirty
cycle_timing_seconds: <N>
self_heals_applied: <list, empty if none>
next_required_uplift: <recommendation for next iteration>
corpus_existence_verified: yes — <corpus path> :: <KEY: used>
rule_set_used: Crb
```

### Step 11 — Update the progress doc

Edit `./progress.md` in place:

1. Update the `snapshot_as_of` line in the frontmatter to the current `tranche/4` HEAD short SHA. (SD-20's own snapshot; not shared with SD-18/SD-19/SD-21.)
2. Append a new entry to the cycle log under `## SD-20 cycles`:

```
### cycle-<cycle-id> | <criterion> | <commit sha> | <card id> | <evidence transition> | cargo test <N>/<N> green | clippy clean | <timing>
```

3. If the cycle did not produce a landed commit (test could not be made green, corpus record missing, in-flight process blocked the criterion, etc.), add an `## Open blockers` entry under SD-20's section with the specific reason so the next cycle routes around it.

Do NOT rewrite the doc from scratch. Edit in place so the diff is small and auditable.

### Step 12 — Exit the cycle

Print a final 7-line report and exit:

```
cycle: <cycle-id>
criterion touched: <criterion>
row_or_kind: <row_or_kind>
commit: <commit sha on tranche/4, or 'no commit: <reason>'>
card: <hermes kanban card id, or 'no card: <reason>'>
verify: cargo test <X>/<X> green; clippy clean
status: GREEN | FAIL | NO-OP | CLAIM-EXISTS
```

`/loop` restarts the cycle 60 minutes later. The next cycle re-reads the progress doc and picks the next criterion.

## Self-healing posture

The loop self-heals wherever the failure is mechanically resolvable. The operator returns from a multi-day run to a list of problems — not a stopped loop.

### Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start | `git status --porcelain \| wc -l` returns non-zero | Run `git stash` (if previous unfinished attempt) or `git checkout -- .` (stray edit noise); re-verify clean; retry |
| A wire-fixture parity `expected_output` diverges from the engine's actual output during RED testing | Cycle's RED test fails on assertion `pilot_receipt == fixture.expected_output` | Recompute the engine's output against the boundary contract; if the engine is right, update the fixture (with an audit comment on the cycle's card); if the contract is wrong, fix the contract first |
| A spellbook school cycle discovers a corpus record whose `TableCellRef` lookup returns `None` (the table store doesn't have an entry for that KEY) | Cycle's `TableCellRef` assertion fails | Extend SD-19's `rules_tables/crb/spell_list.rs` to add the missing KEY-to-row mapping; mark the cycle's PR with audit comment per SD-19 closure pattern |
| A feat cycle discovers a prerequisite path that the engine models partially but doesn't fully ground | Cycle's `PrerequisiteEvaluation` differs from fixture's expected output by exactly one prereq path | Read the upstream epic's output to confirm whether the path is grounded elsewhere; if not, escalate to the operator (this is the boundary contract's territory) |
| Progress doc snapshot drift between SD-20 cycle work and `origin/tranche/4` | Progress doc > 5 commits behind the live-cycle verdict | Append a `## SD-20 cycles — snapshot at <sha>` block; reset snapshotting to that state |

### Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| The `cargo build` doesn't compile because a partial epic's seam signature doesn't match its parent module's expectations | `cargo build 2>&1 \| tail` shows error | This is a slice-bug, not a cycle-bug; the slice needs to be amended, not the cycle |
| Two epics produce contradictory `PilotReceipt` shapes (e.g. one writes `equipment.attack_bonus`, another reads `equipment.attack_bonus_delta`) | Compile error or wire-fixture parity test fails across all fixtures | Boundary-contract drift — the epic's seam signature has diverged from the canonical contract |
| The SD-19 foundation slice's table store has a missing entry the SD-20 epic needs | RED test fails because `TableCellRef` returns `None` on a guaranteed-present corpus record | The foundation slice is incomplete; the foundation slice itself is out of SD-20's scope (SD-19 owns the table store) |
| A spell *effect* is needed for tabletop-readiness but the engine's spellbook epic produces only spell *coverage* without *effects* | Wire-fixture parity test for a tablet-relevance spell fails because the receipt has the spell name and DC but no effect text or dice expression | The engine's spellbook output is `SpellbookCoverage` which holds spell metadata, not effects. SD-20's epic 2 may need to extend to produce effect-text-and-dice; escalate to operator |
| A feat *effect* is needed for tablet-readiness but the engine's feat prerequisite epic produces only prerequisite eligibility without *effects* | Wire-fixture parity test for a chosen-feat scenario fails | Same shape as the spell-effect gap; epic 3 needs to extend `FeatEffects` to produce the actual deltas; escalate |
| A user-picked Level Up selection goes into the next `CharacterInput` but the engine's receipt doesn't reflect it (the engine grant works fine; the integration with user selections breaks) | Wire-fixture parity test for a post-Level-Up scenario with explicit picks fails | Integration closure failure; epic 7's boundary with `CharacterInput` needs an explicit contract |
| Cargo test regresses on a row other than the one the cycle touched | Full suite regresses after a cycle's change | Sibling-preservation is a hard rule |
| Progress doc and live matrix disagree on a row's `evidence_tier` (not just stale snapshot) | `support_state_matrix.rs` says `Supported/Product-visible` but the progress doc's row status is `open` (or vice versa) | Manual operator reconciliation required |
| Two live `claude` processes would both touch `pilot_compute.rs` or any per-epic module file | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule (per SD-18's / SD-19's choke-point file partition) |

## Hard stops (refuse, exit FAIL)

The cycle refuses to advance when any of the following is true. In every case the cycle writes the reason to `## Open blockers` in the progress doc and exits with `FAIL`.

- A slice branch has diverged from `tranche/4` in a way that needs a manual rebase.
- The progress doc and the live matrix disagree on a row's `evidence_tier` and the disagreement is not just a stale snapshot.
- `cargo test --tests` regresses on a row other than the one the cycle touched. Sibling-preservation is a hard rule.
- Two live `claude` processes are working on cycles that would both touch `src/rules_core/contract.rs`, `src/rules_core/spellbook.rs`, or any other per-epic parent module file.
- **SD-20-specific:** A cycle's RED test depends on a corpus record or table-store fixture that does not exist in the SD-19 table store yet (SD-19 owns the table store; SD-20 cannot extend it autonomously).

## What "tabletop-readiness closure" actually means for SD-20

SD-20 closes when all of these are true:

1. **Epic 1 closed**: `CharacterInput` / `PilotReceipt` types land; boundary contract artifact lands; at least 8 wire-fixture parity JSON fixtures exist (one per epic + boundary contract itself).
2. **Epics 2–7 closed**: every PF1 spell school (epic 2, 9 schools), every feat category in CRB's feat catalog (epic 3), every skill-class handling category (epic 4), every CRB equipment category (epic 5, 4 categories), every damage-class criterion (epic 6), every core class's Level Up plan (epic 7, 11 classes) ship with their RED tests green.
3. **Epic 8 closed**: the canonical tabletop scenario fixture (`tests/fixtures/wire/sd20/human_fighter_level_1_tabletop.json`) lands; the integration test (`tests/sd20_tabletop_readiness_integration.rs`) passes; the engine produces a `PilotReceipt` whose every displayed sheet cell matches the table cells referenced by `TableCellRef`s.

Tabletop-readiness is **not** locked to GUI implementation. The GUI stays outside the bundle per `decisions.md` §6; the operator vibe-codes the GUI against the parity-test fixtures. Tabletop-readiness is the engine-completeness criterion, the GUI is a separate operator-owned surface that consumes the engine's output.

## How the loop will end

The `/loop` form exits when the operator stops it. There is no automatic stopping condition. The loop keeps picking the next-best criterion until every criterion is `done` (closure met) or every criterion has a real blocker in `## Open blockers`.

The operator can stop the loop at any time; a stopped loop leaves the progress doc in the state of the last completed cycle, with all open claims expired, and the operator can resume by relaunching `/loop 60m /batch /goal <this file>`.

## Operating posture (for the operator launching the loop)

1. **One launch command, run to closure.** Launch with `/loop 60m /batch /goal ./loop-instruction.md`. The loop runs to closure — every criterion `done` or every criterion has a real blocker in `## Open blockers` — and then exits. The operator does not need to inspect progress between cycles or between epics; the loop's own eligibility check + dependency graph + file-touch partition handle the sequencing automatically. The progress doc `./progress.md` is the durable record; the operator reads it on return (whether that's minutes or days later) and sees the final state.

2. **Why one launch, not three windows.** The dependency graph (`epic-breakdown.md`) is the sequencing mechanism:
   - **Epic 1 (boundary contract)** is the only eligible criterion at launch — no other epic can produce work until the contract exists. The loop's Step 1 eligibility check naturally serializes this.
   - **Epics 2/3/4/5 (spellbook / feat prereqs / skill ranks / equipment effects)** all depend only on SD-19's table store + the boundary contract from epic 1. They have **disjoint parent modules** (`spellbook.rs`, `feat_prereqs.rs`, `skill_allocation.rs`, `equipment_effects.rs`) so the file-touch partition permits concurrent cycles on disjoint modules. The loop's Step 1 priority order (epic 2/3/4/5 in any order) + the file-touch partition together enable parallel progression without operator intervention.
   - **Epic 6 (damage total)** is sequential after epic 5 — the damage engine reads from equipment-effects' outputs. Epic 6's eligibility check requires epic 5 to be `done`; the loop naturally waits.
   - **Epic 7 (Level Up grants)** integrates after epics 2–6 close. Same eligibility gating.
   - **Epic 8 (tabletop-readiness integration closure)** is the integration milestone. Gated on every other epic.
   
   The loop's Step 1 picks the smallest unclaimed eligible criterion from the progress doc's `## SD-20 cycles` open list. Eligibility includes the dependency-graph gate (epic N's cycles only fire after epic N's prerequisites are `done`). The operator does NOT manually switch launch forms between epic 1 / epics 2-5 / epics 6-8 — the loop's own logic handles each transition.

3. **What `/batch` actually does in Hermes.** Per the SD-13 loop-model excerpt (`programs/codex/requirements/SD-18-core-rules-breadth/references/sd13-loop-model-excerpt.md`), `/batch` is the form that lets a single shell invocation run multiple streams concurrently against the shared goal file, with the supervisor managing the 60-minute restart cadence across all streams. The four lanes for epics 2/3/4/5 run as four streams inside one `/loop /batch` invocation, not as four separate shells.

4. **Default ceiling: 1 cycle at a time per file.** The file-touch partition collapses any parallel attempt for the shared `contract.rs` (epic 1), shared parent modules (epics 2/3/4/5 each have their own), and integration test files (epic 8). Two cycles in parallel racing on the same file is a structural violation, not a recommendation.

5. **Watch the progress doc, not the loop output.** The cycle log is the durable truth. If the log shows three cycles in a row with no landed commit, the loop is stuck on a structural problem and the operator should investigate. If you're asleep or away from the terminal, the next time you read the progress doc you see the cumulative state — no operator-attention tax during the run.

6. **Post-mortem record is the kanban board.** Each cycle mints a card on `codex-tranche-4` (separate from the chassis-lane `codex-tranche-3` boards; **board created 2026-07-16** — operator directive 2026-07-16) with the §Step 10 schema. The loop's Step 10 kanban card mint command is **explicit**: `hermes kanban --board codex-tranche-4 create ...` (the `--board` flag is hard-coded, so it works regardless of the operator's `hermes kanban boards current` setting). A 3-day-later operator reads the board to reconstruct what happened.

7. **The 5-hour window applies here too.** A 60-minute cycle × 5 hours = up to 5 landed criteria per 5-hour window per stream. Realistic target: 3-5 criteria per window with 1 cycle each; during the concurrent window (epics 2/3/4/5), the supervisor runs 4 streams in parallel and realizes 12-20 criteria per 5-hour window if all four lanes are green.

8. **SD-21 runs in parallel.** SD-21's loop is on the same `tranche/4` branch (subject to SD-21's launch-branch decision) and `codex-tranche-4` board (or its own board, depending on operator call). Tranche-4 closes when both SD-20's tabletop-readiness gate and SD-21's campaign-manager closure gate land.

9. **Force-push discipline on `tranche/4` is conservative.** A mid-cycle correction requires a `git reset --soft HEAD~1` + force-push. This is acceptable only when the previous commit was seconds old and no downstream observer has fetched. If the commit has been on `tranche/4` for any non-trivial time, escalate to operator before force-pushing.

10. **The `tranche/4` branch must exist on origin before the loop's Step 3 fetch succeeds.** Operator creates the branch once (`git push origin tranche/4` from the operator's side, after `tranche/3` is merged per operator directive 2026-07-16) and the loop's `git fetch origin tranche/4` then resolves cleanly. Until then, Step 3 falls through to the local-only checkout path with a benign "couldn't find remote ref" warning; cycles continue normally on the local branch.

11. **Pre-launch setup checklist (operator action, before first launch; status as of 2026-07-17 end-of-session: loop already in flight, 24 SD-20 cycles recorded, branch already pushed).**
    - [x] `codex-tranche-4` kanban board created (board slug: `codex-tranche-4`; board display name: "Codex Tranche 4 (SD-20 per-character rules engine)"; board DB: `/home/ubuntu/.hermes/kanban/boards/codex-tranche-4/kanban.db`; created 2026-07-16 via `hermes kanban boards create codex-tranche-4 ...`).
    - [x] `tranche/4` branch pushed to origin (operator ran `git push origin tranche/4` after `tranche/3` merged; `origin/tranche/4` resolves; HEAD short SHA `208f326` end-of-session).
    - [ ] Operator's interactive `hermes kanban boards current` is set to `codex-tranche-4` for operator-driven inspection (note: the loop's Step 10 mint command has `--board codex-tranche-4` explicit, so it works regardless of the default-board setting; this step is only for operator inspection convenience). **Status 2026-07-17:** operator has not yet confirmed this was done; if left un-set the loop still mints cards correctly via the explicit `--board` flag.
    - [x] `./progress.md` exists — loop created it on first run with frontmatter (`title`, `mirrors`, `created`, `snapshot_as_of`); 24 SD-20 cycles recorded through `208f326`; loop's durable record per operating-posture rule "watch the progress doc, not the loop output."
- [2026-07-17]: All pre-launch items above were pre-launch-unchecked before the loop started; the loop's been running. The current `[x]` status reflects the as-of-now disk-truth (board created, branch pushed, progress doc in flight). The lone remaining `[ ]` is the operator's interactive `hermes kanban boards current` setting, which is operator-driven and not load-bearing for the loop's Step 10 mint path.

12. **How the operator knows SD-20 is done.** The loop runs to closure per `## How the loop will end`: when the progress doc's `## SD-20 cycles` open list is exhausted (every criterion `done` or every criterion has a real blocker), the loop's last cycle prints a final 7-line report and exits. Operator wakes up, reads the progress doc, and sees the final state. No operator-attention tax during the run; no manual switchover between launch forms; the supervisor manages the streams.

## Cross-reference

- `./scope-draft.md` — canonical handoff (any class any level 1-20, PrintSheetData, tranche/4 branch, operator-approved 2026-07-16).
- `./decisions.md` — 15-item decision record (SD-20 §1–§15: 9 original decisions plus §10 boundary-contract strictness, §11 multiclass Level Up class-selection trigger, §12 print-ready data, §13 campaign-shape ownership, §14 architectural-question closure summary, §15 tranche/4 branch decision).
- `./acceptance-and-verification.md` — 11 closure gates (gate 10 broadened to any class any level 1-20; gate 11 promotion `tranche/4 → develop`).
- `./epic-breakdown.md` — 15 acceptance criteria mapped to 8 epics.
- `./risks-and-open-questions.md` — self-healable vs. non-self-healable split, 4 override flags (A–D, all defaulted), 4 architectural questions (Q1–Q4, all PINNED 2026-07-16: Q1 soft enforcement, Q2 class-selection trigger (Level Up = same mechanic as level-0-to-1), Q3 print-ready data (UI plugs values into cell locations), Q4 SD-21 owns `CampaignSnapshot`).
- `./technical-design.md` — boundary contract shape, wire-fixture parity test format, 7 per-epic seam signatures, per-epic authority surface.
- `./technical-requirements.md` — pre-loop prerequisites.
- `../SD-19/decisions.md` §3 (Canonical Paizo-table store, the authority surface SD-20 reads; §2.5 now at full corpus coverage per the 2026-07-16 coverage-gap closure).
- `../SD-21/` (parallel sibling bundle; SD-21's launch branch is a separate operator decision).
