export const meta = {
  name: 'sd-34-dispatch',
  description: 'SD-34 — completion atlas, proven on two books, then the 35-book forward plan',
  phases: [
    { title: 'Epic 1 — Completion Atlas' },
    { title: 'Epic 2 — Build 8 of 9 tables' },
    { title: 'Epic 3 — Core Rulebook to zero' },
    { title: 'Epic 4 — Ultimate Campaign to zero' },
    { title: 'Epic 5 — Price 35 books' },
    { title: 'Epic 6 — Closure epilogue' },
  ],
}

// Authored from docs/release/SD-34-book-completion/workflow-instruction.md §2.4.
// Invoke with args = { epics: [1] } to run one epic; omit to run all six.
// FULLY SEQUENTIAL (§3): the atlas names the tables, the tables unblock the books,
// the books measure the rates, the rates price the plan. Epics 3 and 4 run in
// parallel ONLY if §4's file-level disjointness check said DISJOINT at launch.

const PKG = 'docs/release/SD-34-book-completion'
const CUT_SHA = '571307724f'          // tranche/14 cut (0.14.0 bump)
const BASE_SHA = 'ea2b3396f2'         // origin/develop at the cut; SD-33 PR #377 merge

const EPICS = {
  1: {
    title: 'Epic 1 — Completion Atlas',
    criteria: [
      { id: 'AT-34-E1-001', title: 'every unit carries exactly one named remaining-step', dir: 'epic-1-atlas' },
      { id: 'AT-34-E1-002', title: 'the atlas fails closed on six conditions', dir: 'epic-1-atlas' },
      { id: 'AT-34-E1-003', title: 'the missing engine tables are enumerated and their book coverage mapped', dir: 'epic-1-atlas' },
      { id: 'AT-34-E1-004', title: 'the shape-engine boundary is stated as a fact, not an assumption', dir: 'epic-1-atlas' },
      { id: 'AT-34-E1-005', title: 'the `not-ingested` status field is renamed', dir: 'epic-1-atlas' },
      { id: 'AT-34-E1-006', title: 'every figure in this package carries its re-derive command, enforced', dir: 'epic-1-atlas' },
      { id: 'AT-34-E1-007', title: '`v06_corpus_trap_report --audit` is a real `verify.sh` stage', dir: 'epic-1-atlas' },
      { id: 'AT-34-E1-008', title: '`wiring-class-mismatch` is driven to zero across every affected book', dir: 'epic-1-atlas' },
    ],
  },
  2: {
    title: 'Epic 2 — Build 8 of 9 tables',
    criteria: [
      { id: 'AT-34-E2-001', title: 'each of the eight tables is built, or proven unnecessary', dir: 'epic-2-tables' },
      { id: 'AT-34-E2-002', title: 'each new table is fail-closed', dir: 'epic-2-tables' },
      { id: 'AT-34-E2-003', title: 'the measured build rate is recorded', dir: 'epic-2-tables' },
      { id: 'AT-34-E2-004', title: 'bucket A reaches zero for both vehicle books', dir: 'epic-2-tables' },
    ],
  },
  3: {
    title: 'Epic 3 — Core Rulebook to zero',
    criteria: [
      { id: 'AT-34-E3-001', title: 'bucket B closes: records reach their tables', dir: 'epic-3-core-rulebook' },
      { id: 'AT-34-E3-002', title: 'bucket C closes: held records reach the player', dir: 'epic-3-core-rulebook' },
      { id: 'AT-34-E3-003', title: 'buckets M, V, D, U, X close', dir: 'epic-3-core-rulebook' },
      { id: 'AT-34-E3-004', title: 'the cost of every step type is measured, not estimated', dir: 'epic-3-core-rulebook' },
      { id: 'AT-34-E3-005', title: 'the Core Rulebook reaches zero remaining steps', dir: 'epic-3-core-rulebook' },
      { id: 'AT-34-E3-006', title: 'anything the atlas failed to predict is recorded as an atlas defect', dir: 'epic-3-core-rulebook' },
    ],
  },
  4: {
    title: 'Epic 4 — Ultimate Campaign to zero',
    criteria: [
      { id: 'AT-34-E4-001', title: 'the 23-unit non-A tail is resolved', dir: 'epic-4-ultimate-campaign' },
      { id: 'AT-34-E4-002', title: 'Ultimate Campaign reaches zero remaining steps', dir: 'epic-4-ultimate-campaign' },
      { id: 'AT-34-E4-003', title: 'a second, independent cost measurement is recorded', dir: 'epic-4-ultimate-campaign' },
    ],
  },
  5: {
    title: 'Epic 5 — Price 35 books',
    criteria: [
      { id: 'AT-34-E5-001', title: 'a per-book, per-bucket forward plan exists for every remaining book', dir: 'epic-5-forward-plan' },
      { id: 'AT-34-E5-002', title: 'every capability that must still be built is named', dir: 'epic-5-forward-plan' },
      { id: 'AT-34-E5-003', title: 'the `power` table is costed', dir: 'epic-5-forward-plan' },
      { id: 'AT-34-E5-004', title: 'the plan is ordered by real cost, cheapest-first, and single-bucket books are flagged', dir: 'epic-5-forward-plan' },
    ],
  },
}

// Every cycle returns THIS shape. The gate check reads fields — never a substring
// (decisions.md §12 L4). `schema` forces the agent to return it.
const CYCLE_SCHEMA = {
  type: 'object',
  required: ['criterion', 'status', 'commit_sha', 'row_count_command_output', 'receipt_path'],
  properties: {
    criterion: { type: 'string' },
    status: { type: 'string', enum: ['complete', 'partial', 'blocked-escalated'],
      description: 'complete = your whole assigned population reached the bar. partial = you closed part of it and NAMED every remaining unit by sub-cause with a population that sums exactly; the dispatch continues and a later cycle picks the remainder up. blocked-escalated = you need an OPERATOR RULING (a policy or scope question you may not decide); this PAUSES the bundle, so use it only for that. Needing more cycles is never blocked-escalated - it is partial.' },
    remainder: { type: 'string', description: 'partial only: every remaining unit named by sub-cause with populations that sum to the stated total' },
    commit_sha: { type: 'string', description: 'the pushed commit for this cycle, or NONE' },
    row_count_command_output: { type: 'string', description: 'literal output of the count run on this cycle own artifact' },
    receipt_path: { type: 'string' },
    figures: { type: 'string', description: 'every number with its re-derive command and denominator' },
    build_scope: { type: 'string', description: 'no-run exit, workspace result, desktop crate result, and the SHA it ran at' },
    sweep_population: { type: 'string', description: 'corpus_literal_sweep examined before -> after and the record delta, or N/A' },
    movement: { type: 'string', description: 'four buckets: closure / reclassification / reachability / instrument-correction' },
    discoveries: { type: 'string' },
    next_cycle_plan: { type: 'string' },
  },
}

const SCAN_SCHEMA = {
  type: 'object',
  required: ['gate', 'status'],
  properties: {
    gate: { type: 'string', enum: ['PASS', 'FAIL'] },
    status: { type: 'string' },
    short: { type: 'string', description: 'what is short, with the command that shows it' },
    cards_complete: { type: 'string', description: 'N of 26 kanban rows re-derived from the repo' },
  },
}

// Only an operator-ruling request halts. A `partial` cycle named its remainder and
// the next cycle picks it up (decisions.md 15).
const halted = r => !r || r.status === 'blocked-escalated'

