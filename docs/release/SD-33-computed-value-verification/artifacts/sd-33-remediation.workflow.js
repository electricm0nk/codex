export const meta = {
  name: 'sd-33-remediation',
  description: 'SD-33 remediation — close Epic 5 over the full 8,330, fix the denominator gate, re-run closure',
  whenToUse: 'After sd-33-dispatch halted at AT-33-E6-001. Clears the four named shortfalls, then re-runs Epic 6.',
  phases: [
    { title: 'Epic 5 rerun — full population' },
    { title: 'Epic 5 rerun — disagreements' },
    { title: 'Instrument fix — denominator gate' },
    { title: 'Epic 6 — Closure epilogue' },
  ],
}

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-33-computed-value-verification'
const BRANCH = 'tranche/13'

const RECEIPT_SCHEMA = `# Cycle <cycle-id> — <epic-name> / <criterion-id>

- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violations>
- **Wired-integration audit result:** OK_NO_TOKENS / <violations>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Figures + their re-derive commands:** <every number, with its command and denominator>
- **Status:** complete | blocked-escalated
- **Movement, four buckets:** closure / reclassification / reachability / instrument-correction
- **Notes:** <judgment calls>
- **Next-cycle plan:** <what the next cycle picks up>`

function standingRules(role) {
  return `You are a dispatched SD-33 REMEDIATION agent. Role: ${role}.

CONTEXT — WHY YOU EXIST. The SD-33 dispatch ran all 6 epics and then HALTED at
AT-33-E6-001, the final-acceptance scan. The scan was correct to halt. You are
closing one of its named shortfalls. This is not a fresh epic; it is the second
attempt at work that was left short.

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md   (the dispatch procedure — binding)
4. ${REPO}/${PKG}/epic-breakdown.md          (your criterion, verbatim)
5. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001_cycle_receipt.md
   (the scan that halted the bundle — read the shortfall that is yours)
Keep context lean beyond that.

ENVIRONMENT (§2.1) — run these first:
  export RETRO_ACTOR="${role}"
  export CARGO_TARGET_DIR="/tmp/cargo-sd33-${role}"
  export CARGO_INCREMENTAL=0
  mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"

BRANCH: all work lands on ${BRANCH}.

ONE TURN ONLY (§2.5). Nothing will wake you. Never end your turn waiting.
Foreground slow work, or poll a background job in a loop inside this turn. If
something will not finish, report what you observed and COMMIT AND PUSH ANYWAY.
Always commit and push before ending the turn.

GIT DISCIPLINE (§5):
- \`git status --porcelain\` before EVERY git write. Inspect it.
- NEVER \`git add -A\`. NEVER \`git stash\`. NEVER force-push.
- Push with: \`git fetch origin ${BRANCH} && git rebase origin/${BRANCH} && git push origin HEAD:${BRANCH}\`
  Retry up to 5 times on non-fast-forward.
- Re-read shared files (progress.md, kanban.md, THE-BOX.md) immediately before editing.
- NEVER hand-edit data/corpus/**. NEVER pass --allow-stamp-loss.

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` as they happen.
\`--verified-by\` is required on a \`correction\`.

FIGURES (decisions.md §2): every number states its denominator in the SAME
construct, and carries its re-derive command. \`scripts/verify.sh --only
denominator-gate\` will be run against your output — write to pass it.

NO STUBS. No stub, inline mock, placeholder, or "Would ..." string in shipping
code. No sampling presented as a population result.

NO SCOPE CUTS. Do not file a deferral that defers Definition-of-Done scope. Per
${REPO}/docs/governance/blocker-closure-doctrine.md, a blocker larger than one
cycle is a SEQUENCING problem, not an exemption. Decompose it and run it.`
}

// ---------------------------------------------------------------------------
// The core lesson from attempt 1, injected into both Epic 5 lanes.
// ---------------------------------------------------------------------------

