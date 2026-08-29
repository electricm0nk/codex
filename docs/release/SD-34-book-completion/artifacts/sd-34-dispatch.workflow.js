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
  return 'You are the SINGLE regeneration-and-attribution cycle that closes this wave of bundle SD-34, in '
    + '/home/ubuntu/workspace/repos/codex on branch ' + BT + 'tranche/14' + BT + '. Four lanes just landed engine changes and '
    + 'DELIBERATELY did not regenerate ' + BT + 'docs/work-inventory.json' + BT + '. You do it once, for all of them.\n\n'
    + '## What the lanes reported\n\n' + laneSummaries + '\n\n'
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

function batchEditPrompt() {
  return [
    'You are the SINGLE batch-edit cycle for bundle SD-34, in your own isolated git worktree on ' + BT + 'tranche/14' + BT + '.',
    '',
    '## Why you are one agent and not four',
    '',
    'This workspace has 543 integration test targets. A cold build is ~38GB of compilation. Four parallel',
    'lanes each paid that separately, so a 2-unit change and a 400-unit change cost the same. You make ALL',
    'the changes first, then compile ONCE, then test ONCE, then iterate on what the tests actually say.',
    'Every iteration after the first build is warm and cheap.',
    '',
    '## Environment',
    '',
    '```bash',
    'export RETRO_ACTOR="sd34-batch-b"',
    'export CARGO_TARGET_DIR="/tmp/cargo-sd34-batch"   # ONE dir, reused across your iterations',
    'export CARGO_INCREMENTAL=0',
    'mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"',
    '```',
    '',
    '## The work: bucket B for core_rulebook, 532 units across three mechanisms',
    '',
    'Re-derive each population yourself before starting; do not trust these numbers.',
    '',
    '1. ' + BT + 'class_feature_owner_matched_by_name_but_record_not_held_by_engine' + BT + ' (~242). §18`s gate widening landed,',
    '   so Wizard/Bard/Paladin/Cleric/Sorcerer now reach the citation gate. Monk (25) + Druid (1) stay blocked',
    '   behind ' + BT + 'is_druid_pillar_id' + BT + '/' + BT + 'is_monk_pillar_id' + BT + ' in ' + BT + 'src/rules_core/level_up/' + BT + ' - widening THAT filter',
    '   by the same citation property is in scope for you. ~18 zero-description units are the OPEN definitional',
    '   question in atlas-defects.md: leave them in B, do not reclassify.',
    '2. ' + BT + 'class_feature_option_pool_record_with_magnitude_not_held_by_engine' + BT + ' (~208). Weapon Training was just',
    '   generalized to all 14 groups; continue from that receipt`s named remainder.',
    '3. ' + BT + 'class_feature_option_pool_record_not_held_by_engine' + BT + ' (~34). Nine cycles, last few closed almost nothing.',
    '   Its remainder needs real subsystems: proficiency/grant possession-tracking, wizard opposition-school',
    '   tracking. Build one properly or say plainly it is not narrow work.',
    '',
    '## How to work',
    '',
    '1. **Read the prior receipts first** in ' + BT + 'artifacts/epic-3-core-rulebook/' + BT + '. Roughly 35 cycles have run these',
    '   mechanisms and their named remainders are sound. Continue from them; do not re-derive their investigations.',
    '   But DO re-derive their NUMBERS and their stated REASONS - both have been wrong before, and a cycle',
    '   already disproved another`s reason for deferring three classes.',
    '2. **Make every edit you intend, across all three mechanisms, BEFORE your first build.** Write the tests',
    '   too. Do not build between edits.',
    '3. Then ' + BT + 'cargo test --locked --no-run' + BT + ' once. Fix compile errors, rebuild (now warm).',
    '4. Then run the scoped suites plus the nine ' + BT + 'sd13_*' + BT + '/' + BT + 'sd25_*' + BT + ' anti-fabrication gates. Fix what fails,',
    '   re-run. **Never weaken, ignore, or narrow a gate to make it pass** - a prior attempt here was rejected',
    '   as GAMED for that. A gate accepts an explanation because it CITES A REAL CORPUS RECORD (decisions.md §18).',
    '5. **Do NOT regenerate ' + BT + 'docs/work-inventory.json' + BT + '** and do not run corpus_literal_sweep or',
    '   derived_evaluator_fixture_check. A separate regeneration cycle measures what you moved. State what you',
    '   EXPECT to move, per mechanism, and why. A wrong expectation is a finding worth reporting, not a failure.',
    '',
    '## Rules',
    '',
    'git status --porcelain before every git write. Stage your own paths explicitly - never git add -A, never',
    'git stash. **Before committing run ' + BT + 'git diff --cached --numstat' + BT + ' and read it**: a commit whose subject says',
    '"add" while its body deletes shipping code is this repo`s recorded failure mode and has happened here.',
    'Push via fetch + rebase + push, retrying up to 5 times.',
    '',
    'ONE turn. Foreground the builds - do not end your turn waiting on a background job; lanes have lost work',
    'that way. Commit and push before ending, even for partial work.',
    '',
    'Return the structured object: status, commit_sha, row_count_command_output, receipt_path, figures,',
    'build_scope, movement, remainder, discoveries, next_cycle_plan.',
  ].join('\n')
}

async function runBucketBMechanisms() {
  const title = 'Epic 3 — Core Rulebook to zero'
  phase(title)
  log('batch wave: ONE edit cycle across all three bucket-B mechanisms (one cold build, not four), then ONE regeneration')

  const edit = await agent(batchEditPrompt(), {
    model: 'sonnet', phase: title, label: 'batch-edit: all bucket-B mechanisms',
    schema: CYCLE_SCHEMA, isolation: 'worktree',
  })
  log('batch edit -> ' + (edit ? edit.status : 'null'))
  if (halted(edit)) return { edit, halted: 'batch-edit' }

  const summary = '- batch edit (' + (edit.status || '?') + '): '
    + String(edit.discoveries || edit.remainder || 'no report').slice(0, 600)
  const regen = await agent(regenPrompt(summary), {
    model: 'sonnet', phase: title, label: 'regen + attribution', schema: REGEN_SCHEMA,
  })
  return { edit, regen }
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