function cycleProcedurePrompt(c) {
  return `You are a dispatched execution lane for bundle SD-34 in the codex repo at
/home/ubuntu/workspace/repos/codex, on branch \`tranche/14\`. You start with ZERO context of this
bundle. Absolute paths only.

## Your criterion

**${c.id} — ${c.title}**

READ THESE FIRST, in this order, before touching anything:
1. \`/home/ubuntu/workspace/repos/codex/CLAUDE.md\` then \`/home/ubuntu/workspace/repos/codex/AGENTS.md\`
2. \`${PKG}/workflow-instruction.md\` — §2.1 (environment), §2.2 (execution boundary), §2.5 (never
   end a turn waiting), §5 (concurrent-write protocol), **§6 (the per-cycle procedure you execute)**,
   §7 (the receipt schema you must write), §8 (self-heal posture), §12 (the 26 standing lessons)
3. \`${PKG}/epic-breakdown.md\` — find the \`### ${c.id}\` section. Its **Evidence** paragraph is
   your acceptance bar, verbatim.
4. \`${PKG}/acceptance-and-verification.md\` §1 — the row for ${c.id} names the verifying command
   and the artifact you must produce. §5 lists what does NOT satisfy a criterion.
5. \`${PKG}/technical-design.md\` — the instruments. \`${PKG}/decisions.md\` — the bundle's ADRs;
   §2 (the ten buckets), §3 (denominators), §4 (status is a row count), §12 (L1–L8).

## Environment — set before anything else

\`\`\`bash
export RETRO_ACTOR="sd34-${c.id.toLowerCase()}"
export CARGO_TARGET_DIR="/tmp/cargo-sd34-${c.id.toLowerCase()}"
export CARGO_INCREMENTAL=0
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
\`\`\`

## Shared-checkout rules — non-negotiable

- \`git status --porcelain\` before EVERY git write.
- NEVER \`git add -A\`. NEVER \`git stash\` (the bare form stashes the whole checkout even from a
  subdirectory). NEVER force-push.
- Do NOT touch \`docs/release/SD-33-computed-value-verification/\` (untracked \`*.workflow.js\`
  litter stays untracked) or \`docs/retro/events/sd31-transcribe.jsonl\` (another lane's dirty file).
- Push via §5: \`git fetch origin tranche/14 && git rebase origin/tranche/14 && git push origin HEAD:tranche/14\`,
  retry up to 5 times on non-fast-forward.
- SD-34's shared files are exactly three: \`${PKG}/progress.md\`, \`${PKG}/kanban.md\`, and
  \`docs/work-inventory.json\`. Re-read any of them immediately before editing.

## Never end a turn waiting (§2.5)

You get **exactly one turn**. Nothing wakes you. Run slow work in the FOREGROUND with a generous
\`timeout\`, or poll a background job in a loop with sleeps INSIDE this turn. Scope your test runs:
name the targets your change touches plus the workspace suites, and say which sweeps you did not
run. \`apps/desktop/src-tauri\` is a SEPARATE cargo workspace — test it explicitly or not at all.
**Measure per-unit cost and project wall time BEFORE any population-scoped run.** If something
genuinely will not finish, report what you observed and **commit the work anyway**. Commit and
push before ending the turn, always.

## Facts you need (do not re-derive from an older document)

- The bundle branch is \`tranche/14\`, cut at \`${CUT_SHA}\` from \`origin/develop\` \`${BASE_SHA}\`
  (SD-33's closure PR #377 merge). Version is \`0.14.0\`.
- Population figures were measured at \`${BASE_SHA}\`; \`${PKG}/content-unit-inventory.md\` carries
  the re-derive command for every one. **Re-derive at HEAD rather than quoting them**
  (\`decisions.md §12\` L2). 49,438 units across 37 books; \`unclassified\` must be 0.
- The denominator gate's default scope is SD-33's folder. Run it against this package explicitly:
  \`python3 scripts/denominator_gate.py --check '${PKG}/*.md'\` → \`violations=0\`.
  (Widening that default is AT-34-E1-006's second obligation.)
- SD-33's inherited test baseline: **29 of 599** workspace suites carrying **46 of 8,034**
  failures, proven pre-existing at the \`tranche/13\` cut. A failure outside that set is SD-34's.
- \`corpus_literal_sweep\`'s baseline examined-population is **48,699 of 51,473**. If your cycle
  adds or regenerates corpus records, that count must move by exactly the number added
  (\`decisions.md §12\` L8).

## The dual-audit gate — run it, do not silence it (§6 step 2)

\`\`\`bash
BASE_BRANCH=$(git merge-base HEAD origin/develop)
git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <YOUR EPIC'S FILE-TOUCH SET FROM §3> ':!**/__tests__/**' ':!**/*.test.*' \\
  | grep -nE '\\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <SAME PATHS> ':!**/__tests__/**' ':!**/*.test.*' \\
  | grep -nE '\\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\\b' || echo 'OK_NO_TOKENS'
\`\`\`

\`<...>\` is a template literal: substitute your epic's own file-touch set from §3 of the workflow
instruction. The trailing \`\\b\` is deliberately omitted from the first pattern — do not add it.
A single-token violation is self-healable inline. A real stub in shipping code is NOT: stop and
file under \`## Open blockers\`.

## What finishing means

1. TDD: RED → confirm it fails **for the intended reason** → GREEN.
2. Verify at the widest build scope (\`decisions.md §10\`), **after the last commit in this cycle
   that can move a figure an assertion depends on** (L7). \`cargo test --locked --no-run\` must
   exit 0.
3. Write your receipt to \`${PKG}/artifacts/${c.dir}/${c.id}_cycle_receipt.md\` using §7's schema
   exactly — including the row-count row, the build-scope row with its SHA, the sweep-population
   row, the figures row (every number with its command AND denominator), and the four-buckets
   movement row. \`- **Status:** …\` is a bullet, not a heading.
4. **Count your own artifact and set your status from that count** (\`decisions.md §4\`). Put the
   literal command output in your return value. Mark the \`kanban.md\` row \`complete\` only if the
   count says so. A self-assessment is not a status.
5. Update \`${PKG}/progress.md\` (prepend your cycle entry) and \`${PKG}/kanban.md\` in the same
   commit, via §5's retry protocol.
6. Emit retro events at the moment they happen: \`python3 scripts/retro.py …\` with \`RETRO_ACTOR\`
   set; \`--verified-by\` is required on a \`correction\`.
7. Commit and push.

## Blockers

A \`## Open blockers\` entry is a request for an operator ruling and it **pauses the bundle**. It is
never a disposition and never a closure path. Two dispositions only: **clear it** (decompose and
run the work — a large blocker is a sequencing problem, not an exemption, and a fix that lives in
another subsystem is still a fix), or **raise your hand** by returning
\`status: "blocked-escalated"\` with the blocker written into \`progress.md\`. Do not defer, do not
hand scope to a successor, do not narrow a gate to pass.

Return the structured object the schema requires. Your final text is a return value, not a message
to a human.`
}

function finalAcceptanceScanPrompt() {
  return `You are the final-acceptance scan for bundle SD-34 (${'AT-34-E6-001'}) in the codex repo at
/home/ubuntu/workspace/repos/codex, branch \`tranche/14\`. You are adversarial: your job is to find
what is short, not to bless what looks finished.

READ FIRST: \`${PKG}/acceptance-and-verification.md\` §3 (the scan's thirteen obligations) and §3a
(the deliverable-integrity checks), \`${PKG}/epic-breakdown.md\` \`### AT-34-E6-001\`, and
\`docs/governance/blocker-closure-doctrine.md\`.

Environment: \`export RETRO_ACTOR=sd34-e6-scan CARGO_TARGET_DIR=/tmp/cargo-sd34-e6-scan CARGO_INCREMENTAL=0\`
and claim the target dir. Shared-checkout rules apply: \`git status --porcelain\` before every git
write, never \`git add -A\`, never \`git stash\`, never force-push.

The scan checks **work**, never reports:
- Count rows and derive SETS, not sizes — subtract id-sets; a count can match while membership does not.
- Re-run every headline command yourself. A subagent recap quotes stale figures.
- Read commit diffs; distinguish a real fix from an edited expectation.
- Verify any method change re-ran everything it already judged, coverage stated as rows-re-run of
  rows-in-affected-set, both with denominators.
- Re-derive failure attribution from \`git\` against the cut SHA \`${CUT_SHA}\`. A lane's claim that a
  failure is pre-existing is a claim, not evidence.
- Grep the closure instruments for hardcoded exclusion lists. Carve-outs hide in code, not prose.
- Verify at the widest build scope, counting targets EXECUTED — not just the exit code.
- Read \`## Open blockers\` in \`${PKG}/progress.md\` — real heading, bounded at the next \`## \`,
  \`<details>\` archives ignored. Any active entry BLOCKS.
- Enumerate open deferrals; none may defer DoD scope, all carry a revisit condition.
- Re-prove the gates still fail: plant a genuine violation, confirm the catch, remove the probe,
  confirm the baseline returns to zero. Leave no residue.
- Every corpus change moved the sweep's examined-population by exactly the records added (L8).
- Every build-scope row names the SHA it ran at, and no later commit in that cycle moved a figure
  its assertions depend on (L7).
- Read \`${PKG}/forward-scope-register.md §E1\` before any sweep reasoning.

Every criterion AT-34-E1-001 … AT-34-E5-004 must be \`complete\`, and **every one of the 26
\`kanban.md\` cards** must be \`complete\`, each re-derived from the repo — never from a dispatch
script's return value (L3). **There is no "complete OR filed under \`## Open blockers\`".** A card at
\`in-progress\`, \`blocked-escalated\`, or \`complete\`-with-a-deferred-half blocks closure.

**If anything is short: return \`gate: "FAIL"\` with \`short\` naming what is short and the command
that shows it.** No retrospective, no sweep, no PR. That is a correct outcome — SD-33's scan halted
ten times and every halt was right. **Do not manufacture a shortfall either:** if the work is
genuinely done, PASS it.

You get exactly one turn — run slow work in the foreground, commit your receipt to
\`${PKG}/artifacts/epic-6-closure/AT-34-E6-001_cycle_receipt.md\` and push before ending it.`
}