const THROUGHPUT_BRIEF = `
## WHY ATTEMPT 1 CAME UP SHORT — READ THIS BEFORE PLANNING

Attempt 1 examined **32 of 8,330 units** (11 fixture + 21 literal). It did not
fail on correctness. Its 32 rows are real, its method is sound, and its
\`(ours, oracle, verdict)\` output is genuine. It failed on THROUGHPUT, for one
specific reason:

**It hand-authored one PCGen \`.pcg\` character per unit.** Eleven units meant
eleven hand-written characters and eleven separate BatchExporter invocations.
That method cannot reach 8,330 in any number of turns.

**Your first deliverable is therefore a GENERATOR, not a batch of characters.**
Do not begin by examining units. Begin by building the thing that examines them:

1. **Generate \`.pcg\` inputs programmatically** from corpus records — a Python
   or Rust emitter that reads a unit and writes a valid \`.pcg\`. Attempt 1's
   committed \`.pcg\` files in
   \`${PKG}/artifacts/epic-5-reverification/fixtures/\` are your worked examples
   of the required format. Derive the template from them; do not re-invent it.
2. **Batch the oracle side.** One BatchExporter run should cover many characters,
   or many runs should be driven by a loop you start once and poll. PCGen JVM
   startup is the dominant fixed cost — amortise it. Measure the per-unit cost
   early and state it.
3. **Batch our side.** Attempt 1 compiled a one-off probe crate OUTSIDE the repo
   to call the real compute function. That is fine for 11 and wrong for
   thousands. Build a proper repo-local binary under \`src/bin/\` that takes a
   unit list and emits our values for all of them in one process.
4. **Then run the full population through it.**

**Budget your turn against the measured rate.** Measure the per-unit cost on a
sample of ~50 first, multiply by your population, and say the projected wall
time out loud in your reasoning BEFORE launching the full run. If the projection
exceeds your turn, that is a signal to make the generator faster (bigger
batches, parallel JVMs, a shared export template covering many units per
character) — NOT a signal to sample and defer the rest.

**A character can carry many units at once.** One exported character with a
template emitting N computed variables verifies N units in one JVM start. This
is the single biggest lever available to you. Attempt 1 used one unit per
character. Use as many as the template and the game rules allow, and say what
that number turned out to be.

**Run the whole population.** If some units are genuinely unverifiable by the
oracle, \`unverifiable\` is a FIRST-CLASS verdict (AT-33-E2-003) — record it per
unit with its reason. That is a real result. \`unverifiable\` is NOT a synonym
for "we ran out of time", and a unit you never fed to the harness is neither
\`agree\` nor \`unverifiable\` — it is unexamined, and unexamined is what this
remediation exists to eliminate.

**Clear your open deferral.** Attempt 1's lane filed a deferral deferring the
un-examined remainder. Resolve it via \`scripts/retro.py\` in this cycle. Do not
file a replacement.
`

