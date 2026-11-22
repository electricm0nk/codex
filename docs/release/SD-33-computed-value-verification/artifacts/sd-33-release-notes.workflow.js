export const meta = {
  name: 'sd-33-release-notes',
  description: 'SD-33 AT-33-E6-003 part 2 — release notes, version confirm, kanban row 21 to complete',
  whenToUse: 'After AT-33-E6-003 part 1 opened PR #377 but the release-notes cycle did not land.',
  phases: [{ title: 'Release notes and closure' }],
}

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-33-computed-value-verification'
const BRANCH = 'tranche/13'

function prompt() {
  return `You are the SD-33 release-notes cycle. Role: sd33-release-notes.

## SITUATION
SD-33 is closed except for one cycle: **AT-33-E6-003 part 2**. Verified by the
orchestrator against the repo, not from a report:

- \`AT-33-E6-001\` gate PASS on attempt 10 (row 19 \`complete\`)
- \`AT-33-E6-002\` retrospective written and cited (row 20 \`complete\`)
- Part 1 landed: 5 architecture docs refreshed, graphify run, **PR #377 OPEN**
  (\`https://github.com/electricm0nk/codex/pull/377\`)
- **Kanban row 21 is \`not-started\`** and \`${PKG}/release-notes.md\` still reads
  \`status: not generated — bundle not launched\`. A prior workflow reported the
  bundle closed; that step did not actually happen.

You are the last cycle of this bundle.

## READ FIRST, in order
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md §5, §6, §7, §11 step 5
4. ${REPO}/${PKG}/release-notes.md — its own "Generation rule" section is binding
5. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt10_cycle_receipt.md
   — the closure scan's verified figures. **Re-derive the headline ones yourself**;
   a number copied from a mid-bundle receipt has silently drifted, which is the
   exact hazard SD-32's release notes hit.

## ENVIRONMENT
  export RETRO_ACTOR="sd33-release-notes"
  export CARGO_TARGET_DIR="/tmp/cargo-sd33-release-notes"
  export CARGO_INCREMENTAL=0
  mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"

## !! CRITICAL — THE SHARED CHECKOUT IS POLLUTED. DO NOT COMMIT IT. !!
\`${REPO}\` currently carries **~139 STAGED files** that are an unexplained,
uncommitted **full revert of wave 6's \`.MOD\`-fold fix**: 137 \`data/corpus/**\`
files, \`src/bin/enrich_equipment_raw_tokens.rs\`, plus deleted receipts,
oracle-results, and retro jsonl files. It was left by an idle teammate. Four
consecutive waves have now hit it.

**Committing it would revert a landed, verified fix and break the closed bundle.**

Therefore:
- **Work in a clean detached worktree**, exactly as the previous lane did:
  \`git worktree add --detach <path> origin/${BRANCH}\`, then work there.
- **Never \`git add -A\`. Never \`git add .\`.** Stage only the specific files you
  edit, by explicit path.
- **Do not attempt to discard, reset, checkout, or restore that pollution.** It is
  not yours to clear and prior attempts were blocked by the harness classifier.
  Leave it exactly as you found it and say so in your report.
- \`git status --porcelain\` before EVERY git write, and inspect it.
- Never \`git stash\` in this repo at all — the bare form stashes the whole shared
  checkout even from a subdirectory.
- Never force-push.

Push with:
  git fetch origin ${BRANCH} && git rebase origin/${BRANCH} && git push origin HEAD:${BRANCH}
Retry up to 5 times on non-fast-forward.

## ONE TURN ONLY
Nothing wakes you. Wait for slow work inside the turn. Commit and push before
ending the turn, always — even a partial result.

## YOUR WORK

### 1. Write \`${PKG}/release-notes.md\` for build \`0.13.0\`
Replace the placeholder body. Keep the frontmatter but set
\`status: generated\`. Content requirements:

**Lead with the defects found and fixed** — that is what this bundle produced.
Twelve computed-value and instrument defects, each with its commit. From the
retrospective and the attempt-10 receipt, they include: the spell-DC fixture
defect (a \`.pcg\` template pinned \`STAT:WIS|SCORE:10\` instead of 18 — correct by
accident for INT/CHA casters, wrong for every WIS caster, 103 units); the armor
compute defect (22 units, \`abc72f75ec\`); the AC measurement method that conflated
a MAXDEX cap (\`a68fbeea3d\`); \`compute_equipmods_effect\` multi-chain summing
(\`2f1d52f22d\`); the \`equipment_id_resolve\` OUTPUTNAME-divergent identity fix; the
campaign-KEY-vs-display-name harness defect; the corpus-extraction gap dropping
\`.MOD\`-attached EQMOD references (\`7d439876b7\` + \`fbc945f198\`); two new resolvers
(\`EQMWEAPON|DAMAGESIZE\`, \`EQM|WEIGHTDIV\`); the unmapped doneness pair from Epic
4's reclassification; the struct rename hiding 543 of 543 integration targets; and
\`corpus_literal_sweep\`'s two \`.COPY=\`/DESC defects (\`1bfb80d7b7\`).
**Verify each against \`git log\` before listing it.**

**Then the figures**, every one stating its denominator in the same construct and
carrying its re-derive command:
- 8,330 of 8,330 units examined; 0 of 8,330 disagree; 811 agree; 7,519
  \`unverifiable\` with 0 reasonless
- work-inventory \`unknown\`: 0 of 49,438
- formula-interpreter corpus-wide: 11,652 of 11,652
- lib 2,837 of 2,837; desktop 548 of 548; 543 of 543 integration targets build
- corpus sweep: 0 findings of 48,634 records examined
- denominator gate: 0 violations of 60 files

**State the inherited debt plainly**: 31 of 599 workspace suites carrying 49 of
8,026 test failures, proven pre-existing at the \`tranche/13\` cut (0 of 31 carry a
commit since it), registered in \`forward-scope-register.md §D1\`. A reviewer
running the workspace suite must not be surprised.

**No narrative ceremony.** No bare hundred-percent tokens — the denominator gate
scans this file and has caught six agents already.

### 2. Record PR #377
In \`${PKG}/release-notes.md\` and in \`${PKG}/receipts.md\`.

### 3. Version — CONFIRM, do not bump
\`apps/desktop/package.json\` and \`apps/desktop/src-tauri/tauri.conf.json\` both
already read \`0.13.0\`, stamped at the \`tranche/13\` cut. **Confirm both and change
neither.** The tranche digit moves only on a NEW \`tranche/N\` cut, never on a
bundle's own closure.

### 4. Placeholder sweep
  grep -rn '<[a-z_-]*>' ${PKG}/*.md
Every match resolves to a real value or is a documented schema/command literal.

### 5. Close the board
Update \`${PKG}/progress.md\` to the closed state and mark \`kanban.md\` row 21
\`complete\`. Re-read both immediately before editing (§5). Kanban Notes are a
POINTER, never a story.

### 6. Gate your own prose before pushing
  scripts/verify.sh --only denominator-gate
Must exit 0. \`release-notes.md\` is in its scan scope, so your own writing IS
checked. If it trips, fix the prose — never narrow the gate.

### 7. Receipt
\`${PKG}/artifacts/epic-6-closure/AT-33-E6-003-part2_cycle_receipt.md\`, using the
§7 schema. \`- **Status:** <x>\` is a BULLET, not a \`## Status\` heading.

## RETURN VALUE — ONLY JSON, no prose:
{"criterion":"AT-33-E6-003-part2","status":"complete"|"blocked-escalated",
 "release_notes_path":"...","pr_number":377,
 "defects_listed":0,"defects_verified_against_git":0,
 "figures_rederived":[{"figure":"...","command":"...","output":"..."}],
 "versions":{"package_json":"...","tauri_conf":"...","changed":false},
 "placeholders_remaining":[...],"denominator_gate":"PASS"|"FAIL",
 "kanban_row_21":"complete"|"...",
 "pollution_untouched":true|false,"worktree_used":"...",
 "commit_shas":[...],"receipt_path":"..."}`
}

log('SD-33 AT-33-E6-003 part 2. PR #377 is open; row 21 is still not-started and release-notes.md is ungenerated. The shared checkout carries a staged revert of wave 6 — the lane works in a clean detached worktree and must not touch it.')

phase('Release notes and closure')
const notes = await agent(prompt(), { model: 'sonnet', label: 'release-notes-part2', phase: 'Release notes and closure' })

return { bundle: 'SD-33', criterion: 'AT-33-E6-003-part2', pr: 377, notes }