function closurePrompt(step, body) {
  return `You are a dispatched closure lane for bundle SD-34 in the codex repo at
/home/ubuntu/workspace/repos/codex, branch \`tranche/14\`. Step: **${step}**.

READ FIRST: \`${PKG}/workflow-instruction.md\` §11 (the closure epilogue, order is load-bearing),
\`${PKG}/acceptance-and-verification.md\` §4, and \`docs/release/template/template.md §6\`.

Environment: \`export RETRO_ACTOR=sd34-e6-closure\`, private \`CARGO_TARGET_DIR\`, \`CARGO_INCREMENTAL=0\`.
Shared-checkout rules apply: \`git status --porcelain\` before every git write, never \`git add -A\`,
never \`git stash\`, never force-push. You get exactly one turn: commit and push before ending it.

${body}

Return the structured object the schema requires.`
}

// -------------------------------------------------- AT-34-E1-008 remediation
// `wiring-class-mismatch` = 7,015 of 10,196 defects across 34 of 37 books
// (verified from the orchestrating session 2026-08-27; decisions.md §13).
// Decomposed into four groups balanced by defect count. They run SEQUENTIALLY
// sharing ONE CARGO_TARGET_DIR: four parallel worktrees would each carry a ~37G
// build cache and the box has ~320G free. Corpus subtrees are disjoint per book,
// so a later parallelisation is possible — it is a disk decision, not a
// correctness one.

const REMEDIATION_GROUPS = [
  { id: 'G1', defects: 1673, books: ['advanced_players_guide', 'core_rulebook'] },
  { id: 'G2', defects: 1694, books: ['beastiary', 'ultimate_psionics', 'ultimate_campaign'] },
  { id: 'G3', defects: 1794, books: ['ultimate_magic', 'bestiary_3', 'horror_adventures', 'bestiary_4', 'inner_sea_gods'] },
  { id: 'G4', defects: 1854, books: [
    'bestiary_2', 'ultimate_wilderness', 'advanced_race_guide', 'mythic_adventures',
    'advanced_class_guide', 'adventurers_guide', 'inner_sea_magic', 'pathfinder_unchained',
    'inner_sea_races', 'inner_sea_faiths', 'bestiary_5', 'ultimate_combat',
    'inner_sea_bestiary', 'book_of_the_damned_volume_2', 'ultimate_equipment',
    'book_of_the_damned_volume_1', 'inner_sea_world_guide', 'ultimate_intrigue',
    'bonus_bestiary', 'occult_adventures', 'inner_sea_intrigue', 'monster_codex',
    'bestiary_6', 'inner_sea_combat',
  ] },
]

function remediationPrompt(g) {
  return `You are a dispatched data-quality lane for bundle SD-34, criterion **AT-34-E1-008**,
group **${g.id}**, in the codex repo at /home/ubuntu/workspace/repos/codex on branch \`tranche/14\`.
Absolute paths only. You start with ZERO context of this bundle.

## The problem you are fixing

This repo keeps a generated cache of game-rules records under \`data/corpus/\`, built from source
files by a canonical generator. Each cached record stores a \`wiring_class\` field describing what
that record does. A self-check compares the stored \`wiring_class\` against one recomputed fresh
from the record's own source tokens, and reports a \`wiring-class-mismatch\` when they disagree —
i.e. the cache is **stale** relative to the current classifier.

\`scripts/verify.sh --only corpus-trap-audit\` reports **7,015 stale records of 10,196 total
findings**, across **34 of 37** books. Commit \`b32926f2af\` (2026-08-14) previously brought this
same count to \`0\` by re-running the generator; the check was not wired into CI afterwards, so the
staleness returned unnoticed. Wiring it is done (AT-34-E1-007). **Your job is to refresh the stale
cache entries so the stored classification matches the source again.**

Read \`${PKG}/decisions.md\` §13 for the ruling and \`${PKG}/epic-breakdown.md\`
\`### AT-34-E1-008\` for your acceptance bar.

**Your group is ${g.id}: ${g.books.length} book(s), ~${g.defects} stale records of 7,015.**

\`\`\`
${g.books.join('\n')}
\`\`\`

Regenerate **only** those books' subtrees under \`data/corpus/\`. Other lanes own the other books.

## Read first

1. \`/home/ubuntu/workspace/repos/codex/CLAUDE.md\`, then \`/home/ubuntu/workspace/repos/codex/AGENTS.md\`
2. \`${PKG}/workflow-instruction.md\` §2.1, §2.2, §2.5, §5, **§6**, §7, §8
3. \`${PKG}/decisions.md\` §13, and §12 L7 and L8 — they bind your build step and your sweep step
4. \`${PKG}/technical-requirements.md\` N5 and \`${PKG}/risks-and-open-questions.md\` §6
5. \`git show b32926f2af\` — the working precedent for exactly this refresh. Follow its mechanism.

## Environment

\`\`\`bash
export RETRO_ACTOR="sd34-e1-008-${g.id.toLowerCase()}"
export CARGO_TARGET_DIR="/tmp/cargo-sd34-remediation"
export CARGO_INCREMENTAL=0
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
\`\`\`
That build cache is shared by the four groups running in sequence. It is warm — reuse it, leave it
in place, and re-claim it as shown so a sibling's cleanup sweep skips it.

## Correctness requirements

**Regenerate through the canonical generator only** (\`gen_book_cache\`, as \`b32926f2af\` did).
Do not hand-edit files under \`data/corpus/\` — a hand-edit produces a record the generator cannot
reproduce, which is the defect class this criterion exists to remove.

**Every record must keep its provenance fields.** Each cached record carries the attribution and
source-tracking fields the project requires — its publisher licence reference, its
Product-Identity term markers (the publisher's trademarked names, which the project tracks so they
are handled correctly downstream), and its \`raw_tokens\` source text. A correct regeneration
carries all of these through unchanged. **Verify this per record, before and after**, and treat any
record that comes back missing one as a generator bug: stop, report it, and do not commit that
book. Run the generator in its default mode, which refuses when provenance would not survive —
do not pass an override that downgrades that refusal to a warning.

**A record-count change compiles clean but leaves other files' hard-coded assertions red.** Grep
the old **and** new counts across \`tests/\`, \`src/\`, \`apps/\`, \`scripts/\` before committing.

**Use recursive search.** A shallow glob such as \`data/corpus/<book>/equipment/*.json\` returns
zero where thousands of files live one directory deeper. State the search you used.

## Procedure

1. Baseline your own group: run the audit, count \`wiring-class-mismatch\` for YOUR books only, and
   state it with its denominator. Record \`corpus_literal_sweep\`'s examined-population too.
2. **Measure before the full run** (§2.5): regenerate ONE book, time it, and project the wall time
   for your whole group before starting the rest. Put the projection in your receipt. A method
   proven at n=1 is not proven at n=24.
3. Regenerate each book. After each: verify the provenance fields per record as above, then
   re-audit that book and confirm \`wiring-class-mismatch = 0\` for it.
4. \`cargo run --locked --bin corpus_literal_sweep\` — 0 findings, and its examined-population must
   move by exactly your record delta (\`decisions.md\` §12 L8). A count that did not move means the
   sweep never looked at your refreshed records, so its pass proves nothing.
5. Widest build scope (\`decisions.md\` §10), run **after your last commit that can move a figure an
   assertion depends on** (L7): \`cargo test --locked --no-run\` exits 0. Name the SHA it ran at.
6. Write \`${PKG}/artifacts/epic-1-atlas/AT-34-E1-008_${g.id}_cycle_receipt.md\` using §7's schema,
   and append your group's rows to
   \`${PKG}/artifacts/epic-1-atlas/wiring-class-remediation.json\` — per book: stale records before,
   after, records regenerated, and the provenance-check result. **Re-read that JSON immediately
   before writing it**; the other groups append to it too.
7. Set your status from the **row count on your own artifact**, never from effort
   (\`decisions.md\` §4). Put the literal count command output in your return value.

## Shared-checkout rules

This checkout is shared with other lanes, so a careless write can discard their work:
- \`git status --porcelain\` before every git write.
- Stage your own paths explicitly. Do not stage the whole tree.
- Do not use \`git stash\` here — the bare form stashes the entire checkout even when run from a
  subdirectory, including other lanes' uncommitted work.
- Do not force-push.
- Leave \`docs/release/SD-33-computed-value-verification/\` and
  \`docs/retro/events/sd31-transcribe.jsonl\` alone; they belong to other lanes.
- Push via §5: \`git fetch origin tranche/14 && git rebase origin/tranche/14 && git push origin HEAD:tranche/14\`,
  retrying up to 5 times on a non-fast-forward.

## One turn only

Nothing wakes you. Run long regenerations in the FOREGROUND with a generous \`timeout\`, or poll a
background job in a loop with sleeps INSIDE this turn. If your group genuinely will not finish,
**commit the books you did finish**, and report exactly which books are done and which are not,
with their remaining counts. Return \`status: "complete"\` only if your artifact's row count covers
your whole group; otherwise report honestly what is short. A cycle that lands partial work and says
so is a success; a cycle that lands nothing while waiting is a total loss.

Return the structured object the schema requires.`
}