function e5RerunPrompt(c) {
  return `${standingRules(c.role)}

## YOUR CRITERION: ${c.id} — ${c.card} (kanban row ${c.row})
Population: **${c.population}** ${c.popName} units.
Receipt: ${PKG}/artifacts/epic-5-reverification/${c.id}_cycle_receipt.md
(OVERWRITE attempt 1's receipt in place — same path. Keep its honest
partition-of-what-was-examined discipline; replace its figures with full-population ones.)

Read ${c.id} verbatim from epic-breakdown.md. Its evidence bar: per-unit
\`(ours, oracle, verdict)\` rows committed; agreement AND disagreement counts
both stated, with the denominator.

Attempt 1 left kanban row ${c.row} at \`in-progress\` and its receipt says
\`in-progress\`. The lane did NOT over-claim — it reported honestly. Your job is
to finish what it correctly reported as unfinished.

${THROUGHPUT_BRIEF}

## PRIOR WORK YOU INHERIT — REUSE, DO NOT REBUILD
- \`scripts/oracle_harness/\` (run.py, compare.py, oracle_export.py) — AT-33-E2-003's
  proven CLI. Extend it for batch throughput; do not fork it.
- \`${PKG}/artifacts/epic-5-reverification/README.md\` — attempt 1's methodology.
- Attempt 1's \`.pcg\` files and \`.ftl\` template — your format reference.
- Attempt 1's 32 verified rows — KEEP them, fold them into your full result set
  rather than re-running them, and say you did.

## PER-CYCLE PROCEDURE (§6) — all nine steps

1. Rebase onto ${BRANCH} (§5), then verify the base is real:
     test -d docs && test -d data && test -d scripts \\
       || { echo 'WRONG BASE — reset before continuing'; exit 1; }
2. Define the audit base once, then run both greps:
     BASE_BRANCH=$(git merge-base HEAD origin/develop)
     git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <your scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \\
       | grep -nE '\\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
     git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <your scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \\
       | grep -nE '\\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\\b' || echo 'OK_NO_TOKENS'
   The trailing \\b is deliberately omitted from the first pattern. Do not add it.
3. TDD: RED -> confirm it fails FOR THE INTENDED REASON -> GREEN -> scoped suite.
4. Re-run both audits on the final diff.
5. Write the receipt using EXACTLY this schema — note \`- **Status:** <x>\` is a
   BULLET, not a \`## Status\` heading (attempt 1 drifted on this):

${RECEIPT_SCHEMA}

6. Commit, push via §5.
7. Update progress.md and kanban.md in place via §5 (re-read first).
8. Mark kanban row ${c.row} \`complete\` — and only if it genuinely is.
9. Report.

## WRITE SCOPE (you are worktree-isolated; stay in your lane)
${c.scope}

You share \`scripts/oracle_harness/\` and \`progress.md\`/\`kanban.md\` with the other
Epic 5 lane running RIGHT NOW in parallel. Re-read before every edit, rebase
before every push, and never revert their rows.

## RETURN VALUE — ONLY JSON, no prose:
{"criterion":"${c.id}","status":"complete"|"blocked-escalated",
 "population":${c.population},"examined":0,"agree":0,"disagree":0,"unverifiable":0,
 "unexamined":0,"denominator_statement":"<X of ${c.population} ${c.popName} units>",
 "per_unit_rows_path":"...","generator_path":"...","per_unit_cost_seconds":0,
 "units_per_character":0,
 "disagreements":[{"unit_id":"...","ours":"...","oracle":"..."}],
 "deferral_resolved":true|false,
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

const E5_RERUN = [
  {
    id: 'AT-33-E5-001', card: 'reverify-fixture-verified', row: 16,
    role: 'sd33-r-e5-fixture', population: 1741, popName: 'fixture-verified',
    scope: `- \`${PKG}/artifacts/epic-5-reverification/\` — files prefixed \`fixture-\` or already yours from attempt 1
- \`scripts/oracle_harness/\` — batch extensions (coordinate: the literal lane is also here)
- \`src/bin/\` — your batch "ours" probe binary, named for the fixture population`,
  },
  {
    id: 'AT-33-E5-002', card: 'reverify-literal-verified', row: 17,
    role: 'sd33-r-e5-literal', population: 6589, popName: 'literal-verified',
    scope: `- \`${PKG}/artifacts/epic-5-reverification/\` — files prefixed \`literal-\` or already yours from attempt 1
- \`scripts/oracle_harness/\` — batch extensions (coordinate: the fixture lane is also here)
- \`src/bin/\` — your batch "ours" probe binary, named for the literal population`,
  },
]

