# SD-18 Core Rules Breadth — Operator-Driven Loop Instruction

This file is the body of the goal the `/loop 60m /batch /goal` invocation runs.
It is **self-sufficient**: no interactive prompts, no mid-loop questions to
the operator, no shared state with anything other than the on-disk files
named here. The loop runs it; the loop restarts every 60 minutes; the loop
dies when the operator stops it.

## What this loop does

Advance SD-18 — full Pathfinder 1e Core Rulebook at levels 1-20 across the
7 core races, 11 core classes, 2 SD-13 interaction rows, 9 PF1 strict
schools, and 4 core-rulebook equipment categories — toward
`SupportState::Supported` AND evidence tier `Product-visible` for every
acceptance criterion. Working in bounded cycles against the integration
branch `tranche/3`. Each cycle lands one acceptance criterion.

The loop inherits from the **matured** SD-13 operator-loop model. The
posture used here differs from the as-written
`/home/ubuntu/workspace/sd13-class-uplift-loop-prompt.md` in three places:

1. **Auto-merge to tranche branch** (no operator review per slice).
2. **Ephemeral feature branches** (deleted after merge).
3. **Kanban card post-mortem** (cards mint at cycle close, never pre-mint).

See `/home/ubuntu/workspace/programs/codex/requirements/SD-18-core-rules-breadth/decisions.md`
§2-5 for the rationale. See
`/home/ubuntu/workspace/programs/codex/requirements/SD-18-core-rules-breadth/references/sd13-loop-model-excerpt.md`
for the SD-13 patterns preserved unchanged.

## Required reading (every cycle)

### 1. Canonical handoff doc

```
cat /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
```

This is the canonical scope doc. The 34 acceptance criteria live there by
section number (§3.1 race rows, §3.2 class rows, §3.3 interaction rows, §3.4
spell schools, §3.5 equipment categories). Each criterion's acceptance
criterion prose and concrete corpus/code pointers live there.

### 2. Progress doc (loop's working memory)

```
cat /home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md
```

Created on first run if missing. Loop's claim protocol lives here. Each
section in the progress doc corresponds 1:1 with §3.x in the scope doc.
Under each section, the loop maintains `done` / `in-flight` / `open`
status rows with cycle-id, branch, merge SHA, and card id.

### 3. Live git state

```
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
git log origin/tranche/3 --oneline -5
git ls-remote origin | grep -E 'loop/tranche3-cycle-' | head
git branch -a | grep -E 'loop/tranche3|tranche'
git worktree list --porcelain
```

### 4. In-flight detection

```
ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep
```

If any `claude` process is running with a prompt that names a specific
acceptance criterion, do NOT pick that criterion. Cycle exits with
`CLAIM-EXISTS` status; loop restarts.

## Concurrency rules (read first, obey always)

These rules are structural. Two concurrent cycles that touch the same
file are guaranteed to collide; the loser will be Tech-Priest (or the
operator) having to reconcile.

### File-touch partition (the hard rule)

The SD-18 cycle surface is concentrated in these files:

| File | Purpose | Cycles that may touch it |
|---|---|---|
| `src/rules_core/pilot_compute.rs` | The compute seam — every class/race/spell cycle extends seam functions here | **One cycle at a time, full stop** |
| `src/rules_core/support_state_matrix.rs` | The matrix carrier — every cycle updates a row's evidence_tier | **One cycle at a time** |
| `src/pcgen_import/ir_converter.rs` (if slice C-extension surfaces) | The LST-to-canonical converter projection | One cycle per conversion layer |
| `tests/sd18_<criterion>.rs` | Per-cycle test file | One cycle per file (its owning criterion) |
| `tests/fixtures/rules_core/pf1_<race-or-class>_<level>_sd18_*.txt` | Per-cycle fixture | One cycle per fixture |

The first two rows are the choke point. The chassis-matrix pair gets
touched by every cycle. **At most one cycle may be active across these
files.**

### Per-cycle spawn budget (the default)