async function runRemediation() {
  const title = 'Epic 1 — Completion Atlas'
  phase(title)
  log(`AT-34-E1-008 — wiring-class-mismatch 7,015 of 10,196 defects across 34 of 37 books; 4 groups, sequential`)
  const rows = []
  for (const g of REMEDIATION_GROUPS) {
    const r = await agent(remediationPrompt(g), {
      model: 'sonnet', phase: title, label: `AT-34-E1-008 ${g.id}`, schema: CYCLE_SCHEMA,
    })
    rows.push({ group: g.id, books: g.books.length, defects: g.defects, result: r })
    log(`${g.id} (${g.books.length} books, ~${g.defects} defects) -> ${r ? r.status : 'null'}`)
  }
  return rows
}

// ------------------------------- AT-34-E3-001 bucket-B mechanisms (decisions.md 14)
// Nine mechanisms, 1,006 of 1,006 remaining core_rulebook bucket-B units,
// re-derived from docs/work-inventory.json at HEAD. Cheapest-first so a long
// criterion banks measured rates as it goes.

const BT = String.fromCharCode(96)   // backtick, for inline code spans in prompt text

// ---------------- wave shape, revised 2026-08-28 (operator: do 1 and 2) ----------------
// 1. Lanes NO LONGER regenerate docs/work-inventory.json. A regeneration is three
//    sequential full-corpus passes (corpus_literal_sweep -> derived_evaluator_fixture_check
//    -> v06_work_inventory, the last refusing to run without the first two's reports), which
//    is why a 1-unit cycle cost about the same as a 400-unit one. ONE regeneration now runs
//    at wave end for every lane at once.
// 2. Lanes therefore no longer conflict on that file and run in PARALLEL, each in its own
//    worktree with its own CARGO_TARGET_DIR. 24 CPUs, 440G free -> 4 lanes.

const LANE_RULES = [
  'YOU DO NOT REGENERATE docs/work-inventory.json. Do not run v06_work_inventory, corpus_literal_sweep,',
  'or derived_evaluator_fixture_check, and do not commit docs/work-inventory.json. A single regeneration',
  'cycle runs after this wave and measures what every lane actually moved. If you regenerate, you will',
  'conflict with three sibling lanes and waste the wave.',
  '',
  'VERIFY YOUR CHANGE WITH TESTS INSTEAD. Write unit/integration tests that assert your engine change',
  'produces the explanation or verdict you intend, for named records. State in your report which units',
  'you EXPECT to move and why; the regeneration cycle will confirm or refute it. An expectation that',
  'turns out wrong is a useful finding, not a failure - say what you expected.',
  '',
  'You are in your OWN git worktree with three sibling lanes running concurrently. Touch only the files',
  'your mechanism needs. Before every commit run git status --porcelain and stage your own paths',
  'explicitly - never git add -A, never git stash (its bare form takes the whole checkout).',
  'Push via: git fetch origin tranche/14 && git rebase origin/tranche/14 && git push origin HEAD:tranche/14,',
  'retrying up to 5 times on non-fast-forward. If a sibling landed first, rebase and re-run your tests.',
  '',
  'BEFORE COMMITTING, RUN git diff --cached --numstat AND READ IT. A commit whose subject says "add X"',
  'but whose body deletes shipping code is this repo`s recorded failure mode - it has shipped a revert',
  'disguised as a docs change before. If you see deletions you did not intend, stop and re-stage.',
].join('\n')