function e5DisagreementPrompt(fixtureResult, literalResult) {
  return `${standingRules('sd33-r-e5-disagreements')}

## YOUR CRITERION: AT-33-E5-003 — disagreement resolution (kanban row 18)

Attempt 1 marked this row \`complete\` over **32 of 8,330 units**. The
final-acceptance scan named that a complete-with-a-deferred-half and it BLOCKED
closure. The row's \`complete\` mark is therefore NOT trustworthy — treat this
criterion as open and re-establish it over the population that the two rerun
lanes just examined.

Their results (reports, NOT evidence — read their committed receipts and
per-unit row files for the authoritative numbers):

FIXTURE lane (AT-33-E5-001):
${JSON.stringify(fixtureResult).slice(0, 4000)}

LITERAL lane (AT-33-E5-002):
${JSON.stringify(literalResult).slice(0, 4000)}

## THE BAR (verbatim from epic-breakdown.md)
> A disagreement is **never** closed by adjusting the expectation to match our
> output. Each is root-caused: either our computation is wrong (fix it) or the
> oracle comparison is wrong (fix the harness, and re-run everything it already
> judged).
> **Evidence:** one entry per disagreement in \`progress.md\`, each resolved to a
> commit or an operator escalation. **A filed blocker does not satisfy this
> criterion.**

Note the second limb: if you fix the HARNESS, you must RE-RUN EVERYTHING IT
ALREADY JUDGED — including both lanes' full populations. Budget for that.

If the two lanes produced ZERO disagreements across 8,330 units, that is a
suspicious result, not a happy one. AT-33-E2-003 required the harness to be
PROVEN capable of returning \`disagree\`. Confirm that capability still holds
against the batch path specifically — a batch rewrite can silently swallow
disagreements that the single-unit path caught. Show the batch path returning
\`disagree\` on a known-disagreeing case before you accept a zero.

## PROCEDURE
Follow ${PKG}/workflow-instruction.md §6, all nine steps (the audit greps, the
§7 receipt schema with \`- **Status:** <x>\` as a BULLET, commit, push, kanban).
Receipt path: ${PKG}/artifacts/epic-5-reverification/AT-33-E5-003_cycle_receipt.md
(overwrite attempt 1's in place). Mark kanban row 18 \`complete\` only if every
disagreement is resolved to a commit or escalation over the FULL examined
population, with the denominator stated.

## RETURN VALUE — ONLY JSON:
{"criterion":"AT-33-E5-003","status":"complete"|"blocked-escalated",
 "population_examined":0,"denominator_statement":"<X of 8,330>",
 "disagreements_found":0,"resolved_to_commit":0,"escalated":0,
 "disagreements":[{"unit_id":"...","ours":"...","oracle":"...","rootcause":"...","resolution":"..."}],
 "harness_refixed":true|false,"rerun_after_harness_fix":true|false,
 "disagree_capability_reproven_on_batch_path":true|false,
 "commit_shas":[...],"receipt_path":"...","next":"..."}`
}