Default: **1 cycle at a time.** Reason: the file-touch partition collapses
any parallel attempt into a serial one for the first two files. Two
cycles in parallel means two cycles racing on the matrix carrier, two
cycles serializing on `pilot_compute.rs` rebase, and zero speedup.

To run more than one cycle in parallel you must show that the second
cycle touches a disjoint file set. That is only possible when the second
cycle is doing **documentation-only work** (e.g. updating the progress
doc, writing a future-cycle handoff doc, refreshing the matrix markdown).
For code-bearing cycles, **1 cycle at a time is the rule**.

This is not a recommendation; it is a structural property of the cycle
surface.

### Branch control protocol (extend SD-13's claim TTL pattern)

SD-18's cycles auto-merge. Claim TTL is replaced by branch-name tracking:
when a cycle creates `loop/tranche3-cycle-<cycle-id>-<criterion>` and
pushes it to origin, the branch's existence IS the claim. Multiple
cycles that would create the same branch name detect each other via the
`git ls-remote origin | grep -E 'loop/tranche3-cycle-'` check.

If the desired branch name is already on origin (i.e. another cycle owns
it), choose a different criterion or exit with `CLAIM-EXISTS`.

## Per-cycle procedure (the steps, in order)

### Step 1 — Pick a criterion

From the progress doc's `open` list, pick the smallest unclaimed acceptance
criterion. Priority order:

1. **§3.3 interaction rows first** (per operator directive 2026-07-12).
2. **§3.1 race rows** in alphabetical order (Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Halfling, Human).
3. **§3.2 class rows** in alphabetical order (Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Wizard).
4. **§3.4 spell schools** in canonical PF1 order (Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, Universal).
5. **§3.5 equipment categories** in corpus-natural order (`arms_armor`, `general`, `magic_items`, `equipmods`).

**Eligibility check.** A criterion is eligible when:

1. The criterion has not yet reached `supported/Product-visible` (per the
   progress doc's `done`/`open` status).
2. No live `claude` process is working on that criterion (in-flight
   detection above).
3. The chosen burden or family is **actually computable** from the
   existing `pilot_compute.rs` shape — i.e. it is an arithmetic
   extension or a single-seam recognition record, NOT a new subsystem
   (feat-prerequisite engine, general skill-rank engine, damage total
   engine, full spellbook engine). New subsystems are tranche-level
   decisions, not cycle decisions.

When several criteria tie on priority above, prefer the one that has
not had a cycle attempted in the last 3 cycles (read the progress doc's
`## Cycle log` block to check). The loop's job is to advance the
**frontier**, not to retry the same criterion forever.

### Step 2 — Pick the criterion's work-unit

Each acceptance criterion has more granular work-units within it. The
loop must NOT try to land a whole criterion in one cycle. Instead:

- For §3.1 race rows: one named family per cycle (e.g., Dwarf's
  Stonecunning; Elf's Elven Immunities). The progress doc tracks
  per-family state under the race row.
- For §3.2 class rows: one level band or one named burden per cycle
  (e.g., Wizard L2-5 progression; Paladin's level-3 Divine Bond).
- For §3.4 spell schools: one school per cycle, landing all spells in
  the school in one round.
- For §3.5 equipment categories: one category per cycle, landing a
  representative sample of items per round.

### Step 3 — Create the feature branch

```bash
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
git checkout -b loop/tranche3-cycle-<cycle-id>-<criterion> origin/tranche/3
```

Cycle-id is the ISO-8601 timestamp the cycle started. Criterion is the
one chosen in Step 1 with work-unit from Step 2. Example:
`loop/tranche3-cycle-2026-07-13T0900-dwarf-stonecunning`.

### Step 4 — Write the failing test first

Add `tests/sd18_<criterion>.rs`. Mirror the shape of the most recent
sibling cycle's test file. The test must fail for the intended reason
when run against `origin/tranche/3` as the base.

```bash
cargo test --locked --test sd18_<criterion> 2>&1 | tail -40
```

Capture the failing output. It is the RED evidence.

### Step 5 — Implement the smallest change that makes the test pass

For most sd18 cycles, the change is one of:

- **Extend existing race seam function** with one new named family.
  Example: `explain_dwarf_race_seam` at `pilot_compute.rs:2529` gets
  a Stonecunning branch.
- **Extend existing class seam function** with one level or burden.
  Example: `explain_wizard_level1_prepared_spell_baseline` at
  `pilot_compute.rs:9247` gets a level-2 readiness branch.
- **Extend existing school coverage** to consume corpus-side spell
  records.
- **Extend existing equipment category coverage** to consume
  corpus-side items.

For all paths, the change must be in `pilot_compute.rs` for race/class
work, in the SD17-B-4/B-5 modules for spell/equipment corpus work, and
the matrix carrier for state transitions. The forbidden write scopes
are documented in the SD-18 decisions file §6.

Run:

```bash
cargo test --locked --test sd18_<criterion> 2>&1 | tail -40
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
```

All three must be green. Capture the output. It is the GREEN evidence.

### Step 6 — Commit, push

```bash
git add src/rules_core/pilot_compute.rs \
        src/rules_core/support_state_matrix.rs \
        tests/sd18_<criterion>.rs \
        tests/fixtures/rules_core/pf1_<race-or-class>_level<N>_sd18_*.txt
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd18): <criterion> (<row transition>)"
git push -u origin loop/tranche3-cycle-<cycle-id>-<criterion>
```

The branch is pushed to origin but NOT yet merged.

### Step 7 — Open the PR (optional)

`/loop` cycles do not need to open PRs. Auto-merge to `tranche/3` is the
merge path; PRs to `develop` are operator-driven only (per `decisions.md`
§3). If `gh` is available and the operator wants visibility, the cycle
may open a PR against `tranche/3` for traceability:

```bash
GH_TOKEN=$(cat ~/.config/gh/.claude_gh_token) \
  gh pr create --repo electricm0nk/codex \
    --base tranche/3 \
    --head loop/tranche3-cycle-<cycle-id>-<criterion> \
    --title "SD18: <criterion>" \
    --body "<PR body with criterion, test file, evidence tier transition>"
```

This is informational, not gating. Skip if PR creation is noisy.

### Step 8 — Auto-merge to tranche/3

```bash
cd /home/ubuntu/workspace/repos/codex
git checkout tranche/3
git pull origin tranche/3
git merge --no-ff loop/tranche3-cycle-<cycle-id>-<criterion> -m "merge: sd18 <criterion>"
git push origin tranche/3
```

Capture the merge commit SHA. It is the durable merge receipt.

**On merge conflict**: see §Self-healing below.

### Step 9 — Cleanup (delete ephemeral branch)

```bash
git checkout tranche/3
git branch -d loop/tranche3-cycle-<cycle-id>-<criterion>
git push origin --delete loop/tranche3-cycle-<cycle-id>-<criterion>
git worktree list --porcelain | grep -F "$WT" && git worktree remove --force "$WT" || true
rm -rf /home/ubuntu/workspace/repos/codex/.claude/worktrees/loop-<cycle-id>
```

The branch's only purpose was carrying the cycle to `tranche/3`. After
merge, the merge commit on `tranche/3` is the durable record; the feature
branch is ephemeral.

### Step 10 — Mint the kanban card (post-mortem record)

```bash
hermes kanban --board codex-tranche-3 create \
  "SD18 <criterion> (<criterion-section>) [cycle <cycle-id>]" \
  --assignee operator \
  --workspace scratch \
  --initial-status done \
  --created-by operator \
  --priority 3 \
  --body "<card body per schema below>"
```

Card body schema:

```
epic: SD-18
criterion_section: <scope doc section reference, e.g. "§3.1 Race rows: Dwarf">
row_or_kind: race:dwarf | class:wizard | school:abjuration | category:arms_armor | interaction:human-bonus-feat-seam
evidence_tier_before: <previous SD-13 row state>
evidence_tier_after: <new SD-13 row state after this merge>
feature_branch: loop/tranche3-cycle-<cycle-id>-<criterion>
merge_receipt_sha: <merge commit SHA on tranche/3>
cycle_id: <ISO-8601 timestamp>
cargo_test_summary: <test summary string>
clippy_signal: clean | dirty
cycle_timing_seconds: <N>
self_heals_applied: <list, empty if none>
next_required_uplift: <recommendation for next iteration>
ui_surface: <operator-provided surface name, empty if none>
```

The card mint is the post-mortem record. It exists so a 3-day-later
operator can find any specific cycle by searching the board.

### Step 11 — Update the progress doc

Edit `/home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md` in place:

1. Update the "Snapshot as of" line to the current `tranche/3` HEAD short SHA.
2. For the criterion worked this cycle, move its row from `open` to
   `done` with cycle-id, branch, merge SHA, card id.
3. Append one row to the `## Cycle log` block:

```
### cycle-2026-07-13T09:00 | <criterion> | <branch> | <merge sha> | <card id> | <evidence transition> | cargo test <N>/<N> green | clippy clean | <timing>
```

4. If the cycle did not produce a landed commit (test could not be made
   green, branch had a conflict, in-flight process blocked the
   criterion, etc.), add an `## Open blockers` entry with the specific
   reason so the next cycle routes around it.

Do NOT rewrite the doc from scratch. Edit in place so the diff is small
and auditable.

### Step 12 — Exit the cycle

Print a final 7-line report and exit:

```
cycle: <cycle-id>
criterion touched: <criterion>
row_or_kind: <row_or_kind>
feature_branch: <branch name, deleted after merge>
merge: <merge sha on tranche/3, or 'no merge: <reason>'>
card: <hermes kanban card id, or 'no card: <reason>'>
verify: cargo test <X>/<X> green; clippy clean
status: GREEN | FAIL | NO-OP | CLAIM-EXISTS
```

`/loop` restarts the cycle 60 minutes later. The next cycle re-reads the
progress doc and picks the next criterion.

## Self-healing posture

The loop self-heals wherever the failure is mechanically resolvable. The
operator returns from a multi-day run to a list of problems — not a
stopped loop.

### Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Feature branch diverged from `tranche/3` mid-cycle | `git fetch origin tranche/3` reveals new commits | `git rebase origin/tranche/3` in worktree, re-run tests, re-push, re-merge |
| Merge conflict in auto-merge (mechanical) | `git merge` reports conflicts | Resolve inline if mechanical (import ordering, unrelated additions). Re-commit the merge. |
| Cargo build cache corruption | `cargo build` reports stale state | `cargo clean`, rebuild, re-run tests |
| Disk pressure from `target/` | Worktree disk usage high | `rm -rf $WT/target` |
| Stale worktree from prior cycle | `git worktree list` shows orphaned paths | `git worktree remove --force`, retry |
| Progress doc snapshot drift | Progress doc > 5 commits behind `tranche/3` | Read live matrix, refresh progress doc snapshot, retry |
| Accidental commit to wrong branch | `git log <branch>` shows the cycle's commit | `git reset --hard origin/tranche/3`, retry on correct branch |

### Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| Conflict requires a domain decision (which side wins on a class-feature semantics question) | Merge conflict has overlap on a question with no mechanical resolution | Operator must decide which semantics are canonical |
| Slice branch needs manual rebase | `git rebase` reports conflicts the auto-resolver cannot handle | Manual operator action required |
| Two live `claude` processes would both touch `pilot_compute.rs` | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule |
| Chosen burden needs a new subsystem (feat-prerequisite engine, spellbook engine, damage-total engine) | Cycle scope requires extending `pilot_compute.rs` beyond existing seam functions | Tranche-level decision |
| Disk at 100% with no `target/`-strip remedy | `df` reports full disk | Disk pressure outside the loop's control |
| Same criterion has failed twice already in this run | Progress doc `## Cycle log` shows two cycles for the criterion with FAIL status | Operator pause; consider scoping or seed down the criterion |

## Hard stops (refuse, exit FAIL)

The cycle refuses to advance when any of the following is true. In every
case the cycle writes the reason to `## Open blockers` in the progress
doc and exits with `FAIL`.

- The chosen burden needs a new subsystem (feat-prerequisite engine,
  spellbook engine, damage total engine).
- A slice branch has diverged from `tranche/3` in a way that needs a
  manual rebase.
- The progress doc and the live matrix disagree on a row's
  `evidence_tier` and the disagreement is not just a stale snapshot.
- `cargo test --tests` regresses on a row other than the one the cycle
  touched. Sibling-preservation is a hard rule.
- Two live `claude` processes are working on cycles that would both
  touch `pilot_compute.rs`.

## What "supported / Product-visible" actually means

A row or criterion reaches `supported/Product-visible` only when **both**
of these are true:

1. The row's evidence_tier is `Product-visible` — the operator's UI
   surfaces it (operator-driven, not loop-driven).
2. **Every** named burden / class-feature / school-spell / equipment
   sample listed in the row's `blocker_or_lossiness_note` is grounded
   as a real computed contribution (not a recognition record, not a
   diagnostic string).

Until both are true, the row is `Partial/Computed` or `Blocked/Computed`,
or higher tiers. The honest promotion per cycle is usually
`Blocked→Partial` or `Partial→Partial` widening. Do NOT promise a
`→Supported` jump; do NOT ship a card whose body claims a promotion
the diff does not support.

The remaining structural gaps that **no cycle can close** are the
new-subsystem work named in §Hard stops above. When every other row is
`Partial` or `Supported`, those gaps become named blockers for a future
tranche, not excuses for fake completion.

## How the loop will end

The `/loop` form exits when the operator stops it. There is no automatic
stopping condition. The loop keeps picking the next-best criterion until
every criterion is `Supported/Product-visible` or every criterion has
a real blocker in `## Open blockers`.

The operator can stop the loop at any time; a stopped loop leaves the
progress doc in the state of the last completed cycle, with all open
claims expired, and the operator can resume by relaunching `/loop 60m
/batch /goal <this file>`.

## Operating posture (for the operator launching the loop)

1. **Launch with `/loop 60m /batch /goal <this file>`.** The `/batch`
   form lets the loop restart every 60 minutes without operator
   intervention. The 60-minute cadence is one cycle long enough to land
   a small criterion, short enough that a stuck cycle doesn't waste a
   long block.

2. **Default ceiling: 1 cycle at a time.** The file-touch partition
   collapses any parallel attempt.

3. **Watch the progress doc, not the loop output.** The cycle log is
   the durable truth. If the log shows three cycles in a row with no
   landed commit, the loop is stuck on a structural problem and the
   operator should investigate before letting it run another cycle.

4. **Tranche-branch auto-merge means PRs to develop are operator's
   call.** Per `decisions.md` §3, the loop auto-merges to `tranche/3`.
   The operator opens the `tranche/3 → develop` promotion PR when
   review-ready.

5. **Post-mortem record is the kanban board.** Each cycle mints a card
   on `codex-tranche-3` with the §Step 10 schema. A 3-day-later operator
   reads the board to reconstruct what happened.

6. **The 5-hour window applies here too.** A 60-minute cycle × 5 hours
   = up to 5 landed criteria per 5-hour window, with each criterion on
   a distinct SD-18 acceptance criterion. Realistic target: 3-5
   criteria per window with 1 cycle each. The progress doc accumulates
   the progress; the operator reviews the merge history and the kanban
   board on the same cadence.

## Cross-reference

- `/home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md` (canonical handoff doc; 34 acceptance criteria with corpus/code pointers)
- `/home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md` (loop working memory; created on first run)
- `/home/ubuntu/workspace/programs/codex/requirements/SD-18-core-rules-breadth/` (spec-domain STC bundle; doctrine + decisions + technical-design + risks + acceptance)
- `/home/ubuntu/workspace/programs/codex/requirements/SD-18-core-rules-breadth/references/sd13-loop-model-excerpt.md` (matured SD-13 patterns preserved unchanged)
- `/home/workspace/sd13-class-uplift-loop-prompt.md` (source pattern for this file's structure)