const LANES = [
  { id: 'gate-widening', label: 'section-18 anti-fabrication gate widening',
    brief: 'Implement operator ruling ' + BT + 'decisions.md 18' + BT + ': widen the anti-fabrication gates BY CONSTRUCTION. '
      + 'Read that section in full first, then ' + BT + 'src/rules_core/pilot_compute/class_feature_grant_consumer.rs' + BT + '`s '
      + 'module doc (the section explaining the exclusions) and its ' + BT + 'ANTI_FABRICATION_GATE_EXCLUDED_CLASSES' + BT + ' constant. '
      + '**161 of the 242 remaining owner_matched units are gated by that seven-class list** (Sorcerer, Cleric, Monk, Wizard, '
      + 'Paladin, Bard, Druid) - re-derive that split yourself. '
      + 'The ruling: a gate accepts an explanation WHEN IT CITES A REAL CORPUS RECORD, not when its id is on a hand-maintained '
      + 'allowlist. The allowlist becomes a property. '
      + 'THE BAR IS HIGH: OPEN-ISSUES row 338 records a prior attempt REJECTED AS GAMED for falsely claiming these gates needed '
      + 'no widening. You may NOT weaken, delete, ignore, or narrow any of the nine acceptance tests. For EACH gate you change, '
      + 'produce a RED->GREEN mutation proof: plant an explanation citing NO corpus record, confirm the gate catches it, remove '
      + 'the probe, confirm the baseline returns clean. A gate never observed to fail is not a gate. '
      + 'Run the FULL test suite against your draft, not a scoped subset - this exclusion list has already grown twice from '
      + 'gates nobody knew existed (Cleric and Sorcerer were found live, not from OPEN-ISSUES). '
      + 'Druid and Monk sit behind a SEPARATE closed id-prefix filter (is_druid_pillar_id / is_monk_pillar_id in '
      + 'src/rules_core/level_up/) - the citation property alone will not clear them. If you do not also fix that filter, '
      + 'say plainly that Druid and Monk remain and how many units that is.' },
  { id: 'owner-matched', label: 'owner_matched, the 81 NOT gated by the seven-class list',
    brief: 'Mechanism ' + BT + 'class_feature_owner_matched_by_name_but_record_not_held_by_engine' + BT + ', 242 units in core_rulebook. '
      + '**A sibling lane owns the 161 blocked by ANTI_FABRICATION_GATE_EXCLUDED_CLASSES - do not touch those.** '
      + 'You own the ~81 that are NOT gated by that list. Re-derive that split yourself first; do not trust this number. '
      + 'Seven prior cycles ran this mechanism - READ their receipts in artifacts/epic-3-core-rulebook/ and continue from them. '
      + 'Cycle 7 re-derived the inherited four-way sub-cause split and found it did NOT match (105/143/0 against the inherited '
      + '118/15/67/48), so re-derive rather than inherit, and treat inherited REASONS as suspect too: cycle 7 also disproved '
      + 'cycle 4`s stated reason for deferring three prestige classes. '
      + 'The zero-description internal-bookkeeping units are the OPEN definitional question in atlas-defects.md - leave them '
      + 'in bucket B and do not reclassify them into X or U on your own authority.' },
  { id: 'with-magnitude', label: 'option_pool_record_with_magnitude',
    brief: 'Mechanism ' + BT + 'class_feature_option_pool_record_with_magnitude_not_held_by_engine' + BT + ', 258 units in core_rulebook, '
      + '3,052 across 21 of 37 books corpus-wide. Moved 333 -> 258 across six cycles; READ the prior receipts and continue from '
      + 'their named remainder rather than re-deriving from scratch. BUILD GENERICALLY - the payoff is measured, not theoretical: '
      + 'the bucket-U cycle wrote one generic predicate and moved 110 units corpus-wide where a book-scoped one would have moved 40.' },
  { id: 'option-pool', label: 'option_pool_record_not_held_by_engine',
    brief: 'Mechanism ' + BT + 'class_feature_option_pool_record_not_held_by_engine' + BT + ', 34 units in core_rulebook. '
      + 'EIGHT cycles have run this: 63 -> 57 -> 55 -> 52 -> 52 -> 49 -> 44 -> 34 -> 34, and the last closed ZERO. '
      + 'Its named remainder is proficiency/grant possession-tracking and wizard opposition-school tracking - genuinely new '
      + 'engine subsystems, not narrow fixes. BUILD ONE OF THEM PROPERLY this cycle. If you cannot, return partial and state '
      + 'plainly that no narrow work remains, naming exactly what must be built and its population. Do not run a tenth cycle '
      + 'that closes zero and repeats the same remainder.' },
]

function lanePrompt(lane) {
  return cycleProcedurePrompt({
    id: 'AT-34-E3-001',
    dir: 'epic-3-core-rulebook',
    title: lane.label + '\n\n' + lane.brief + '\n\n## WAVE RULES - this wave runs four lanes in parallel\n\n' + LANE_RULES,
  })
}

const REGEN_SCHEMA = {
  type: 'object',
  required: ['status', 'before', 'after', 'attribution'],
  properties: {
    status: { type: 'string', enum: ['complete', 'partial', 'blocked-escalated'] },
    before: { type: 'string' }, after: { type: 'string' },
    attribution: { type: 'string', description: 'which lane moved which units, derived from the diff' },
    commit_sha: { type: 'string' }, unexpected: { type: 'string' },
  },
}

function regenPrompt(laneSummaries) {
  return 'You are the SINGLE closing cycle of a GATE-REMEDIATION wave for bundle SD-34, in '
    + '/home/ubuntu/workspace/repos/codex on branch ' + BT + 'tranche/14' + BT + '. Three lanes just fixed failing '
    + 'verify.sh stages. **Your job is the full sweep, NOT a regeneration** — do not run the inventory '
    + 'regenerator or the dashboard producer (the review names both as silent stamp-droppers).\n\n'
    + '1. ' + BT + 'git fetch origin tranche/14 && git rebase origin/tranche/14' + BT + '.\n'
    + '2. Run ' + BT + 'bash scripts/verify.sh' + BT + ' (full, not quick). It is long — foreground it.\n'
    + '3. Report the stage table: which PASS, which FAIL, and the count of each.\n'
    + '4. **The bar is: same-or-fewer red stages than the 14 the review recorded, and ZERO stages that '
    + 'were green going red.** A stage that flipped green->red is a regression this wave caused and must be '
    + 'named, not averaged away.\n'
    + '5. Baseline note the review flagged: ' + BT + 'BASELINE_CORPUS_LITERAL_RECORDS' + BT + ' 26500 -> 48708 needs a '
    + 'DELIBERATE update. Update it only if you can state why the new number is right.\n\n'
    + 'Write the receipt with the before/after stage counts. Do not claim the gate is green unless every '
    + 'stage exits 0.\n\nIGNORE the regeneration instructions that follow if they conflict with the above.\n\n'
    + 'Historical context for this wave (three lanes just reported):\n\n'
    + '## What the lanes reported\n\n' + laneSummaries + '\n\nLEGACY REGEN PROCEDURE (reference only):\n'
    + 'You are the SINGLE regeneration-and-attribution cycle that closes this wave of bundle SD-34, in '
    + '/home/ubuntu/workspace/repos/codex on branch ' + BT + 'tranche/14' + BT + '. Four lanes just landed engine changes and '
    + 'DELIBERATELY did not regenerate ' + BT + 'docs/work-inventory.json' + BT + '. You do it once, for all of them.\n\n'
    + '## What the lanes reported\n\n' + laneSummaries + '\n\n'
    + '## A red test is waiting on your regeneration\n\n'
    + 'Commit ' + BT + '38e10d066b' + BT + ' prefixed two deferral evidence strings with '
    + BT + 'engine_diagnostic:' + BT + ' (15 of 170 deferrals lacked it, from two classifier sites). The code is landed '
    + 'and its unit tests pass, but ' + BT + 'docs/work-inventory.json' + BT + ' still carries the old strings, so '
    + BT + 'tests/v06_work_inventory.rs::the_committed_inventory_is_well_formed_and_uses_only_declared_statuses' + BT + '\n'
    + 'is RED until your regeneration bakes them in. **Run that test after regenerating and report its result.** '
    + 'Do NOT hand-edit the inventory to make it pass — the regeneration is the only legitimate route.\n\n'
    + '## Record this wave in the wave ledger\n\n'
    + 'Run ' + BT + 'python3 scripts/wave_ledger.py' + BT + ' and paste its table into your receipt, then state in one '
    + 'line how long THIS wave ran and how that compares to the last three. The ledger reads wave timings off the '
    + 'Workflow transcript directories on disk, so it covers waves that were killed before they could report — it '
    + 'already shows that waves 11 and 13 both died at a host reset (a `KILLED?` row means the wave`s last activity '
    + 'lands within 3 minutes of a boot ending). If this wave`s own row is missing a number in the WAVE column, add '
    + 'its run id to ' + BT + 'KNOWN_WAVES' + BT + ' in that script with a one-line note, and commit that too — an '
    + 'unlabelled run is the ledger`s only real failure mode.\n\n'
    + '## Your job\n\n'
    + '1. ' + BT + 'git fetch origin tranche/14 && git rebase origin/tranche/14' + BT + ' so you have every lane`s work.\n'
    + '2. Snapshot the baseline: ' + BT + 'git show HEAD:docs/work-inventory.json > /tmp/wi-wave-before.json' + BT + '.\n'
    + '3. Run the three-pass pipeline IN ORDER - the third refuses to run without the first two`s reports, and that guard '
    + 'exists because a bare run would silently drop 9,516 verification stamps:\n'
    + '   - ' + BT + 'cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep.json' + BT + '\n'
    + '   - ' + BT + 'cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture.json' + BT + '\n'
    + '   - ' + BT + 'CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture.json cargo run --locked --bin v06_work_inventory' + BT + '\n'
    + '   NEVER pass --allow-stamp-loss.\n'
    + '4. Whole-corpus before/after diff BY UNIT ID. Report: total changed, and per lane, which units moved and to which '
    + 'bucket. Compare against what each lane EXPECTED. **A lane whose expectation did not match is the most valuable finding '
    + 'in this wave - report it plainly, do not smooth it over.**\n'
    + '5. Report movement in four buckets: closure (reached DONE) / reclassification (moved between non-DONE buckets) / '
    + 'reachability / instrument-correction. A B->X move is RECLASSIFICATION, never closure.\n'
    + '6. Run ' + BT + 'python3 scripts/completion_atlas.py --book core_rulebook --check' + BT + ' and ' + BT + '--check' + BT + ' corpus-wide; '
    + 'if citation_failures is non-zero the lanes` line insertions shifted completion_atlas.py`s hardcoded file:line citations - '
    + 're-derive each with grep and fix them in this same cycle.\n'
    + '7. Verify at the widest build scope AFTER your regeneration commit: ' + BT + 'cargo test --locked --no-run' + BT + ' exits 0.\n'
    + '8. Update ' + BT + 'kanban.md' + BT + ' and ' + BT + 'progress.md' + BT + ' with the REAL numbers, write a wave receipt to '
    + BT + 'docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_wave9_regen_receipt.md' + BT + ', and '
    + 'commit + push.\n\n'
    + '**Also record the wave`s cost**: wall time of the three passes, so the retro has a measured number for what one '
    + 'regeneration costs. That is the figure this whole wave shape was changed to reduce.\n\n'
    + 'Shared-checkout rules: git status --porcelain before every git write, stage your own paths explicitly, never git add -A, '
    + 'never git stash. ONE turn - foreground the long runs, commit and push before ending it.'
}