function denominatorGatePrompt(priorSummary) {
  return `${standingRules('sd33-r-denominator-gate')}

## YOUR TASK — the denominator gate is RED, and AT-33-E1-004 requires it GREEN

\`scripts/verify.sh --only denominator-gate\` currently FAILS. AT-33-E1-004's
evidence obligation is that this stage runs and passes. It is an SD-33
deliverable failing on SD-33's own output.

The final-acceptance scan reported, at halt time:
    FAIL  denominator-gate  (violations=7 of files_checked=16)

The scan's own reading of those 7 — a JUDGMENT, which you must verify yourself,
not inherit:
- **Six** appear to be the gate matching the warning IDIOM used in prose that
  argues AGAINST the defect. That is a matcher precision bug: a sentence saying
  "a bare percentage is not evidence" should not itself trip the gate.
- **One** (\`AT-33-E5-002_cycle_receipt.md:217\` as of the scan) looks like a real
  agree/disagree percentage pair whose denominator sits in a NEIGHBOURING
  sentence rather than the same construct. That one is a real violation and the
  prose is what should change.

**Re-derive the current violation set yourself.** Two Epic 5 lanes and a
disagreement lane have rewritten those receipts since the scan ran, so the line
numbers and probably the set have MOVED. Prior remediation results:
${JSON.stringify(priorSummary).slice(0, 3000)}

## THE RULE YOU MUST NOT BREAK
Fixing a gate by making it easier to pass is the failure mode this whole bundle
exists to prevent. Loosening the matcher is legitimate ONLY where it is a
demonstrated FALSE POSITIVE — a construct that genuinely states its denominator,
or prose ABOUT the rule rather than a figure subject to it. It is NOT legitimate
as a way to make a real violation disappear.

For every one of the current violations, state in the receipt: false positive
(and why) -> fix the matcher; or real violation -> fix the PROSE. Both fixes are
allowed. Which one you chose, per violation, is the deliverable.

## RED -> GREEN OBLIGATION
Whatever you change in \`scripts/denominator_gate.py\`, its own detection power
must be re-proven after the change:
- a deliberately-malformed receipt (a bare percentage, no denominator) STILL FAILS
- the corrected form passes
Transcribe both into the receipt. A matcher relaxed until nothing trips it is a
gate that has stopped working, and AT-33-E1-002's doctrine applies: a tool that
has never been observed to fail is not a gate.

## FINISH LINE
\`scripts/verify.sh --only denominator-gate\` exits 0, across ALL files it checks,
with its detection power re-proven. Also run \`scripts/verify.sh\` in full and
report whether any OTHER stage is red — do not fix those, just report them.

## PROCEDURE
§6, all nine steps. Receipt:
${PKG}/artifacts/epic-1-instruments/AT-33-E1-004-remediation_cycle_receipt.md
Append a pointer to kanban row 4's Notes (pointer only, never a story); row 4
stays \`complete\`.

## RETURN VALUE — ONLY JSON:
{"task":"denominator-gate-green","status":"complete"|"blocked-escalated",
 "violations_before":0,"violations_after":0,"files_checked":0,
 "dispositions":[{"file":"...","line":0,"verdict":"false-positive"|"real-violation","fix":"matcher"|"prose"}],
 "detection_reproven":true|false,"red_green":"...",
 "verify_sh_full_result":"...","other_red_stages":[...],
 "commit_shas":[...],"receipt_path":"..."}`
}

// --- Epic 6 (re-run) --------------------------------------------------------

function finalAcceptanceScanPrompt(remediationSummary) {
  return `${standingRules('sd33-r-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, SECOND ATTEMPT (kanban row 19)

The first scan FAILED and correctly halted the bundle. A remediation wave has
since run against its four named shortfalls. You are re-running the gate.

The first scan's receipt is at
${PKG}/artifacts/epic-6-closure/AT-33-E6-001_cycle_receipt.md — READ IT. Its four
shortfalls are your first four checks. Remediation lanes reported:
${JSON.stringify(remediationSummary).slice(0, 6000)}

That is a REPORT, not evidence. Verify all of it against the repo yourself.

## THE FOUR SHORTFALLS — verify each is genuinely closed
1. kanban rows 16 and 17 at \`complete\`, with their receipts agreeing, over the
   FULL populations (1,741 and 6,589) — not a sample. Check the per-unit row
   files and COUNT them; do not trust a receipt's summary line.
2. row 18 (AT-33-E5-003) \`complete\` over the full examined population, with the
   denominator stated, every disagreement resolved to a commit or escalation.
3. \`scripts/verify.sh --only denominator-gate\` exits 0 — AND its detection power
   is still real. Re-prove it: feed it a bare percentage with no denominator and
   confirm it FAILS. A gate that was fixed by being blinded is a worse outcome
   than the original red.
4. No open deferral defers Definition-of-Done scope. All open deferrals carry a
   named revisit condition. Enumerate them; do not trust a count.

## THEN THE FULL SCAN (as before)
Every criterion AT-33-E1-001 .. AT-33-E5-003 \`complete\`; every kanban card rows
1-18 \`complete\` (19-21 are Epic 6's own). \`returned-to-backlog\`,
\`in-progress\`, \`blocked-escalated\`, or \`complete\`-with-a-deferred-half BLOCKS.
There is NO "complete OR filed under Open blockers".

Check the WORK, not the reports:
- \`git log\` and the actual target files per criterion
- every receipt exists at its kanban-stated path and matches §7, INCLUDING the
  figures row (number + denominator + re-derive command) and the four-buckets row
- re-run the headline re-derive commands yourself — subagent recaps quote stale
  intermediate figures
- grep the closure INSTRUMENTS for hardcoded exclusion lists; carve-outs hide in
  code, not prose
- \`python3 scripts/box_ledger.py --check\` exits 0
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- Epic 3's corpus-wide artifact is at the SD-33 path and SD-32's
  \`docs/release/SD-32-.../artifacts/gate-2-engines/\` file is UNTOUCHED

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. That is a CORRECT outcome. You are the
scanner, not an executor — do not fix it yourself.

Receipt: ${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt2_cycle_receipt.md
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":2,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 2:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE -> \`docs/retro/sd33-computed-value-verification-retrospective.md\`,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-24 --json\` — READ
   that output, do not paraphrase from memory.
   \`deferrals.open\` IS trustworthy here: SD-32's fix landed
   (\`grep -n 'len(open_deferrals)' scripts/retro.py\` -> line 772). Confirm it
   yourself and SAY SO in the doc.

   **The headline lesson of this bundle is the one that halted it.** SD-33's
   Epic 5 came up short not on correctness but on THROUGHPUT: a method proven at
   n=1 (one hand-authored PCGen character per unit) was carried into a
   population of 8,330 without measuring its per-unit cost first. The scan
   caught it; the remediation wave fixed it by building a generator. Record that
   as a lesson WITH ITS ENFORCING MECHANISM — e.g. a dispatch-brief field
   requiring a measured per-unit cost and a projected wall time before any
   population-scoped run begins. A lesson without a mechanism is a quote
   (decisions.md §4).

   Also record: the first scan's FAIL was the system working. Say so plainly.

   Close out workflow-instruction.md §12 rows 3 and 8, both marked UNENFORCED at
   launch: state whether they were closed and how, or why not.

2. CITE IT from ${PKG}/references/README.md IN THIS SAME CYCLE. An uncited
   retrospective does not satisfy the criterion.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs count REMOVED. NEVER remove a \`locked\` worktree or one
   carrying unmerged commits — report those instead. Note: the first dispatch
   run left worktrees behind (\`.claude/worktrees/wf_13e796f5-f34-*\`); check them
   for unmerged commits before removing.

NO PR IN THIS CYCLE. Steps 2 and 3 land before the PR opens.

Receipts to ${PKG}/artifacts/epic-6-closure/. Commit, push (§5), mark row 20 complete.

Return ONLY JSON:
{"criterion":"AT-33-E6-002","status":"complete"|"blocked-escalated",
 "retro_path":"...","cited_from":"...","retro_summary_figures":[...],
 "deferrals_field_corrected":true|false,
 "throughput_lesson_mechanism":"...",
 "sweep":{"worktrees_found":0,"worktrees_removed":0,"branches_found":0,"branches_removed":0,"kept_locked":[...],"kept_unmerged":[...]},
 "unenforced_rows_closed":{"row3":"...","row8":"..."},
 "receipt_paths":[...],"commit_shas":[...]}`
}