// 2026-08-30: the box this runs on is a KVM guest its host hard-kills roughly every 3 hours
// (boots 04:00:00->07:02:01, 07:27:43->10:38:11; journal ends mid-line, no shutdown, 5.5 GB used
// of 167 GB and load 4 on 24 CPUs). That last reading was WRONG -- it sampled `free` after a
// reboot, not before one. sysstat for the same day shows free memory falling 64.0G -> 57.2G ->
// 18.4G -> 1.7G between 20:00 and 21:00 with page cache flat at ~1G (anonymous memory, not
// reclaimable cache), and the box stopped at 22:12. The Proxmox host stops the VM when the guest
// exhausts its RAM, and the thing exhausting it is this script's own build fan-out. `jobs = 6` in
// .cargo/config.toml is the fix; the checkpoint rule below is the seatbelt, not the cure.
// Wave 13's three lanes each died ~38 minutes in holding EVERYTHING uncommitted. So the rule is no
// longer "commit early", it is "checkpoint on a clock" -- a reset must cost minutes, never hours.
const COMMIT_RULE = '## CHECKPOINT ON A CLOCK - the machine WILL be killed under you\\n\\n**The Proxmox host STOPS this VM when the guest runs out of memory, with no warning and no shutdown.** Measured: free memory fell 64G -> 1.3G in twenty minutes and the box was stopped. The cause is build fan-out across parallel lanes -- `cargo` defaults to one job per CPU (24 here), and three lanes at once meant ~72 concurrent `rustc` processes. `.cargo/config.toml` now pins `jobs = 6`, so **do not override it with `-j`** and do not set `CARGO_BUILD_JOBS` higher. If you must run two cargo commands, run them one after the other, never at once. Wave 13\'s three lanes each died ~38 minutes in holding every byte of their work uncommitted. Assume you will be killed mid-sentence.\\n\\n**Commit and push AT LEAST every 20 minutes, whatever state you are in.** Not at the end. Not when it is tidy. A checkpoint commit of half-finished work is worth infinitely more than a perfect uncommitted one, because the uncommitted one does not exist after the reset. Say `wip:` in the subject and note what is unverified -- an honest checkpoint is never an overclaim. Then `git push` it: a commit that lives only in a worktree on this box dies with the box.\\n\\n**Commit as soon as it compiles and its scoped tests pass, then verify AFTER.** The pipeline takes ~30 minutes; a commit takes seconds.\\n\\n**Never write a claim into progress.md or kanban.md whose artifact you have not committed in the same cycle.** A salvage lane once dropped 2,000+ claimed units because the ledger behind them was never captured. If you produce a data file your numbers depend on, `git add` it explicitly - an untracked file is not evidence, and a `git diff` patch never captures it.\\n\\nForeground the long passes; nothing wakes you. `git status --porcelain` before every git write; stage your own paths; never `git add -A`; never `git stash`. Run `git diff --cached --numstat` and READ IT before committing.'

// Wave 13 fenced lanes by BUCKET and they still collided: lanes 2 and 3 both edited
// scripts/completion_atlas.py, src/bin/v06_work_inventory.rs, docs/work-inventory.json and
// completion-atlas.json. A bucket is not a file list -- any lane closing units must touch the
// instrument regardless of which bucket it owns. Two of those four are GENERATED, so no lane may
// write them at all; the other two are serialized by running the classifier lanes back to back.
// Wave 15's bucket-C lane moved 6 Monk Unarmed Damage units from C to V and its commit message
// said it "closes 6". It closed nothing: C and V are both non-DONE buckets, so that was a
// reclassification wearing a closure's words. Derived by bucket-diffing the inventory across the
// wave, which is the only way it shows up -- the lane's own report read like progress.
// Wave 19's UC lane opened in a worktree sitting at ea2b3396f2 -- the tranche/14 CUT point, not
// the branch tip -- and had to reset onto origin/tranche/14 before it could do anything. By then
// seven cycles of its own criterion had landed past that base. Telling every lane to fix it first
// costs one command; discovering it costs a lane half a cycle.
const FRESH_BASE_RULE = '\\n\\n## Your worktree may be at a stale base -- fix it before anything else\\n\\nYour worktree can open at the tranche CUT commit rather than the branch tip, which means work you are about to duplicate may already be landed. **First command, before reading anything: `git fetch origin && git log --oneline -1 origin/tranche/14`, and if your HEAD is not that commit, `git rebase origin/tranche/14` (or reset onto it if your worktree is clean).** Then re-derive every population at that tip. A lane lost half a cycle to this.\\n\\nThe same staleness applies to THIS brief. Its figures were measured when the wave was dispatched and cycles may have landed since. Where the repo and this brief disagree, **the repo wins** -- say so in your receipt rather than quietly following either one.'

const NO_RELABEL_RULE = '\\n\\n## A bucket change is not a closure\\n\\nOnly `-> DONE` is closure. Every other bucket move is a RECLASSIFICATION and must be reported as one, under `movement`, separately from anything you closed. Last wave a lane moved 6 units from bucket C to bucket V and wrote that it "closes 6" -- both are non-DONE buckets, so the real closure count was zero and the wave`s headline was wrong until it was re-derived.\\n\\n**Before you report, bucket-diff the inventory yourself** and state each move as `FROM -> TO: n`. If your number for "closed" is not exactly the size of your `-> DONE` set, it is wrong. A reclassification can be honest and useful work -- say so plainly and it counts; dress it as closure and it will be caught and reversed.'

const GENERATED_FILE_BAN = '\\n\\n## Files you must NOT write\\n\\n`docs/work-inventory.json` and `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are GENERATED, and the single regeneration cycle at the end of this wave owns them. Do not hand-edit them and do not commit them -- if your work changes what they should contain, that is the regeneration\'s job, not yours. Running `completion_atlas.py --check` rewrites the atlas timestamp as a side effect: `git restore` it before you commit. Wave 13 lost a lane to exactly this collision.'