function architectureDocsGraphifyPrPrompt(prior) {
  return `${standingRules('sd33-r-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow ${REPO}/docs/release/template/template.md §6 — read it first, it is the
procedure of record.

1. ARCHITECTURE DOCS — docs/architecture/ is CURRENT-STATE TRUTH. Refresh it for
   what SD-33 actually changed: the PCGen oracle harness and its batch generator,
   \`box_ledger.py\` and THE-BOX partition, the denominator gate in verify.sh,
   formula-interpreter corpus-wide coverage, and the work-inventory
   \`unknown\` -> zero classification.
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. The body cites the retrospective, the
   receipts, and the headline figures WITH THEIR DENOMINATORS. State plainly
   that the first final-acceptance scan failed and what the remediation wave
   closed — that history belongs in the PR, not hidden.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; the release-notes cycle closes it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r-release-notes')}

## YOUR CRITERION: AT-33-E6-003 (part 2) — release notes + version bump (kanban row 21)

Housekeeping. The PR is already open:
${JSON.stringify(prPrior).slice(0, 2000)}

1. Fill in ${PKG}/release-notes.md for build 0.13.0 — what shipped, every figure
   stating its denominator. No narrative ceremony.
2. Record the PR number in release-notes.md and in ${PKG}/receipts.md.
3. VERSION BUMP: confirm \`apps/desktop/package.json\` and
   \`apps/desktop/src-tauri/tauri.conf.json\` BOTH read \`0.13.0\`. They were stamped
   at the ${BRANCH} cut. DO NOT bump the tranche digit — it moves only on a NEW
   tranche/N branch cut, never on a bundle's own closure.
4. Placeholder sweep — every match resolves or is a documented deferral:
   grep -rn '<[a-z_-]*>' ${PKG}/*.md
5. Update ${PKG}/progress.md to the closed state; mark kanban row 21 \`complete\`.
6. Re-run \`scripts/verify.sh --only denominator-gate\` on your own new prose and
   confirm it still exits 0 before you push.

Commit and push (§5).

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part2","status":"complete",
 "release_notes_path":"...","pr_number":0,
 "versions":{"package_json":"...","tauri_conf":"..."},
 "placeholders_remaining":[...],"denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...]}`
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

log('SD-33 REMEDIATION. Attempt 1 halted at AT-33-E6-001 with 4 shortfalls. Closing them, then re-running Epic 6.')

// --- Epic 5 rerun: two lanes, parallel, worktree-isolated -------------------
phase('Epic 5 rerun — full population')
const [fixture, literal] = await parallel([
  () => agent(e5RerunPrompt(E5_RERUN[0]), { model: 'sonnet', label: 'reverify-fixture-1741', phase: 'Epic 5 rerun — full population', isolation: 'worktree' }),
  () => agent(e5RerunPrompt(E5_RERUN[1]), { model: 'sonnet', label: 'reverify-literal-6589', phase: 'Epic 5 rerun — full population', isolation: 'worktree' }),
])
log('Epic 5 rerun lanes returned. Check `df -h /` and `git worktree list` (§8).')

// --- AT-33-E5-003 over the full examined population -------------------------
phase('Epic 5 rerun — disagreements')
const disagreements = await agent(e5DisagreementPrompt(fixture, literal), {
  model: 'sonnet', label: 'disagreement-resolution', phase: 'Epic 5 rerun — disagreements',
})

// --- Instrument fix: the denominator gate must be green ---------------------
// Sequenced AFTER Epic 5 deliberately: those lanes rewrite the very receipts the
// gate scans, so fixing it first would fix a moving target.
phase('Instrument fix — denominator gate')
const gateFix = await agent(denominatorGatePrompt({ fixture, literal, disagreements }), {
  model: 'sonnet', label: 'denominator-gate-green', phase: 'Instrument fix — denominator gate',
})

const remediationSummary = { fixture, literal, disagreements, gateFix }

// --- Epic 6 re-run ----------------------------------------------------------
phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(remediationSummary), {
  model: 'opus', label: 'final-acceptance-scan-attempt2', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 2 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', remediation: true, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 2)',
    scan, remediationSummary,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', remediation: true, branch: BRANCH, closed: true,
  remediationSummary,
  epic6: { scan, retroSweep, archPr, notes },
}