// Wave 13's lanes died holding real work. It was committed in place and pushed to
// origin/salvage/wave13-lane{1,2,3} -- unreviewed, unverified checkpoints, but a genuine head
// start. Each lane is pointed at its own predecessor so the wave does not redo 38 lost minutes.
function salvageNote(branch, what) {
  return '\\n\\n## Start from the rescued work, do not redo it\\n\\n**`origin/' + branch + '` holds your predecessor\'s uncommitted work**, rescued after the host killed the box mid-run: ' + what + '. Read it FIRST — `git diff 1ea93e99ce origin/' + branch + '` — and build on whatever is sound.\\n\\n**It is an unreviewed checkpoint, not a result.** Nothing in it was verified, tested, or measured, and its author never got to check it. Treat every line as a claim to confirm, not a fact to inherit: keep what survives your own review, fix what does not, and say in your receipt which parts you kept and which you discarded and why. Do not cite it as evidence and do not carry any figure out of it un-re-derived.';
}


// WAVE 23 RETARGET. The operator-commissioned fable review found tranche/14 does not pass
// its own gate -- 14 of 40 verify.sh stages red, and already red at wave-22's own commit.
// Turning them green IS SD-34's remaining card work (fable-review.md section 7), so these
// three lanes stop mining buckets and fix the gate. Two stages are already green again:
// pi-sweep (104519e553) and reachability-audit + shape-coverage-standing-gate (58b4f837cc),
// the latter being the dominant root cause -- wave-22's restamp introduced `oracle-agree`
// and `oracle-unverifiable` with no doneness rule. Content lanes resume once the gate is green.

function ucLanePrompt() {
  return cycleProcedurePrompt({ id: 'AT-34-E6-001', dir: 'epic-6-closure',
    title: 'GATE LANE A — the four data/corpus mechanisms that are the last of root-full.\n\n'
      + 'The gate has gone 14 red -> 5 -> nearly clear. Wave 24`s lane A diagnosed the last root-full\n'
      + 'failures completely and withheld the fixes only because its territory was PI-only. **You have\n'
      + 'the corpus grant it lacked.** Read ' + BT + 'artifacts/epic-6-closure/AT-34-E6-001_gate-lane-a_wave24_cycle_receipt.md' + BT + '\n'
      + 'FIRST — all four are fully diagnosed, none needs further investigation:\n\n'
      + '1. ' + BT + 'sd27_book_license_record_counts.rs' + BT + ' (2 tests) — 21 books` ' + BT + 'records_processed' + BT + ' and\n'
      + '   19 books` ' + BT + 'records_redacted' + BT + ' are stale in ' + BT + 'data/corpus/**/LICENSE.json' + BT + '. The receipt\n'
      + '   captured every book and value live. This is a **guarded LICENSE-only** regeneration, NOT a full\n'
      + '   corpus regen — a full one has previously destroyed licence metadata and raw_tokens.\n'
      + '2. ' + BT + 'sd27_equipment_modifier_price_matches_corpus_cost_token.rs' + BT + ' (2 tests) —\n'
      + '   ' + BT + 'pathfinder_unchained' + BT + ' has 4 genuinely duplicated corpus keys (Special Ability ~ ABP +0 ~\n'
      + '   {Ammunition,Armor,Shield,Weapon}). That is a GENERATOR defect: find why it emits duplicates\n'
      + '   before writing corpus. The sibling price count (447,1,126)->(447,1,130) is plausibly the same 4,\n'
      + '   unconfirmed — confirm it rather than assuming.\n'
      + '3. ' + BT + 'sd31_class_feature_corpus_key_uniqueness.rs' + BT + ' (1 test) — delete ONE stale leftover:\n'
      + '   ' + BT + 'data/corpus/adventurers_guide/class_feature/enlightened_bloodrager/bloodline_feat-2.json' + BT + ',\n'
      + '   superseded at the same source line by ' + BT + 'bloodline_feat.json' + BT + ' after a08973ae35. Verify the\n'
      + '   supersession before deleting — read both files.\n'
      + '4. ' + BT + 'v06_corpus_trap_report.rs' + BT + ' (4 tests) — 3,181 findings (249+650+2117+165). Wave 24 judged\n'
      + '   these belong to the EXISTING AT-34-E1-007/AT-34-E1-008 trap epic, not here. Route them: record\n'
      + '   the population against that epic and say so. Do NOT silently re-scope 3,181 findings into this\n'
      + '   card, and do NOT weaken the trap tests to make them pass.\n\n'
      + '**Territory:** ' + BT + 'src/' + BT + ', ' + BT + 'tests/' + BT + ', ' + BT + 'data/corpus/**' + BT + '.\n\n' + COMMIT_RULE + GENERATED_FILE_BAN + FRESH_BASE_RULE })
}

function cLanePrompt() {
  return cycleProcedurePrompt({ id: 'AT-34-E6-001', dir: 'epic-6-closure',
    title: 'GATE LANE B — site-dashboard-check, and the producer timeout behind it.\n\n'
      + 'Wave 24 closed 6 of the 7 desktop failures. ' + BT + 'site-dashboard-check' + BT + ' is the stage still\n'
      + 'attributed to your territory. Read ' + BT + '..._gate-lane-b_wave24_cycle_receipt.md' + BT + ' FIRST.\n\n'
      + 'The committed feeds under ' + BT + 'site/dashboard/' + BT + ' are stale against the current inventory. The\n'
      + 'obstacle is real and measured, not hypothetical: the producer`s own\n'
      + BT + 'v06_work_inventory --summary' + BT + ' step times out at its 600s cap when the box is under wave\n'
      + 'load. Unloaded it takes about 2m26s. **That is contention, not a performance bug — do not raise\n'
      + 'the cap to hide it.** You are the only heavy lane running, so you have the quiet box; measure it\n'
      + 'and report the real number.\n\n'
      + '**The standing hazard, which the review states twice:** do NOT run the inventory regenerator or\n'
      + 'the dashboard producer from a lane — both can silently drop stamps. If the feed genuinely can\n'
      + 'only be refreshed by running the producer, say so plainly in your receipt and leave it for the\n'
      + 'closing sweep. An honest deferral is worth more than a feed refreshed by a tool that ate its own\n'
      + 'provenance.\n\n'
      + 'If ' + BT + 'desktop' + BT + ' or ' + BT + 'reach' + BT + ' are still red, the 7th desktop failure is yours too — wave 24\'s\n'
      + 'receipt names it.\n\n'
      + '**Territory:** ' + BT + 'apps/desktop/' + BT + ' and ' + BT + 'site/' + BT + '.\n\n' + COMMIT_RULE + GENERATED_FILE_BAN + FRESH_BASE_RULE })
}

function mLanePrompt() {
  return cycleProcedurePrompt({ id: 'AT-34-E6-001', dir: 'epic-6-closure',
    title: 'GATE LANE C — hold clippy at zero, then re-measure the whole gate honestly.\n\n'
      + '**Clippy is at 0/0 and the ceilings are now 0/0 — no slack at all.** Wave 24 fixed 86 root and\n'
      + '25 desktop warnings and tightened the ceilings to match, which is right, but it means any warning\n'
      + 'a sibling lane introduces is an instant FAIL. I already had to fix two that appeared right after\n'
      + '(199ec991e0): ' + BT + 'probe_reachable_race_traits' + BT + ' and ' + BT + 'probe_equipment_key_universe' + BT + '\n'
      + 'read as dead under ' + BT + '--all-targets' + BT + ' but are called from `#[cfg(test)]` modules — **scoped with\n'
      + BT + '#[cfg(test)]' + BT + ', not deleted, because deleting them breaks three live tests.** Expect more of\n'
      + 'that shape and apply the same test: before deleting a "never used" function, grep for callers\n'
      + 'inside test modules.\n\n'
      + 'You run LAST. Rebase, re-measure clippy for both crates, and fix anything lanes A and B\n'
      + 'introduced. Do not raise the ceilings — a ceiling raised to meet the count is the gate disabled.\n\n'
      + '**Then the real job: re-measure the whole gate and write down what is actually left.** Run\n'
      + BT + 'bash scripts/verify.sh' + BT + ' (full) and produce the stage table: PASS/FAIL for all 40, the count\n'
      + 'of each, and for every remaining FAIL a one-line named cause. The bundle has been carrying a\n'
      + '"14 red" figure from a review that is now several waves stale; nobody has stated the current\n'
      + 'truth in one place. **Do not report a stage as green because a lane said so — you ran it.**\n\n'
      + '**Territory:** clippy anywhere, plus the sweep. Lanes A and B own the corpus and the desktop/site\n'
      + 'trees respectively; report their stages, do not edit their files.\n\n' + COMMIT_RULE + GENERATED_FILE_BAN + FRESH_BASE_RULE })
}


async function runBucketBMechanisms() {
  const title = 'Epic 3 — Core Rulebook to zero'
  phase(title)
  // Wave 14 shape. Wave 13 ran all three in parallel and lanes C and M collided on four files --
  // completion_atlas.py, v06_work_inventory.rs and the two generated JSONs -- because both are
  // classifier work and a BUCKET fence cannot separate lanes that share the instrument. Two of
  // those files are now banned outright (GENERATED_FILE_BAN); the other two are made safe by
  // running C and M back to back instead of together. UC touches neither (it is a capability
  // build in src/rules_core + apps/desktop) and was disjoint from both in wave 13's own diff,
  // so it still runs alongside. Serializing costs wall-clock, which no longer matters: the
  // 20-minute checkpoint rule means a host reset costs minutes regardless of how long a wave is.
  log('wave 25 (GATE): the last corpus mechanisms + a full, honest gate re-measure')

  const [uc, [vled, m]] = await parallel([
    () => agent(ucLanePrompt(), { model: 'sonnet', phase: title, label: 'A: corpus mechanisms', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
    async () => {
      const c = await agent(cLanePrompt(), { model: 'sonnet', phase: title, label: 'B: site-dashboard + producer', schema: CYCLE_SCHEMA, isolation: 'worktree' })
      log('B -> ' + (c && c.status) + '; starting C (docs gates)')
      const mm = await agent(mLanePrompt(), { model: 'sonnet', phase: title, label: 'C: clippy + full gate re-measure', schema: CYCLE_SCHEMA, isolation: 'worktree' })
      return [c, mm]
    },
  ])
  log('UC -> ' + (uc && uc.status) + ' | C -> ' + (vled && vled.status) + ' | M -> ' + (m && m.status))

  const summary = [['A rust-suites', uc], ['B frontend', vled], ['C docs-gates', m]].map(([n, r]) =>
    '- ' + n + ' (' + ((r && r.status) || '?') + '): ' + String((r && (r.discoveries || r.remainder)) || 'no report').slice(0, 400)).join('\n')
  const regen = await agent(regenPrompt(summary), {
    model: 'sonnet', phase: title, label: 'full verify.sh sweep', schema: REGEN_SCHEMA,
  })
  return { uc, vled, m, regen }
}

// args.bucketB runs ONLY the parallel bucket-B lanes + the single wave regeneration.
// This entry point was accidentally deleted on 2026-08-28 during a two-step rewrite; the
// script then fell through to the line below and ran all six epics -- 26 agents, ~8 hours,
// mostly re-verifying already-complete criteria. The smoke test at the time exercised the
// prompt builders but not the ROUTING, which is why it passed. Any future edit to this file
// must assert that {bucketB:true} selects the lanes and does NOT reach the six-epic default.
if (args && args.bucketB) {
  return await runBucketBMechanisms()
}

const requested = (args && args.epics) ? args.epics : [1, 2, 3, 4, 5, 6]
const completed = []

function runEpicExists() { return typeof runEpic === 'function' }

// `runEpic` was removed on 2026-08-28 when this script moved to wave-shaped dispatch, but this
// call site survived as dead code. On 2026-08-30 a restart passed {epics:[3,4]} and the run died
// instantly with `ReferenceError: runEpic is not defined` -- a confusing failure for a caller who
// had no way to know the per-epic mode no longer exists. Fail with an instruction instead.
if (!runEpicExists()) {
  throw new Error(
    'The per-epic entry point (args.epics) no longer exists -- `runEpic` was removed on 2026-08-28 '
    + 'when this script moved to wave-shaped dispatch. Epics 1, 2 and 5 are complete; the open work '
    + 'is epics 3, 4 and 6. Run the lanes with args {bucketB: true}, or add an explicit Epic 6 '
    + 'closure entry point. Requested was: ' + JSON.stringify(requested))
}

for (const n of requested) {
  if (n === 6) continue
  const out = await runEpic(n)
  completed.push(...out.done)
  if (out.halted) {
    return { halted: out.halted, result: out.result, epics_run: requested, completed }
  }
}

if (!requested.includes(6)) {
  return { epics_run: requested, completed, note: 'epics ran to completion; Epic 6 not requested' }
}

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(), {
  model: 'opus',
  phase: 'Epic 6 — Closure epilogue',
  label: 'AT-34-E6-001 scan',
  schema: SCAN_SCHEMA,
})
if (!scan || scan.gate !== 'PASS') {
  log('Final-acceptance scan did not PASS — no retrospective, no sweep, no PR.')
  return { halted: 'AT-34-E6-001', result: scan, completed }
}

const retro = await agent(closurePrompt('AT-34-E6-002 — retrospective and sweep', `
1. Write \`docs/retro/sd34-book-completion-retrospective.md\`, grounded in
   \`python3 scripts/retro.py summary --since 2026-08-27 --json\`. Follow the shape of
   \`docs/retro/sd31-retrospective.md\`: raw event tally, what the data says, what worked, what did
   not, and named changes for the next bundle. Every lesson names its enforcing command.
2. **Cite it from \`${PKG}/references/README.md\` in THIS SAME cycle** — not as a follow-up.
3. Full worktree/branch sweep for the whole bundle, reporting count found vs removed. Never remove
   a \`locked\` worktree or one carrying unmerged commits. Read
   \`${PKG}/forward-scope-register.md §E1\` first — those three branches are deleted on SD-33's
   ruling, not re-diagnosed. Any OTHER branch carrying unmerged records is diagnosed
   schema-against-HEAD before it is folded or removed (L6), and the diagnosis goes in the receipt.

Steps 1–3 happen BEFORE any PR opens.`), {
  model: 'sonnet', phase: 'Epic 6 — Closure epilogue', label: 'AT-34-E6-002', schema: CYCLE_SCHEMA,
})

const arch = await agent(closurePrompt('AT-34-E6-003 part 1 — architecture docs, graphify, PR', `
Per \`docs/release/template/template.md §6\`: refresh \`docs/architecture/\` for every topic SD-34
touched, run graphify, open the \`tranche/14\` → \`develop\` PR, and resolve merge conflicts.
Append a YAML receipt to \`${PKG}/receipts.md\` for EACH sub-step — \`architecture:truth-up\`,
\`graphify:update\`, \`pr:open\`, and \`merge_conflict:*\` if any — **including on an empty diff**,
because the receipt IS the evidence the gate fired. A non-zero graphify exit does not refuse the
pipeline; the failure receipt is the audit trail.

**The operator merges the PR. No dispatched agent merges.**`), {
  model: 'sonnet', phase: 'Epic 6 — Closure epilogue', label: 'AT-34-E6-003a', schema: CYCLE_SCHEMA,
})

const notes = await agent(closurePrompt('AT-34-E6-003 part 2 — release notes and version confirmation', `
Write \`${PKG}/release-notes.md\` per its own "Required content" block: what shipped, led by the
Completion Atlas — remaining steps by bucket, what the Core Rulebook cost to finish, what the 35
remaining books will cost, then the defects found. Not process narrative. Every figure states its
denominator in the same construct; run
\`python3 scripts/denominator_gate.py --check '${PKG}/*.md'\` and confirm \`violations=0\`.
Record the PR number here and in \`${PKG}/receipts.md\`. Confirm \`apps/desktop/package.json\` and
\`apps/desktop/src-tauri/tauri.conf.json\` both read \`0.14.0\`. **The tranche digit is NOT bumped at
closure** — it moves only on a new \`tranche/N\` cut.`), {
  model: 'haiku', phase: 'Epic 6 — Closure epilogue', label: 'AT-34-E6-003b', schema: CYCLE_SCHEMA,
})

return {
  scan,
  closure: { retro, arch, notes },
  completed,
  note: 'closure claimed — verify against the repo before relaying (decisions.md §12 L3)',
}
